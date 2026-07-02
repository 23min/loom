//! M-0016 / AC-5 — parse + dispatch are total, end to end.
//!
//! The parser and dispatch are unit-tested in `src/umbrella.rs` and `src/backend.rs`. Here we
//! pin the runner's totality: a well-formed umbrella routes and populates `substrate`; a
//! malformed one produces an explicit *error* report — never a silent skip, never silently
//! unverified.

mod common;
use common::{discovered_properties, temp_copy_of_fixture};
use loom::{OVERLAY_DIR, REPORT_FILE, UMBRELLA_FILE};
use serde_json::Value;
use std::fs;
use std::path::Path;

fn read_report(overlay: &Path, property: &str) -> Value {
    let bytes = fs::read_to_string(overlay.join(property).join(REPORT_FILE)).expect("read report");
    serde_json::from_str(&bytes).expect("report parses")
}

#[test]
fn wellformed_umbrellas_route_and_malformed_ones_error() {
    let tmp = temp_copy_of_fixture("ac5-totality");
    let overlay = tmp.join(OVERLAY_DIR);

    // Corrupt one property's umbrella to an unknown substrate — it must be flagged, not skipped.
    let victim = "cancel-edge-legality";
    fs::write(
        overlay.join(victim).join(UMBRELLA_FILE),
        "# broken umbrella\n\nsubstrate: coq\n",
    )
    .expect("overwrite umbrella");

    loom::runner::verify(&overlay, None).expect("verify");

    let properties = discovered_properties(&overlay);
    assert!(
        properties.contains(&victim.to_string()),
        "victim still discovered"
    );

    for property in &properties {
        let report = read_report(&overlay, property);
        if property == victim {
            assert_eq!(
                report["verdict"], "error",
                "a malformed umbrella must yield an explicit error report"
            );
            let codes: Vec<&str> = report["gaps"]
                .as_array()
                .unwrap()
                .iter()
                .map(|g| g["code"].as_str().unwrap())
                .collect();
            assert!(
                codes.contains(&"parse"),
                "the parse failure must be recorded as a gap"
            );
            assert_eq!(
                report["substrate"],
                Value::Null,
                "unparsed property has no substrate"
            );
        } else {
            // Totality of routing: a well-formed umbrella is routed to its substrate. The verdict
            // is the backend's concern (AC-6) and depends on Dafny's availability, so it is not
            // asserted here.
            assert_eq!(
                report["substrate"], "dafny",
                "a well-formed umbrella routes to its substrate"
            );
        }
    }
    let _ = fs::remove_dir_all(&tmp);
}
