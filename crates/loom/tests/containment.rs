//! M-0016 / AC-1 — the overlay is contained and removable-without-trace.
//!
//! These tests pin the containment contract: the entire loom footprint lives under the
//! overlay directory, and the host's default pipeline is byte-identical whether or not
//! the overlay is present. Fixture helpers are shared via `common`.

mod common;
use common::{fixture, run_make, temp_copy_of_fixture};
use loom::OVERLAY_DIR;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Content-snapshot every file under `root`, keyed by path relative to `root`,
/// skipping anything under the `skip` subdirectory.
fn snapshot(root: &Path, skip: &str) -> BTreeMap<PathBuf, Vec<u8>> {
    let skip_root = root.join(skip);
    let mut out = BTreeMap::new();
    fn walk(dir: &Path, root: &Path, skip: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(dir).expect("read_dir") {
            let p = entry.expect("dir entry").path();
            if p == *skip {
                continue;
            }
            if p.is_dir() {
                walk(&p, root, skip, out);
            } else {
                let rel = p.strip_prefix(root).expect("under root").to_path_buf();
                out.insert(rel, fs::read(&p).expect("read file"));
            }
        }
    }
    walk(root, root, &skip_root, &mut out);
    out
}

#[test]
fn overlay_exists_and_default_target_has_no_loom_dependency() {
    let fx = fixture();
    assert!(
        fx.join(OVERLAY_DIR).is_dir(),
        "fixture must carry a `{OVERLAY_DIR}/` overlay at {}",
        fx.display()
    );
    let makefile = fs::read_to_string(fx.join("Makefile")).expect("fixture Makefile");
    // The default target (`all`) must not name loom as a prerequisite — loom is opt-in.
    let all_target = makefile
        .lines()
        .find(|l| l.starts_with("all:"))
        .expect("fixture must declare an `all:` default target");
    assert!(
        !all_target.contains(OVERLAY_DIR),
        "default target must not depend on loom: {all_target:?}"
    );
}

#[test]
fn removing_the_overlay_leaves_every_other_file_byte_identical() {
    let tmp = temp_copy_of_fixture("rm");
    let before = snapshot(&tmp, OVERLAY_DIR);
    fs::remove_dir_all(tmp.join(OVERLAY_DIR)).expect("remove overlay");
    let after = snapshot(&tmp, OVERLAY_DIR);
    assert_eq!(
        before, after,
        "removing `{OVERLAY_DIR}/` must leave every non-overlay file byte-identical"
    );
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn default_pipeline_output_is_identical_with_and_without_the_overlay() {
    let tmp = temp_copy_of_fixture("make");
    let (ok_with, out_with, _) = run_make(&tmp, &["--silent"]);
    fs::remove_dir_all(tmp.join(OVERLAY_DIR)).expect("remove overlay");
    let (ok_without, out_without, _) = run_make(&tmp, &["--silent"]);
    assert!(
        ok_with && ok_without,
        "the default pipeline must succeed with and without the overlay (with={ok_with}, without={ok_without})"
    );
    assert_eq!(
        out_with, out_without,
        "the default pipeline's output must not change when the overlay is removed"
    );
    let _ = fs::remove_dir_all(&tmp);
}
