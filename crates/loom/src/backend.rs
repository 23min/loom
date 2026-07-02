//! Substrate → backend dispatch and execution — Contract 5 / §4.5 totality.
//!
//! Routing is total *by construction*: [`dispatch`] is an exhaustive match over [`Substrate`],
//! so adding a substrate without a backend is a compile error — nothing is silently unverified.
//! [`run`] executes the routed backend. The Dafny backend shells out to `dafny verify` with the
//! property directory as the working directory (so error locations are relative and reproducible,
//! G1) under a wall-clock timeout (Z3 nondeterminism is isolated — a hang becomes an `error`
//! verdict, never a hung runner). The pure output→verdict mapping ([`interpret_dafny_output`]) is
//! unit-tested with canned output and needs no Dafny.

use crate::report::{Gap, Substrate, Verdict};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

/// The per-property lowering file the backend verifies (Contract 3 — an attached artifact).
pub(crate) const MODEL_FILE: &str = "model.dfy";

/// Wall-clock ceiling on a single verification. A hang past this is surfaced as `error`, not a
/// hung runner (G1 — the verifier's nondeterminism is isolated and reported, never folded in).
pub(crate) const DAFNY_TIMEOUT: Duration = Duration::from_secs(120);

/// The verification engine responsible for a substrate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Backend {
    /// Dafny + Z3.
    Dafny,
}

impl Backend {
    /// The backend's short name, for reports and logs.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Backend::Dafny => "dafny",
        }
    }
}

/// Route a substrate to exactly one backend. Total: the match is exhaustive, so every substrate
/// maps to one backend and none is silently unverified.
pub(crate) fn dispatch(substrate: Substrate) -> Backend {
    match substrate {
        Substrate::Dafny => Backend::Dafny,
    }
}

/// What a backend run yields: a graded verdict, any gaps, and the audit rationale.
#[derive(Debug, Clone)]
pub(crate) struct BackendOutcome {
    pub verdict: Verdict,
    pub gaps: Vec<Gap>,
    pub rationale: String,
}

/// Run the routed backend over the property directory (which holds the [`MODEL_FILE`] lowering).
pub(crate) fn run(backend: Backend, prop_dir: &Path, timeout: Duration) -> BackendOutcome {
    match backend {
        Backend::Dafny => run_dafny(prop_dir, timeout),
    }
}

fn error_outcome(reason: String) -> BackendOutcome {
    BackendOutcome {
        verdict: Verdict::Error,
        gaps: Vec::new(),
        rationale: reason,
    }
}

fn run_dafny(prop_dir: &Path, timeout: Duration) -> BackendOutcome {
    if !prop_dir.join(MODEL_FILE).is_file() {
        return error_outcome(format!("lowering {MODEL_FILE} not found"));
    }
    // Working directory = the property dir, so `dafny` prints locations relative to MODEL_FILE
    // (no absolute/temp paths leak into the report — G1 reproducibility).
    let mut child = match Command::new("dafny")
        .arg("verify")
        .arg(MODEL_FILE)
        .current_dir(prop_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => return error_outcome(format!("could not launch dafny: {e}")),
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
            interpret_dafny_output(&out)
        }
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            error_outcome(format!(
                "dafny verification exceeded {}s — Z3 nondeterminism isolated",
                timeout.as_secs()
            ))
        }
        Err(e) => error_outcome(format!("waiting on dafny failed: {e}")),
    }
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
        let outcome = run(Backend::Dafny, &dir, DAFNY_TIMEOUT);
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
}
