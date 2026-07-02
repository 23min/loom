//! M-0016 / AC-4 — reports are written atomically (C3) and reproducibly (G1).
//!
//! The atomicity of the write primitive (temp-then-rename; a withheld commit leaves the prior
//! report intact-or-absent, never partial) is unit-tested in `src/atomic.rs`. Here we pin the
//! runner's determinism: the same overlay + same pinned source yields byte-identical reports.

mod common;
use common::{discovered_properties, temp_copy_of_fixture};
use loom::{OVERLAY_DIR, REPORT_FILE};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn read_reports(overlay: &Path) -> BTreeMap<String, Vec<u8>> {
    discovered_properties(overlay)
        .into_iter()
        .map(|p| {
            let bytes = fs::read(overlay.join(&p).join(REPORT_FILE)).expect("read report");
            (p, bytes)
        })
        .collect()
}

#[test]
fn two_runs_produce_byte_identical_reports() {
    let tmp = temp_copy_of_fixture("ac4-determinism");
    let overlay = tmp.join(OVERLAY_DIR);

    loom::runner::verify(&overlay, None).expect("run 1");
    let run1 = read_reports(&overlay);
    loom::runner::verify(&overlay, None).expect("run 2");
    let run2 = read_reports(&overlay);

    assert!(!run1.is_empty(), "fixture overlay should carry properties");
    assert_eq!(
        run1, run2,
        "reports must be byte-identical across runs — no time/randomness in the output (G1)"
    );
    let _ = fs::remove_dir_all(&tmp);
}
