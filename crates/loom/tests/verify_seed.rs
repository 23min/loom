//! M-0016 / AC-6 — the seed properties verify, and the at-risk gap surfaces (value demo).
//!
//! Running the runner over the three-property aiwf overlay: FSM-terminality and
//! archive⇔terminality verify clean; cancel-edge-legality's report carries the real `(B)`
//! finding. Reproducible. Requires Dafny — skipped with a notice when it is not on PATH, so
//! loom's suite stays portable without laundering the value claim (the backend's output-→-verdict
//! mapping is unit-tested with canned Dafny output in `src/backend.rs`, and always runs).

mod common;
use common::{discovered_properties, temp_copy_of_fixture};
use loom::{OVERLAY_DIR, REPORT_FILE};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

fn dafny_available() -> bool {
    Command::new("dafny")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn read_report(overlay: &Path, prop: &str) -> Value {
    let bytes = fs::read_to_string(overlay.join(prop).join(REPORT_FILE)).expect("read report");
    serde_json::from_str(&bytes).expect("report parses")
}

fn read_all(overlay: &Path) -> Vec<(String, Vec<u8>)> {
    discovered_properties(overlay)
        .into_iter()
        .map(|p| {
            let bytes = fs::read(overlay.join(&p).join(REPORT_FILE)).expect("read report");
            (p, bytes)
        })
        .collect()
}

#[test]
fn seed_properties_verify_and_the_at_risk_gap_surfaces() {
    if !dafny_available() {
        eprintln!("skipping seed verification: dafny not on PATH (AC-6 value demo needs Dafny)");
        return;
    }
    let tmp = temp_copy_of_fixture("ac6-verify");
    let overlay = tmp.join(OVERLAY_DIR);
    loom::runner::verify(&overlay, None).expect("verify");

    // Two properties verify clean — category (A).
    for prop in ["fsm-terminality", "archive-terminality"] {
        let r = read_report(&overlay, prop);
        assert_eq!(r["verdict"], "proved", "{prop} should verify clean");
        assert_eq!(r["substrate"], "dafny");
        assert!(
            r["gaps"].as_array().unwrap().is_empty(),
            "{prop} should carry no gaps"
        );
    }

    // The at-risk property surfaces the real (B) finding — category (B).
    let cancel = read_report(&overlay, "cancel-edge-legality");
    assert_eq!(
        cancel["verdict"], "refuted",
        "cancel-edge-legality's claim is not discharged"
    );
    let gaps = cancel["gaps"].as_array().unwrap();
    assert_eq!(gaps.len(), 1, "exactly one gap");
    assert_eq!(gaps[0]["code"], "B", "category (B): claimed but not proved");
    assert!(
        gaps[0]["detail"].as_str().unwrap().contains("model.dfy"),
        "the gap cites the lowering's failing obligation"
    );

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn seed_verification_is_reproducible() {
    if !dafny_available() {
        eprintln!("skipping reproducibility: dafny not on PATH");
        return;
    }
    let tmp = temp_copy_of_fixture("ac6-repro");
    let overlay = tmp.join(OVERLAY_DIR);
    loom::runner::verify(&overlay, None).expect("run 1");
    let first = read_all(&overlay);
    loom::runner::verify(&overlay, None).expect("run 2");
    let second = read_all(&overlay);
    assert_eq!(
        first, second,
        "verification reports must be byte-identical across runs (G1)"
    );
    let _ = fs::remove_dir_all(&tmp);
}
