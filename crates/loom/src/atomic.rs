//! Atomic, crash-safe file writes — C3.
//!
//! A report is written to a temporary file in the destination's *own* directory, then
//! atomically renamed into place. A crash between the two steps leaves the destination
//! fully-old (or absent, if it never existed) — never a partial or corrupt report. The temp
//! is a sibling of the destination so both are on one filesystem, which makes `rename`
//! atomic on POSIX.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Disambiguates concurrent temp files. Not part of any report's content, so it does not
/// affect output reproducibility (G1).
static COUNTER: AtomicU64 = AtomicU64::new(0);

/// A staged write: `contents` are on disk in a sibling temp file, not yet visible at `dest`.
/// Dropping without [`commit`](Staged::commit) models a crash — `dest` stays untouched.
#[derive(Debug)]
struct Staged {
    tmp: PathBuf,
    dest: PathBuf,
}

impl Staged {
    /// Atomically move the staged contents into place, cleaning up the temp on failure.
    fn commit(self) -> io::Result<()> {
        match std::fs::rename(&self.tmp, &self.dest) {
            Ok(()) => Ok(()),
            Err(e) => {
                let _ = std::fs::remove_file(&self.tmp);
                Err(e)
            }
        }
    }
}

/// Write `contents` to a sibling temp file of `dest`, not yet visible at `dest`.
fn stage(dest: &Path, contents: &str) -> io::Result<Staged> {
    let file_name = dest
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "destination has no file name"))?
        .to_string_lossy();
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let tmp = dest.with_file_name(format!(".{file_name}.tmp.{}.{n}", std::process::id()));
    std::fs::write(&tmp, contents)?;
    Ok(Staged {
        tmp,
        dest: dest.to_path_buf(),
    })
}

/// Atomically write `contents` to `dest`: stage to a sibling temp, then rename into place.
pub(crate) fn write_atomic(dest: &Path, contents: &str) -> io::Result<()> {
    stage(dest, contents)?.commit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("loom-atomic-{}-{}", std::process::id(), tag));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).expect("mkdir scratch");
        d
    }

    #[test]
    fn stage_without_commit_leaves_a_prior_report_intact() {
        let dir = scratch("intact");
        let dest = dir.join("report.json");
        write_atomic(&dest, "OLD\n").unwrap();
        let staged = stage(&dest, "NEW\n").unwrap(); // temp written; commit withheld = crash
        assert_eq!(
            fs::read_to_string(&dest).unwrap(),
            "OLD\n",
            "dest must stay fully-old before commit"
        );
        drop(staged); // crash: never commit
        assert_eq!(
            fs::read_to_string(&dest).unwrap(),
            "OLD\n",
            "a withheld commit must not corrupt the prior report"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn stage_without_commit_leaves_no_report_when_none_existed() {
        let dir = scratch("absent");
        let dest = dir.join("report.json");
        let staged = stage(&dest, "NEW\n").unwrap();
        assert!(
            !dest.exists(),
            "dest must be absent until commit — never partial"
        );
        drop(staged);
        assert!(!dest.exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn commit_replaces_fully_new() {
        let dir = scratch("new");
        let dest = dir.join("report.json");
        write_atomic(&dest, "OLD\n").unwrap();
        write_atomic(&dest, "NEW\n").unwrap();
        assert_eq!(
            fs::read_to_string(&dest).unwrap(),
            "NEW\n",
            "a committed write is fully-new"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn stage_rejects_a_destination_with_no_file_name() {
        let err = stage(Path::new("/"), "x").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn commit_errors_and_cleans_up_when_the_temp_vanished() {
        let dir = scratch("vanished");
        let dest = dir.join("report.json");
        let staged = stage(&dest, "NEW\n").unwrap();
        fs::remove_file(&staged.tmp).unwrap(); // the temp disappears before commit
        assert!(
            staged.commit().is_err(),
            "commit must surface the rename failure"
        );
        assert!(
            !dest.exists(),
            "a failed commit must not create the destination"
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
