//! M-0018 / AC-2 + AC-3 — the second substrate (TLA+/TLC) verifies through the frozen seam.
//!
//! Running the runner over the `tlc-downstream` overlay: the FSM-terminality property is
//! model-checked clean (`proved`), and the at-risk cancel-reachability property surfaces its
//! real counterexample as a category-(B) gap. Reproducible. Requires TLC — skipped with a notice
//! when it is not on PATH, so loom's suite stays portable (the TLC output→verdict mapping is
//! unit-tested with canned output in `src/backend.rs`, and always runs).

mod common;
use common::copy_dir;
use loom::{OVERLAY_DIR, REPORT_FILE};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Is TLC on PATH? The end-to-end assertions need it; without it they skip with a notice.
fn tlc_available() -> bool {
    Command::new("tlc")
        .output()
        .map(|o| {
            let combined = format!(
                "{}{}",
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
            combined.contains("TLC2")
        })
        .unwrap_or(false)
}

/// The TLA+ second-substrate downstream fixture (a stand-in repo carrying `tla` properties).
fn tlc_fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap() // crates/
        .parent()
        .unwrap() // workspace root
        .join("examples/tlc-downstream")
}

/// Copy the fixture into a unique temp dir so verification can write reports hermetically.
fn temp_overlay(tag: &str) -> PathBuf {
    let tmp = std::env::temp_dir().join(format!("loom-tlc-fx-{}-{}", std::process::id(), tag));
    let _ = std::fs::remove_dir_all(&tmp);
    copy_dir(&tlc_fixture(), &tmp);
    tmp.join(OVERLAY_DIR)
}

fn read_report(overlay: &Path, prop: &str) -> Value {
    let bytes = std::fs::read_to_string(overlay.join(prop).join(REPORT_FILE)).expect("read report");
    serde_json::from_str(&bytes).expect("report parses")
}

#[test]
fn fsm_terminality_is_model_checked_proved() {
    // AC-2 — a TLA+ property verifies `proved` end to end, routed through the TLC backend.
    if !tlc_available() {
        eprintln!("skipping: tlc not on PATH (AC-2 needs the TLA+ model checker)");
        return;
    }
    let overlay = temp_overlay("proved");
    loom::runner::verify(&overlay, Some("fsm-terminality")).expect("verify");
    let r = read_report(&overlay, "fsm-terminality");

    assert_eq!(
        r["verdict"], "proved",
        "fsm-terminality must model-check clean"
    );
    assert_eq!(r["substrate"], "tla", "routed to the tla substrate");
    assert!(
        r["gaps"].as_array().unwrap().is_empty(),
        "no gaps on a clean check"
    );
    // The audit records the model checker's own artifacts, not the Dafny lowering.
    let inputs = r["audit"]["inputs"].as_array().unwrap();
    assert_eq!(
        inputs,
        &[Value::from("model.tla"), Value::from("model.cfg")]
    );
    // Subject pinned through the same binding the Dafny backend uses.
    assert_eq!(r["subject"]["repo"], "aiwf");
    let _ = std::fs::remove_dir_all(overlay.parent().unwrap());
}

#[test]
fn cancel_reachability_surfaces_a_counterexample_gap() {
    // AC-3 — the at-risk property is refuted, and TLC's counterexample trace becomes the gap.
    if !tlc_available() {
        eprintln!("skipping: tlc not on PATH (AC-3 needs the TLA+ model checker)");
        return;
    }
    let overlay = temp_overlay("refuted");
    loom::runner::verify(&overlay, Some("cancel-reachability")).expect("verify");
    let r = read_report(&overlay, "cancel-reachability");

    assert_eq!(r["verdict"], "refuted", "the at-risk claim does not hold");
    let gaps = r["gaps"].as_array().unwrap();
    assert_eq!(gaps.len(), 1, "exactly one gap");
    assert_eq!(gaps[0]["code"], "B", "category (B): claimed but refuted");
    let detail = gaps[0]["detail"].as_str().unwrap();
    assert!(
        detail.contains("is violated"),
        "the gap names the violated invariant: {detail:?}"
    );
    assert!(
        detail.contains("cancelled"),
        "the gap carries the counterexample reaching cancelled: {detail:?}"
    );
    let _ = std::fs::remove_dir_all(overlay.parent().unwrap());
}

#[test]
fn tlc_verification_is_reproducible() {
    // G1 — two runs produce byte-identical reports (the checker's nondeterminism is isolated).
    if !tlc_available() {
        eprintln!("skipping reproducibility: tlc not on PATH");
        return;
    }
    let overlay = temp_overlay("repro");
    loom::runner::verify(&overlay, None).expect("run 1");
    let first = std::fs::read(overlay.join("fsm-terminality").join(REPORT_FILE)).unwrap();
    loom::runner::verify(&overlay, None).expect("run 2");
    let second = std::fs::read(overlay.join("fsm-terminality").join(REPORT_FILE)).unwrap();
    assert_eq!(
        first, second,
        "TLC reports must be byte-identical across runs (G1)"
    );
    let _ = std::fs::remove_dir_all(overlay.parent().unwrap());
}
