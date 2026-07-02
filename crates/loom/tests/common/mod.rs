//! Shared fixture helpers for the M-0016 integration tests.
//!
//! The seed represents a downstream repo via the fixture at `examples/seed-downstream/`.
//! Each test binary that needs the fixture includes this module with `mod common;`; the
//! `dead_code` allow covers helpers a given binary happens not to use.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The fixture downstream repo (a stand-in for aiwf-with-loom-adopted).
pub fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap() // crates/
        .parent()
        .unwrap() // workspace root
        .join("examples/seed-downstream")
}

/// The freshly-built `loom` binary Cargo produces for the integration tests.
pub fn loom_bin() -> String {
    env!("CARGO_BIN_EXE_loom").to_string()
}

/// Recursively copy `src` into `dst`.
pub fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("mkdir dst");
    for entry in fs::read_dir(src).expect("read_dir src") {
        let p = entry.expect("entry").path();
        let target = dst.join(p.file_name().unwrap());
        if p.is_dir() {
            copy_dir(&p, &target);
        } else {
            fs::copy(&p, &target).expect("copy file");
        }
    }
}

/// Copy the fixture into a unique temp directory so a test can mutate it freely.
pub fn temp_copy_of_fixture(tag: &str) -> PathBuf {
    let tmp = std::env::temp_dir().join(format!("loom-fx-{}-{}", std::process::id(), tag));
    let _ = fs::remove_dir_all(&tmp);
    copy_dir(&fixture(), &tmp);
    tmp
}

/// Run `make <args>` in `dir`; return (success, stdout, stderr).
pub fn run_make(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let out = Command::new("make")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("invoke make");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// An independent scan of the overlay for property ids — a subdirectory carrying an
/// `umbrella.md`. Deliberately hand-rolled (not the runner's own discovery) so tests
/// check the runner against an independent oracle. Sorted for determinism.
pub fn discovered_properties(overlay_dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for entry in fs::read_dir(overlay_dir).expect("read overlay") {
        let p = entry.expect("entry").path();
        if p.is_dir() && p.join("umbrella.md").is_file() {
            out.push(p.file_name().unwrap().to_string_lossy().into_owned());
        }
    }
    out.sort();
    out
}
