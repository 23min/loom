//! The `loom verify` runner — Contract 4, the runner interface.
//!
//! Discovers each property in an overlay directory (a subdirectory carrying an
//! [`UMBRELLA_FILE`](crate::UMBRELLA_FILE)) and writes a gap report beside it
//! ([`REPORT_FILE`](crate::REPORT_FILE)). The report shape is the frozen
//! [`GapReport`](crate::report::GapReport) schema (M-0016/AC-3); the runner emits a
//! discovery-stage [`pending`](crate::report::GapReport::pending) report, which richens as the
//! umbrella parse (AC-5) and verification (AC-6) land. The write is made atomic and
//! reproducible in AC-4.

use crate::{REPORT_FILE, UMBRELLA_FILE};
use std::io;
use std::path::{Path, PathBuf};

/// One discovered property: its id (the subdirectory name) and its directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    pub id: String,
    pub dir: PathBuf,
}

/// A written gap report: which property it covers and where it landed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrittenReport {
    pub property: String,
    pub path: PathBuf,
}

/// Discover every property under `overlay_dir`: an immediate subdirectory carrying an
/// [`UMBRELLA_FILE`](crate::UMBRELLA_FILE). Sorted by id so runs are deterministic (G1).
pub fn discover_properties(overlay_dir: &Path) -> io::Result<Vec<Property>> {
    let mut props = Vec::new();
    for entry in std::fs::read_dir(overlay_dir)? {
        let path = entry?.path();
        if path.is_dir() && path.join(UMBRELLA_FILE).is_file() {
            let id = path
                .file_name()
                .expect("directory entry has a name")
                .to_string_lossy()
                .into_owned();
            props.push(Property { id, dir: path });
        }
    }
    props.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(props)
}

/// Run `loom verify` over `overlay_dir`. When `prop` is `Some(id)`, scope to that one
/// property and error if it is not found; when `None`, process every discovered property.
/// Writes one gap report per selected property and returns what was written.
pub fn verify(overlay_dir: &Path, prop: Option<&str>) -> io::Result<Vec<WrittenReport>> {
    let mut selected = discover_properties(overlay_dir)?;
    if let Some(want) = prop {
        selected.retain(|p| p.id == want);
        if selected.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("no property named {want:?} under {}", overlay_dir.display()),
            ));
        }
    }
    let mut written = Vec::with_capacity(selected.len());
    for p in selected {
        let path = p.dir.join(REPORT_FILE);
        let report = report_for(&p)?;
        crate::atomic::write_atomic(&path, &report.to_canonical_json())?;
        written.push(WrittenReport {
            property: p.id,
            path,
        });
    }
    Ok(written)
}

/// Build the report for one property: parse its umbrella, route it to a backend, and run the
/// backend to a graded verdict — or, if the umbrella is malformed, emit an explicit error report.
/// A parse failure is *recorded*, never a silent skip (M-0016/AC-5 totality); the verdict, gaps,
/// and audit come from the backend run (AC-6).
fn report_for(p: &Property) -> io::Result<crate::report::GapReport> {
    use crate::report::{Audit, GapReport};
    let source = std::fs::read_to_string(p.dir.join(UMBRELLA_FILE))?;
    Ok(match crate::umbrella::parse(&source) {
        Ok(umbrella) => {
            let backend = crate::backend::dispatch(umbrella.substrate);
            let outcome = crate::backend::run(backend, &p.dir, crate::backend::VERIFY_TIMEOUT);
            let audit = Audit {
                checked: format!("umbrella claim for {} via {} verify", p.id, backend.name()),
                inputs: backend.inputs().iter().map(|s| s.to_string()).collect(),
                rationale: outcome.rationale,
            };
            GapReport::verified(
                &p.id,
                umbrella.subject,
                umbrella.substrate,
                outcome.verdict,
                outcome.gaps,
                audit,
            )
        }
        Err(e) => GapReport::parse_error(&p.id, &e.to_string()),
    })
}

#[cfg(test)]
mod tests {
    //! Direct pins on the runner interface (Contract 4), at the library seam. These assert
    //! *behavior* — a report exists per selected property, scoping, and the not-found error
    //! — never the report's byte content, which AC-3 replaces with the frozen schema.
    use super::*;
    use std::fs;

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("loom-runner-{}-{}", std::process::id(), tag));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).expect("mkdir scratch");
        d
    }

    fn make_property(overlay: &Path, id: &str) {
        let d = overlay.join(id);
        fs::create_dir_all(&d).expect("mkdir property");
        fs::write(d.join(UMBRELLA_FILE), "# stub umbrella\n").expect("write umbrella");
    }

    #[test]
    fn discover_finds_property_dirs_and_skips_non_properties() {
        let ov = scratch("discover");
        make_property(&ov, "alpha");
        make_property(&ov, "beta");
        fs::write(ov.join("README.md"), "not a property\n").expect("write file"); // non-dir → skipped
        fs::create_dir_all(ov.join("notes")).expect("mkdir"); // dir without umbrella → skipped
        let ids: Vec<String> = discover_properties(&ov)
            .unwrap()
            .into_iter()
            .map(|p| p.id)
            .collect();
        assert_eq!(ids, vec!["alpha".to_string(), "beta".to_string()]);
        let _ = fs::remove_dir_all(&ov);
    }

    #[test]
    fn verify_all_writes_a_report_per_property() {
        let ov = scratch("all");
        make_property(&ov, "alpha");
        make_property(&ov, "beta");
        let written = verify(&ov, None).unwrap();
        assert_eq!(written.len(), 2);
        for id in ["alpha", "beta"] {
            assert!(ov.join(id).join(REPORT_FILE).is_file());
        }
        let _ = fs::remove_dir_all(&ov);
    }

    #[test]
    fn verify_scoped_writes_only_the_named_property() {
        let ov = scratch("scoped");
        make_property(&ov, "alpha");
        make_property(&ov, "beta");
        let written = verify(&ov, Some("alpha")).unwrap();
        assert_eq!(written.len(), 1);
        assert_eq!(written[0].property, "alpha");
        assert!(ov.join("alpha").join(REPORT_FILE).is_file());
        assert!(!ov.join("beta").join(REPORT_FILE).exists());
        let _ = fs::remove_dir_all(&ov);
    }

    #[test]
    fn verify_unknown_property_errors_without_writing() {
        let ov = scratch("unknown");
        make_property(&ov, "alpha");
        let err = verify(&ov, Some("ghost")).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        assert!(!ov.join("alpha").join(REPORT_FILE).exists());
        let _ = fs::remove_dir_all(&ov);
    }
}
