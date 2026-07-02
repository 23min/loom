//! M-0016 / AC-3 — the load-bearing seam (Contract 2, D2).
//!
//! Two guarantees over the shared on-disk scenarios in `tests/scenarios/`:
//!
//! 1. **writer ↔ reader equivalence.** A reader written *independently* of loom's serde types
//!    (it pulls fields by canonical name out of a `serde_json::Value`, modelling a foreign
//!    consumer such as aiwf's future Go reader) sees the same facts whether it reads the shared
//!    scenario or the writer's re-emission of it. If either side's shape drifts, they disagree.
//!    The scenario files are the language-neutral contract a future Go reader test reuses.
//! 2. **runner output validates.** Every report the runner actually writes validates against the
//!    frozen, published JSON Schema.
//!
//! Honest scope: aiwf's real Go reader is out of scope for M-0016 (the subject is referenced
//! read-only), so the equivalence here is writer ↔ an-independent-reader-in-loom. The checked-in
//! schema file is what actually guarantees agreement across a language boundary.

mod common;
use common::temp_copy_of_fixture;
use loom::report::{GapReport, SCHEMA_RELPATH};
use loom::OVERLAY_DIR;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

const SCENARIOS: &[&str] = &[
    "proved-fsm-terminality.json",
    "refuted-cancel-edge-legality.json",
    "pending-archive-terminality.json",
];

fn manifest(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn scenario_path(name: &str) -> PathBuf {
    manifest("tests/scenarios").join(name)
}

/// What an independent consumer recovers from a report — extracted by canonical field name,
/// deliberately NOT via loom's `GapReport` type, so this reader can drift from the writer.
#[derive(Debug, PartialEq, Eq)]
struct ConsumerView {
    schema_version: String,
    property: String,
    verdict: String,
    substrate: Option<String>,
    subject_ref: Option<String>,
    subject_path: Option<String>,
    gap_codes: Vec<String>,
    audit_checked: String,
}

fn nested_str(v: &Value, outer: &str, inner: &str) -> Option<String> {
    v.get(outer)
        .and_then(|o| o.get(inner))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn read_independently(json: &str) -> ConsumerView {
    let v: Value = serde_json::from_str(json).expect("consumer parses JSON");
    ConsumerView {
        schema_version: v["schema_version"]
            .as_str()
            .expect("schema_version")
            .to_string(),
        property: v["property"].as_str().expect("property").to_string(),
        verdict: v["verdict"].as_str().expect("verdict").to_string(),
        substrate: v["substrate"].as_str().map(str::to_string),
        subject_ref: nested_str(&v, "subject", "ref"),
        subject_path: nested_str(&v, "subject", "path"),
        gap_codes: v["gaps"]
            .as_array()
            .expect("gaps array")
            .iter()
            .map(|g| g["code"].as_str().expect("gap code").to_string())
            .collect(),
        audit_checked: v["audit"]["checked"]
            .as_str()
            .expect("audit.checked")
            .to_string(),
    }
}

#[test]
fn writer_and_independent_reader_agree_on_shared_scenarios() {
    for name in SCENARIOS {
        let canonical = fs::read_to_string(scenario_path(name)).expect("read scenario");
        // A consumer reading the shared contract fixture.
        let view_canonical = read_independently(&canonical);
        // The writer ingests the fixture (deny_unknown_fields ⇒ strict) and re-emits it.
        let report: GapReport = serde_json::from_str(&canonical)
            .unwrap_or_else(|e| panic!("writer must ingest {name}: {e}"));
        let emitted = report.to_canonical_json();
        // The same consumer reading the writer's output.
        let view_emitted = read_independently(&emitted);
        assert_eq!(
            view_canonical, view_emitted,
            "writer<->reader disagree on {name}: the writer's emission drifted from the shared scenario"
        );
    }
}

#[test]
fn runner_output_validates_against_the_frozen_schema() {
    let tmp = temp_copy_of_fixture("ac3-validate");
    let overlay = tmp.join(OVERLAY_DIR);
    let written = loom::runner::verify(&overlay, None).expect("verify");
    assert!(
        !written.is_empty(),
        "fixture overlay should carry properties"
    );

    let schema: Value =
        serde_json::from_str(&fs::read_to_string(manifest(SCHEMA_RELPATH)).expect("read schema"))
            .expect("schema parses");
    let validator = jsonschema::validator_for(&schema).expect("compile schema");

    for w in &written {
        let bytes = fs::read_to_string(&w.path).expect("read emitted report");
        let instance: Value = serde_json::from_str(&bytes).expect("report parses");
        let errors: Vec<String> = validator
            .iter_errors(&instance)
            .map(|e| e.to_string())
            .collect();
        assert!(
            errors.is_empty(),
            "runner report for {} failed schema validation: {errors:?}",
            w.property
        );
    }
    let _ = fs::remove_dir_all(&tmp);
}
