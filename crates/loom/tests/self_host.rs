//! M-0017 — self-host: loom verifies loom's own trust-critical plumbing through loom.
//!
//! The `loom/` overlay at the repo root carries three self-host properties whose Dafny
//! lowerings model loom's load-bearing invariants (dispatch totality, parser totality,
//! atomic-write crash-safety) and are pinned to the Rust symbols they stand for. This
//! binary drives the M-0016 runner over that overlay and asserts:
//!   * AC-1 — the overlay is a valid loom overlay, opt-in, and off the default cargo graph.
//!   * AC-2/3/4 — each property verifies `proved` under real Dafny (skipped without Dafny).
//!   * AC-5 — each report's `subject` records the pinned Rust symbol + version (no schema change).
//!
//! The models mirror the Rust by reference; they do not verify the Rust directly (no
//! extraction — ADR-0017). Dafny-backed assertions are skipped with a notice when Dafny is
//! not on PATH, so loom's suite stays portable.

mod common;
use common::copy_dir;
use loom::runner::discover_properties;
use loom::REPORT_FILE;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The workspace root — `crates/loom`'s grandparent.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap() // crates/
        .parent()
        .unwrap() // workspace root
        .to_path_buf()
}

/// The self-host overlay directory (repo-root `loom/`).
fn self_host_overlay() -> PathBuf {
    repo_root().join(loom::OVERLAY_DIR)
}

/// The three self-host properties, sorted (the runner sorts by id for G1 determinism).
const SELF_HOST_PROPERTIES: [&str; 3] = [
    "atomic-crash-safety",
    "dispatch-totality",
    "parser-totality",
];

/// Is Dafny on PATH? The self-host verdict assertions need it; without it they skip with a
/// notice, so the suite stays portable (the output→verdict mapping is unit-tested in
/// `src/backend.rs` and always runs).
fn dafny_available() -> bool {
    Command::new("dafny")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Copy the repo-root self-host overlay into a unique temp dir so verification can write
/// reports without mutating the working tree (and each test runs hermetically).
fn temp_copy_of_overlay(tag: &str) -> PathBuf {
    let tmp = std::env::temp_dir().join(format!("loom-selfhost-{}-{}", std::process::id(), tag));
    let _ = std::fs::remove_dir_all(&tmp);
    copy_dir(&self_host_overlay(), &tmp);
    tmp
}

/// Read one property's generated report as JSON.
fn read_report(overlay: &Path, prop: &str) -> Value {
    let bytes = std::fs::read_to_string(overlay.join(prop).join(REPORT_FILE)).expect("read report");
    serde_json::from_str(&bytes).expect("report parses")
}

/// Verify a single self-host property and return its report. Skips (returns `None`) when Dafny
/// is not on PATH.
fn verify_one(tag: &str, prop: &str) -> Option<(PathBuf, Value)> {
    if !dafny_available() {
        eprintln!("skipping {prop}: dafny not on PATH (self-host verification needs Dafny)");
        return None;
    }
    let overlay = temp_copy_of_overlay(tag);
    loom::runner::verify(&overlay, Some(prop)).expect("verify");
    let report = read_report(&overlay, prop);
    Some((overlay, report))
}

/// Assert a self-host property verifies cleanly (`proved`, dafny substrate, no gaps).
fn assert_proves(tag: &str, prop: &str) {
    let Some((overlay, r)) = verify_one(tag, prop) else {
        return;
    };
    assert_eq!(r["verdict"], "proved", "{prop} must verify proved");
    assert_eq!(
        r["substrate"], "dafny",
        "{prop} routes to the dafny backend"
    );
    assert!(
        r["gaps"].as_array().unwrap().is_empty(),
        "{prop} carries no gaps on a clean proof"
    );
    let _ = std::fs::remove_dir_all(&overlay);
}

#[test]
fn dispatch_totality_self_hosts_and_verifies() {
    // AC-2 — `loom::backend::dispatch` totality, self-hosted and verified proved.
    assert_proves("dispatch", "dispatch-totality");
}

#[test]
fn parser_totality_self_hosts_and_verifies() {
    // AC-3 — `loom::umbrella::parse` totality, self-hosted and verified proved.
    assert_proves("parser", "parser-totality");
}

#[test]
fn atomic_crash_safety_self_hosts_and_verifies() {
    // AC-4 — `loom::atomic` crash-safety, self-hosted and verified proved.
    assert_proves("atomic", "atomic-crash-safety");
}

#[test]
fn self_host_overlay_carries_exactly_the_three_properties() {
    let ids: Vec<String> = discover_properties(&self_host_overlay())
        .expect("discover self-host properties")
        .into_iter()
        .map(|p| p.id)
        .collect();
    assert_eq!(
        ids,
        SELF_HOST_PROPERTIES.map(String::from).to_vec(),
        "the self-host overlay must carry exactly the three modelable properties"
    );
}

#[test]
fn self_host_overlay_is_off_the_default_cargo_graph() {
    // Containment, mirrored onto loom-on-loom: the overlay is not a cargo package and not a
    // workspace member, so `cargo build`/`cargo test` never touch it. Removing `loom/` leaves
    // loom's own pipeline byte-identical — there is nothing in the build graph to change.
    let overlay = self_host_overlay();
    assert!(
        !overlay.join("Cargo.toml").exists(),
        "the overlay must not be a cargo package — it is pure data, off the build graph"
    );
    let manifest =
        std::fs::read_to_string(repo_root().join("Cargo.toml")).expect("root Cargo.toml");
    let members = manifest
        .lines()
        .find(|l| l.trim_start().starts_with("members"))
        .expect("workspace declares members");
    assert!(
        members.contains("crates/loom") && !members.contains("\"loom\""),
        "workspace members must build only crates/loom, never the overlay: {members:?}"
    );
}

#[test]
fn opt_in_alias_runs_verify_over_the_overlay() {
    // The opt-in entry (`cargo loom`) is a cargo alias — never a prerequisite of `cargo build`
    // or `cargo test`. It invokes the runner over the overlay and nothing else.
    let config = std::fs::read_to_string(repo_root().join(".cargo/config.toml"))
        .expect("read .cargo config");
    let alias = config
        .lines()
        .find(|l| l.trim_start().starts_with("loom"))
        .expect("`.cargo/config.toml` declares a `loom` alias");
    assert!(
        alias.contains("verify") && alias.contains(loom::OVERLAY_DIR),
        "the `loom` alias must run `verify` over the overlay: {alias:?}"
    );
}

/// The pinned Rust subject each self-host property's model stands for: (repo, ref, path, symbol).
const EXPECTED_SUBJECTS: [(&str, &str, &str, &str, &str); 3] = [
    (
        "dispatch-totality",
        "loom",
        "v0.1.0",
        "crates/loom/src/backend.rs",
        "loom::backend::dispatch",
    ),
    (
        "parser-totality",
        "loom",
        "v0.1.0",
        "crates/loom/src/umbrella.rs",
        "loom::umbrella::parse",
    ),
    (
        "atomic-crash-safety",
        "loom",
        "v0.1.0",
        "crates/loom/src/atomic.rs",
        "loom::atomic",
    ),
];

#[test]
fn each_self_host_report_records_its_pinned_subject() {
    // AC-5 — `loom verify` populates each report's `subject` with the pinned Rust symbol +
    // version the model stands for. This is a parse-time fact (independent of Dafny), so the
    // test runs without a verifier: it asserts the subject binding, not the verdict.
    let overlay = temp_copy_of_overlay("subjects");
    loom::runner::verify(&overlay, None).expect("verify");
    for (prop, repo, reference, path, symbol) in EXPECTED_SUBJECTS {
        let r = read_report(&overlay, prop);
        let subject = &r["subject"];
        assert!(
            subject.is_object(),
            "{prop}: subject must be populated, got {subject}"
        );
        assert_eq!(subject["repo"], repo, "{prop}: subject.repo");
        assert_eq!(subject["ref"], reference, "{prop}: subject.ref");
        assert_eq!(subject["path"], path, "{prop}: subject.path");
        assert_eq!(subject["symbol"], symbol, "{prop}: subject.symbol");
    }
    let _ = std::fs::remove_dir_all(&overlay);
}

#[test]
fn subject_populated_reports_still_satisfy_the_frozen_schema() {
    // AC-5 — expressing loom's own subjects required NO change to the frozen gap-report schema
    // (Contract 2): the `subject` field already existed. Proof from the inside — every
    // subject-populated self-host report validates against the checked-in frozen schema.
    let schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(loom::report::SCHEMA_RELPATH);
    let schema: Value =
        serde_json::from_str(&std::fs::read_to_string(&schema_path).expect("read frozen schema"))
            .expect("schema parses");
    let validator = jsonschema::validator_for(&schema).expect("compile frozen schema");

    let overlay = temp_copy_of_overlay("schema");
    loom::runner::verify(&overlay, None).expect("verify");
    for (prop, ..) in EXPECTED_SUBJECTS {
        let instance = read_report(&overlay, prop);
        let errors: Vec<String> = validator
            .iter_errors(&instance)
            .map(|e| e.to_string())
            .collect();
        assert!(
            errors.is_empty(),
            "{prop}: subject-populated report must validate against the frozen schema: {errors:?}"
        );
    }
    let _ = std::fs::remove_dir_all(&overlay);
}

/// Every property directory carries an `umbrella.md` — the overlay marker the runner keys on.
#[test]
fn every_self_host_property_has_an_umbrella() {
    let overlay = self_host_overlay();
    for prop in SELF_HOST_PROPERTIES {
        assert!(
            overlay.join(prop).join(loom::UMBRELLA_FILE).is_file(),
            "{prop} must carry an {}",
            loom::UMBRELLA_FILE
        );
    }
}
