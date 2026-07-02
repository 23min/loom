//! Substrate → backend dispatch and execution — Contract 5 / §4.5 totality.
//!
//! Routing is total *by construction*: [`dispatch`] is an exhaustive match over [`Substrate`],
//! so adding a substrate without a backend is a compile error — nothing is silently unverified.
//! [`run`] executes the routed backend. Two backends exist: Dafny + Z3 (deductive) and TLA+/TLC
//! (explicit-state model checking, the M-0018 second substrate). Each shells out to its tool with
//! the property directory as the working directory (so locations are relative and reproducible,
//! G1) via the shared [`run_under_timeout`] skeleton — the single home of the wall-clock ceiling
//! that isolates each verifier's nondeterminism (a hang becomes an `error`, never a hung runner or
//! a false verdict). The pure output→verdict mappings ([`interpret_dafny_output`],
//! [`interpret_tlc_output`]) are unit-tested with canned output and need no toolchain.

use crate::report::{Gap, Substrate, Verdict};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

/// The per-property lowering file the Dafny backend verifies (Contract 3 — an attached artifact).
pub(crate) const MODEL_FILE: &str = "model.dfy";

/// The per-property TLA+ module the TLC backend checks, and its model config. TLC finds the
/// `.cfg` by the module's base name automatically.
pub(crate) const TLA_MODEL_FILE: &str = "model.tla";
pub(crate) const TLA_CONFIG_FILE: &str = "model.cfg";

/// Wall-clock ceiling on a single verification, applied to every backend. A hang past this is
/// surfaced as `error`, not a hung runner (G1 — the verifier's nondeterminism is isolated and
/// reported, never folded in).
pub(crate) const VERIFY_TIMEOUT: Duration = Duration::from_secs(120);

/// The verification engine responsible for a substrate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Backend {
    /// Dafny + Z3 (deductive verification).
    Dafny,
    /// TLA+ / TLC (explicit-state model checking) — the M-0018 second substrate.
    Tlc,
}

impl Backend {
    /// The backend's short name, for reports and logs.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Backend::Dafny => "dafny",
            Backend::Tlc => "tlc",
        }
    }

    /// The property-dir artifacts this backend reads, recorded in the report's audit trail so a
    /// verdict names the exact inputs it saw (E3). Order-stable (G1).
    pub(crate) fn inputs(self) -> &'static [&'static str] {
        match self {
            Backend::Dafny => &[MODEL_FILE],
            Backend::Tlc => &[TLA_MODEL_FILE, TLA_CONFIG_FILE],
        }
    }
}

/// Route a substrate to exactly one backend. Total: the match is exhaustive, so every substrate
/// maps to one backend and none is silently unverified.
pub(crate) fn dispatch(substrate: Substrate) -> Backend {
    match substrate {
        Substrate::Dafny => Backend::Dafny,
        Substrate::Tla => Backend::Tlc,
    }
}

/// What a backend run yields: a graded verdict, any gaps, and the audit rationale.
#[derive(Debug, Clone)]
pub(crate) struct BackendOutcome {
    pub verdict: Verdict,
    pub gaps: Vec<Gap>,
    pub rationale: String,
}

/// Run the routed backend over the property directory (which holds the backend's artifact).
pub(crate) fn run(backend: Backend, prop_dir: &Path, timeout: Duration) -> BackendOutcome {
    match backend {
        Backend::Dafny => run_dafny(prop_dir, timeout),
        Backend::Tlc => run_tlc(prop_dir, timeout),
    }
}

fn error_outcome(reason: String) -> BackendOutcome {
    BackendOutcome {
        verdict: Verdict::Error,
        gaps: Vec::new(),
        rationale: reason,
    }
}

/// Run a verifier subprocess under a wall-clock ceiling, isolating its nondeterminism (G1). The
/// single home of the timeout protocol every backend shares: a completed run's combined
/// stdout+stderr is handed to `interpret`; a hang past `timeout` is killed and surfaced as `error`
/// (never a hung runner, never a false verdict); a spawn or wait failure is likewise `error`.
/// `engine` names the tool in these operator-facing messages. The caller sets the command's args
/// and working directory; stdio piping is set here.
fn run_under_timeout(
    mut command: Command,
    timeout: Duration,
    engine: &str,
    interpret: fn(&str) -> BackendOutcome,
) -> BackendOutcome {
    let mut child = match command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => return error_outcome(format!("could not launch {engine}: {e}")),
    };
    match child.wait_timeout(timeout) {
        Ok(Some(_status)) => {
            let mut out = String::new();
            if let Some(mut stdout) = child.stdout.take() {
                let _ = stdout.read_to_string(&mut out);
            }
            if let Some(mut stderr) = child.stderr.take() {
                let _ = stderr.read_to_string(&mut out);
            }
            interpret(&out)
        }
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            error_outcome(format!(
                "{engine} exceeded {}s — the verifier's nondeterminism isolated",
                timeout.as_secs()
            ))
        }
        Err(e) => error_outcome(format!("waiting on {engine} failed: {e}")),
    }
}

fn run_dafny(prop_dir: &Path, timeout: Duration) -> BackendOutcome {
    if !prop_dir.join(MODEL_FILE).is_file() {
        return error_outcome(format!("lowering {MODEL_FILE} not found"));
    }
    // Working directory = the property dir, so `dafny` prints locations relative to MODEL_FILE
    // (no absolute/temp paths leak into the report — G1 reproducibility).
    let mut command = Command::new("dafny");
    command.arg("verify").arg(MODEL_FILE).current_dir(prop_dir);
    run_under_timeout(command, timeout, "dafny", interpret_dafny_output)
}

/// Map Dafny's output to a verdict. Pure — the seam that lets the mapping be tested with canned
/// output. Total over the summary line: a run is `proved` only when the verifier discharged at
/// least one obligation with no errors *and* no "gave up" category; verification errors are
/// `refuted` with a category-(B) gap; anything else — no recognizable summary, the verifier
/// declining to decide (out of resource / time out / inconclusive), or a vacuous run that verified
/// nothing — is `error`. A "gave up" outcome is never laundered into a proof (G1: Z3
/// nondeterminism is surfaced, never silently folded into a result).
fn interpret_dafny_output(output: &str) -> BackendOutcome {
    let Some(summary) = parse_summary(output) else {
        return error_outcome("dafny produced no recognizable result summary".to_string());
    };

    if summary.errors > 0 {
        return BackendOutcome {
            verdict: Verdict::Refuted,
            gaps: vec![Gap {
                code: "B".to_string(),
                summary: "umbrella claim not discharged by the verifier".to_string(),
                detail: Some(extract_error_lines(output)),
            }],
            rationale: format!(
                "dafny reported {} verification error(s); the claim is not established",
                summary.errors
            ),
        };
    }

    if !summary.undecided.is_empty() {
        // The verifier declined to decide — Z3 exhausted its budget or gave up. This is NOT a
        // proof; surfacing it as `error` keeps the nondeterminism visible rather than folding a
        // "gave up" into a green `proved`.
        return error_outcome(format!(
            "dafny did not discharge the obligations ({}); the verifier gave up, not a proof",
            summary.undecided.join(", ")
        ));
    }

    if summary.verified == 0 {
        // 0 verified, 0 errors, nothing undecided: the lowering carried no obligations, so a proof
        // claim would be vacuous.
        return error_outcome(
            "dafny verified 0 obligations — the lowering carries nothing to prove".to_string(),
        );
    }

    BackendOutcome {
        verdict: Verdict::Proved,
        gaps: Vec::new(),
        rationale: format!(
            "dafny discharged every proof obligation ({} verified, 0 errors)",
            summary.verified
        ),
    }
}

/// The parsed Dafny summary line
/// (`Dafny program verifier finished with N verified, M error(s)[, K <category>]…`).
struct DafnySummary {
    verified: u64,
    errors: u64,
    /// Non-error, non-verified categories carrying a positive count — the verifier declining to
    /// decide (e.g. `out of resource`, `time out`, `inconclusive`). Each rendered as `"K label"`.
    undecided: Vec<String>,
}

/// Parse Dafny's summary line **totally**: classify every `count label` segment so no category
/// the verifier reports can be silently dropped. Returns `None` when no summary line is present.
fn parse_summary(output: &str) -> Option<DafnySummary> {
    let tail = output.lines().find_map(|l| {
        l.split_once("Dafny program verifier finished with ")
            .map(|(_, tail)| tail)
    })?;
    let mut summary = DafnySummary {
        verified: 0,
        errors: 0,
        undecided: Vec::new(),
    };
    for segment in tail.split(',') {
        let segment = segment.trim();
        let mut parts = segment.splitn(2, ' ');
        let Some(count) = parts.next().and_then(|c| c.parse::<u64>().ok()) else {
            continue;
        };
        match parts.next().unwrap_or("").trim() {
            "verified" => summary.verified = count,
            "error" | "errors" => summary.errors = count,
            label if count > 0 => summary.undecided.push(format!("{count} {label}")),
            _ => {}
        }
    }
    Some(summary)
}

/// The Dafny error lines, sorted for a deterministic report (G1). Locations are relative to
/// MODEL_FILE because the process ran with the property dir as its cwd.
fn extract_error_lines(output: &str) -> String {
    let mut lines: Vec<&str> = output.lines().filter(|l| l.contains(": Error:")).collect();
    lines.sort_unstable();
    lines.join("\n")
}

/// Disambiguates concurrent TLC metadirs so parallel/repeated runs never collide (TLC otherwise
/// names its states dir by wall-clock time and aborts on a same-second re-run — which would break
/// reproducibility). Not part of any report's content (G1).
static TLC_METADIR_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn run_tlc(prop_dir: &Path, timeout: Duration) -> BackendOutcome {
    if !prop_dir.join(TLA_MODEL_FILE).is_file() {
        return error_outcome(format!("lowering {TLA_MODEL_FILE} not found"));
    }
    // TLC writes a states/ metadir; direct it OUT of the property dir (so the overlay stays clean)
    // and make it unique per run (so a same-second re-run does not collide — reproducibility).
    let n = TLC_METADIR_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let metadir = std::env::temp_dir().join(format!("loom-tlc-{}-{}", std::process::id(), n));

    // Working directory = the property dir, so TLC prints locations relative to the module and
    // finds `model.cfg` by the module base name. `-workers 1` pins state counts for reproducible
    // output (G1 — the model checker's nondeterminism is isolated, not folded into a result).
    let mut command = Command::new("tlc");
    command
        .arg("-workers")
        .arg("1")
        .arg("-metadir")
        .arg(&metadir)
        .arg(TLA_MODEL_FILE)
        .current_dir(prop_dir);
    let outcome = run_under_timeout(command, timeout, "tlc", interpret_tlc_output);
    let _ = std::fs::remove_dir_all(&metadir);
    outcome
}

/// Does the output report a property violation (invariant or temporal)? Either shape is a
/// refutation carrying a counterexample.
fn tlc_reports_violation(output: &str) -> bool {
    output.contains("is violated") || output.contains("Temporal properties were violated")
}

/// Map TLC's output to a verdict. Pure — the seam that lets the mapping be tested with canned
/// output. Total: `proved` only on the explicit "no error" completion sentinel; a property
/// violation is `refuted` with a category-(B) gap carrying the counterexample trace; anything
/// else — a parse/semantic failure, an unexpected deadlock, an exhausted or killed run, an
/// unrecognized dump — is `error`. A "gave up" outcome is never laundered into a proof (G1).
fn interpret_tlc_output(output: &str) -> BackendOutcome {
    // A completed check with no error is the only path to `proved`.
    if output.contains("Model checking completed. No error has been found.") {
        return BackendOutcome {
            verdict: Verdict::Proved,
            gaps: Vec::new(),
            rationale: "tlc explored the full reachable state space with no violation".to_string(),
        };
    }

    if tlc_reports_violation(output) {
        return BackendOutcome {
            verdict: Verdict::Refuted,
            gaps: vec![Gap {
                code: "B".to_string(),
                summary: "model checker found a state violating the umbrella claim".to_string(),
                detail: Some(extract_tlc_trace(output)),
            }],
            rationale: "tlc found a counterexample; the claim does not hold".to_string(),
        };
    }

    // No success sentinel and no violation: TLC failed to decide (parse/semantic error, an
    // unexpected deadlock, a killed/exhausted run, or a runtime exception). Surface as `error` so
    // the nondeterminism/failure stays visible rather than becoming a false `proved`.
    error_outcome(
        "tlc produced no completion result — the checker did not decide, not a proof".to_string(),
    )
}

/// The counterexample region of a violated run: the violation line and the state trace, stopping
/// before the run's statistics. Deterministic under `-workers 1` (G1).
fn extract_tlc_trace(output: &str) -> String {
    let mut trace = Vec::new();
    let mut in_trace = false;
    for line in output.lines() {
        if !in_trace && tlc_reports_violation(line) {
            in_trace = true;
        }
        if in_trace {
            // The per-run statistics follow the trace; stop before them so the detail is just the
            // counterexample (and stays stable across runs).
            if line.contains("states generated") {
                break;
            }
            let line = line.trim_end();
            if !line.is_empty() {
                trace.push(line);
            }
        }
    }
    trace.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_substrate_routes_to_exactly_one_backend() {
        assert!(!Substrate::ALL.is_empty());
        for &s in Substrate::ALL {
            let _backend: Backend = dispatch(s);
        }
    }

    #[test]
    fn dispatch_is_stable_for_dafny() {
        assert_eq!(dispatch(Substrate::Dafny), Backend::Dafny);
        assert_eq!(dispatch(Substrate::Dafny).name(), "dafny");
    }

    #[test]
    fn tla_substrate_routes_to_the_tlc_backend() {
        // M-0018/AC-1 — the second substrate routes through the frozen seam to a distinct backend.
        assert_eq!(dispatch(Substrate::Tla), Backend::Tlc);
        assert_eq!(dispatch(Substrate::Tla).name(), "tlc");
    }

    #[test]
    fn clean_verification_is_proved() {
        let o =
            interpret_dafny_output("Dafny program verifier finished with 3 verified, 0 errors\n");
        assert_eq!(o.verdict, Verdict::Proved);
        assert!(o.gaps.is_empty());
    }

    #[test]
    fn verification_errors_become_a_category_b_gap() {
        let out =
            "model.dfy(88,0): Error: a postcondition could not be proved on this return path\n\
                   Dafny program verifier finished with 1 verified, 1 error\n";
        let o = interpret_dafny_output(out);
        assert_eq!(o.verdict, Verdict::Refuted);
        assert_eq!(o.gaps.len(), 1);
        assert_eq!(o.gaps[0].code, "B");
        assert!(o.gaps[0]
            .detail
            .as_ref()
            .unwrap()
            .contains("model.dfy(88,0)"));
    }

    #[test]
    fn unrecognized_output_is_an_error_not_a_pass() {
        let o = interpret_dafny_output("bespoke tool crash, no summary line\n");
        assert_eq!(o.verdict, Verdict::Error);
        assert!(o.gaps.is_empty());
    }

    #[test]
    fn missing_lowering_is_an_error_not_a_pass() {
        // Reaches run_dafny's "no model" guard before any subprocess — deterministic, no Dafny.
        let dir = std::env::temp_dir().join(format!("loom-backend-{}-nomodel", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mkdir");
        let outcome = run(Backend::Dafny, &dir, VERIFY_TIMEOUT);
        assert_eq!(outcome.verdict, Verdict::Error);
        assert!(outcome.gaps.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn out_of_resource_is_an_error_not_a_pass() {
        // Z3 exhausted its budget: 0 errors, but the obligation was NOT discharged. Reporting this
        // as `proved` is the worst failure mode for a verifier — a false proof. It must surface as
        // `error` so the nondeterminism stays visible (G1), never silently folded into a result.
        let o = interpret_dafny_output(
            "Dafny program verifier finished with 0 verified, 0 errors, 1 out of resource\n",
        );
        assert_eq!(o.verdict, Verdict::Error);
        assert!(o.gaps.is_empty());
    }

    #[test]
    fn time_out_is_an_error_not_a_pass() {
        let o = interpret_dafny_output(
            "Dafny program verifier finished with 2 verified, 0 errors, 1 time out\n",
        );
        assert_eq!(o.verdict, Verdict::Error);
        assert!(o.gaps.is_empty());
    }

    #[test]
    fn inconclusive_is_an_error_not_a_pass() {
        let o = interpret_dafny_output(
            "Dafny program verifier finished with 0 verified, 0 errors, 1 inconclusive\n",
        );
        assert_eq!(o.verdict, Verdict::Error);
    }

    #[test]
    fn a_zero_count_category_does_not_derail_a_proof() {
        // Defensive: a trailing category with count 0 carries no "gave up" signal and must be
        // ignored, not treated as undecided — the run is still a clean proof.
        let o = interpret_dafny_output(
            "Dafny program verifier finished with 1 verified, 0 errors, 0 out of resource\n",
        );
        assert_eq!(o.verdict, Verdict::Proved);
        assert!(o.gaps.is_empty());
    }

    #[test]
    fn verifying_nothing_is_an_error_not_a_vacuous_pass() {
        // No obligations were checked at all (0 verified, 0 errors) — a proof claim would be
        // vacuous. An empty or obligation-free lowering is an authoring error, not a pass.
        let o =
            interpret_dafny_output("Dafny program verifier finished with 0 verified, 0 errors\n");
        assert_eq!(o.verdict, Verdict::Error);
        assert!(o.gaps.is_empty());
    }

    #[test]
    fn error_lines_are_sorted_for_determinism() {
        let out = "b.dfy(2,0): Error: second\na.dfy(1,0): Error: first\n\
                   Dafny program verifier finished with 0 verified, 2 errors\n";
        let detail = interpret_dafny_output(out).gaps[0].detail.clone().unwrap();
        assert!(detail.starts_with("a.dfy(1,0)"), "sorted: {detail:?}");
    }

    // --- TLC backend (M-0018) -------------------------------------------------------------

    #[test]
    fn tla_backend_names_its_inputs() {
        assert_eq!(dispatch(Substrate::Tla).name(), "tlc");
        assert_eq!(Backend::Tlc.inputs(), &[TLA_MODEL_FILE, TLA_CONFIG_FILE]);
        // The Dafny arm stays pinned too, so the audit trail names the right artifact per backend.
        assert_eq!(Backend::Dafny.inputs(), &[MODEL_FILE]);
    }

    #[test]
    fn tlc_clean_completion_is_proved() {
        let out = "TLC2 Version 2.19\n\
                   Model checking completed. No error has been found.\n\
                   4 states generated, 4 distinct states found, 0 states left on queue.\n";
        let o = interpret_tlc_output(out);
        assert_eq!(o.verdict, Verdict::Proved);
        assert!(o.gaps.is_empty());
    }

    #[test]
    fn tlc_invariant_violation_becomes_a_category_b_gap_with_the_trace() {
        let out = "Computing initial states...\n\
                   Error: Invariant Inv is violated.\n\
                   Error: The behavior up to this point is:\n\
                   State 1: <Initial predicate>\n\
                   x = 0\n\
                   \n\
                   State 2: <Next line 5>\n\
                   x = 3\n\
                   \n\
                   4 states generated, 4 distinct states found, 0 states left on queue.\n";
        let o = interpret_tlc_output(out);
        assert_eq!(o.verdict, Verdict::Refuted);
        assert_eq!(o.gaps.len(), 1);
        assert_eq!(o.gaps[0].code, "B");
        let detail = o.gaps[0].detail.as_ref().unwrap();
        assert!(
            detail.contains("Invariant Inv is violated"),
            "names the claim: {detail:?}"
        );
        assert!(
            detail.contains("State 2:") && detail.contains("x = 3"),
            "carries the trace: {detail:?}"
        );
        // The trace stops before the run statistics, so it stays stable across runs.
        assert!(
            !detail.contains("states generated"),
            "trace excludes stats: {detail:?}"
        );
    }

    #[test]
    fn tlc_temporal_violation_is_refuted() {
        let out = "Error: Temporal properties were violated.\n\
                   Error: The following behavior constitutes a counter-example:\n\
                   State 1: <Initial predicate>\n\
                   x = 0\n\
                   3 states generated, 3 distinct states found, 0 states left on queue.\n";
        assert_eq!(interpret_tlc_output(out).verdict, Verdict::Refuted);
    }

    #[test]
    fn tlc_no_completion_result_is_an_error_not_a_pass() {
        // A killed / state-exhausted / exception run prints neither the success sentinel nor a
        // violation. Reporting it as `proved` would be a false proof — it must be `error`.
        let out = "TLC2 Version 2.19\n\
                   Computing initial states...\n\
                   TLC threw an unexpected exception.\n";
        let o = interpret_tlc_output(out);
        assert_eq!(o.verdict, Verdict::Error);
        assert!(o.gaps.is_empty());
    }

    #[test]
    fn tlc_parse_failure_is_an_error_not_a_pass() {
        let o = interpret_tlc_output("Error: Parsing or semantic analysis failed.\n");
        assert_eq!(o.verdict, Verdict::Error);
    }

    #[test]
    fn a_run_that_exceeds_the_timeout_is_an_error_not_a_hang() {
        // The shared G1 isolation branch: a subprocess outliving the ceiling is killed and
        // surfaced as `error`, never a hung runner or a false verdict. Uses `sleep` as a stand-in
        // slow verifier; the interpret fn must not be reached on a timeout.
        let mut command = Command::new("sleep");
        command.arg("30");
        let outcome = run_under_timeout(command, Duration::from_millis(200), "sleeper", |_| {
            panic!("interpret must not be called when the run times out")
        });
        assert_eq!(outcome.verdict, Verdict::Error);
        assert!(
            outcome.rationale.contains("exceeded"),
            "names the timeout: {}",
            outcome.rationale
        );
    }

    #[test]
    fn missing_tla_model_is_an_error_not_a_pass() {
        // Reaches run_tlc's "no model" guard before any subprocess — deterministic, no TLC.
        let dir = std::env::temp_dir().join(format!("loom-tlc-{}-nomodel", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mkdir");
        let outcome = run(Backend::Tlc, &dir, VERIFY_TIMEOUT);
        assert_eq!(outcome.verdict, Verdict::Error);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
