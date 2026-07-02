//! M-0016 / AC-2 — `make loom` is opt-in and off the default pipeline.
//!
//! Three claims, one per test: (1) the default `make` target's command graph never
//! invokes loom; (2) `make loom` writes a gap report per property; (3) `make loom
//! PROP=<id>` scopes to a single property. Contract 4 (the runner interface) plus the
//! opt-in constraint. Report writes land in a temp copy so the real fixture stays clean.

mod common;
use common::{discovered_properties, fixture, loom_bin, run_make, temp_copy_of_fixture};
use loom::{OVERLAY_DIR, REPORT_FILE};
use std::process::Command;

#[test]
fn default_target_command_graph_never_invokes_loom() {
    // `make -n all` prints the commands the default target WOULD run, without running
    // them — the real command graph, not a text grep of the Makefile source.
    let (ok, out, err) = run_make(&fixture(), &["-n", "all"]);
    assert!(ok, "`make -n all` failed: {err}");
    assert!(
        !out.contains("verify") && !out.to_lowercase().contains("loom"),
        "default target must invoke no loom command; `make -n all` printed:\n{out}"
    );
}

#[test]
fn make_loom_writes_a_report_per_property() {
    let tmp = temp_copy_of_fixture("loom-all");
    let overlay = tmp.join(OVERLAY_DIR);
    let expected = discovered_properties(&overlay);
    assert!(
        expected.len() >= 2,
        "fixture should carry several properties, found {expected:?}"
    );

    let bin = format!("LOOM_BIN={}", loom_bin());
    let (ok, _out, err) = run_make(&tmp, &["loom", bin.as_str()]);
    assert!(ok, "`make loom` failed: {err}");

    for prop in &expected {
        let report = overlay.join(prop).join(REPORT_FILE);
        assert!(
            report.is_file(),
            "`make loom` must write a report for every property; missing {}",
            report.display()
        );
    }
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn make_loom_prop_scopes_to_a_single_property() {
    let tmp = temp_copy_of_fixture("loom-one");
    let overlay = tmp.join(OVERLAY_DIR);
    let all = discovered_properties(&overlay);
    let target = "fsm-terminality";
    assert!(
        all.iter().any(|p| p == target),
        "fixture must carry the `{target}` property, found {all:?}"
    );

    let bin = format!("LOOM_BIN={}", loom_bin());
    let prop = format!("PROP={target}");
    let (ok, _out, err) = run_make(&tmp, &["loom", bin.as_str(), prop.as_str()]);
    assert!(ok, "`make loom PROP={target}` failed: {err}");

    assert!(
        overlay.join(target).join(REPORT_FILE).is_file(),
        "the scoped property must get a report"
    );
    for other in all.iter().filter(|p| p.as_str() != target) {
        assert!(
            !overlay.join(other).join(REPORT_FILE).exists(),
            "`PROP={target}` must not touch other properties, but wrote a report for {other}"
        );
    }
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn cli_rejects_malformed_invocations_and_unknown_properties() {
    // Exit-code contract for the runner CLI. Every case below either errors before the
    // verify step or (for the unknown-property case) errors before any write, so running
    // them against the real fixture overlay never mutates it.
    let overlay = fixture().join(OVERLAY_DIR);
    let ov = overlay.to_str().expect("overlay path is utf-8");
    let cases: &[(&[&str], i32)] = &[
        (&[], 2),                                // no subcommand
        (&["verify"], 2),                        // no overlay argument
        (&["verify", ov, "--prop"], 2),          // --prop without a value
        (&["verify", ov, "extra"], 2),           // unexpected extra positional
        (&["verify", ov, "--prop", "ghost"], 1), // overlay exists, property does not
    ];
    for (args, want) in cases {
        let status = Command::new(loom_bin())
            .args(*args)
            .status()
            .expect("run loom binary");
        assert_eq!(
            status.code(),
            Some(*want),
            "`loom {}` should exit {want}",
            args.join(" ")
        );
    }
}

#[test]
fn cli_treats_an_empty_overlay_as_success() {
    // An overlay with no properties is not an error — the runner reports "none" and exits 0.
    let tmp = std::env::temp_dir().join(format!("loom-empty-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).expect("mkdir empty overlay");
    let status = Command::new(loom_bin())
        .args(["verify", tmp.to_str().expect("utf-8 path")])
        .status()
        .expect("run loom binary");
    assert_eq!(
        status.code(),
        Some(0),
        "an empty overlay must not be an error"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}
