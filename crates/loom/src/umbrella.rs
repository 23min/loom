//! The umbrella parser — Contract 3 (substrate-agnostic umbrella format) / §4.5 totality.
//!
//! An umbrella is substrate-agnostic markdown carrying a declared `substrate:` field; the formal
//! lowering (e.g. `model.dfy`) is an *attached* artifact, not the source of truth. [`parse`] is
//! **total**: every umbrella file either parses to an [`Umbrella`] or is rejected with a typed
//! [`ParseError`] — none is ever silently misparsed. The parsed form grows additively (subject,
//! intent, …) without weakening that guarantee.

use crate::report::{Subject, Substrate};
use std::fmt;

/// A parsed umbrella — the machine-load-bearing content of an `umbrella.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Umbrella {
    /// The declared verification substrate.
    pub substrate: Substrate,
    /// The pinned code the property stands for, when the umbrella declares `subject-*` fields.
    /// Optional and additive: a subject-less umbrella (e.g. the M-0016 seed) parses with `None`,
    /// so this grows the parsed form without weakening the totality guarantee.
    pub subject: Option<Subject>,
}

/// Why an umbrella was rejected. A typed error, so a malformed umbrella is never silently
/// dropped or misparsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParseError {
    /// No `substrate:` field was declared (or it had no value).
    Missing,
    /// More than one `substrate:` field — ambiguous, so rejected rather than silently picked.
    Duplicate,
    /// The `substrate:` value is not a substrate loom knows.
    Unknown(String),
    /// A `subject-*` declaration is malformed — declared more than once, or partially declared
    /// (some but not all of the required `subject-repo` / `subject-ref` / `subject-path`).
    /// Additive: the substrate rejections above are unchanged.
    BadSubject(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Missing => {
                write!(f, "umbrella declares no `substrate:` field")
            }
            ParseError::Duplicate => {
                write!(f, "umbrella declares more than one `substrate:` field")
            }
            ParseError::Unknown(token) => {
                write!(f, "unknown substrate {token:?}")
            }
            ParseError::BadSubject(reason) => {
                write!(f, "invalid subject declaration: {reason}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// The unique trimmed value declared for `key:` in the umbrella source, or `None` when `key` is
/// absent or its value is empty. `Err(())` when `key` is declared more than once — the caller
/// maps that to the right typed rejection. A `:` must immediately follow `key`, so distinct keys
/// never collide (`subject-ref` does not match `subject-reference`).
fn unique_field<'a>(content: &'a str, key: &str) -> Result<Option<&'a str>, ()> {
    let mut value: Option<&str> = None;
    for line in content.lines() {
        if let Some(rest) = line
            .trim()
            .strip_prefix(key)
            .and_then(|rest| rest.strip_prefix(':'))
        {
            if value.is_some() {
                return Err(());
            }
            value = Some(rest.trim());
        }
    }
    Ok(value.filter(|v| !v.is_empty()))
}

/// Parse an umbrella's markdown source. Total: returns the [`Umbrella`] or a typed
/// [`ParseError`], never a panic or a silent misparse.
pub(crate) fn parse(content: &str) -> Result<Umbrella, ParseError> {
    let token = unique_field(content, "substrate")
        .map_err(|()| ParseError::Duplicate)?
        .ok_or(ParseError::Missing)?;
    let substrate =
        Substrate::from_token(token).ok_or_else(|| ParseError::Unknown(token.to_string()))?;
    let subject = parse_subject(content)?;
    Ok(Umbrella { substrate, subject })
}

/// Parse the optional `subject-*` binding. Absent entirely → `None`. Present but partial (some
/// but not all required fields) or a field declared twice → a typed [`ParseError::BadSubject`],
/// never a silently half-populated subject (B2 — validated boundary).
fn parse_subject(content: &str) -> Result<Option<Subject>, ParseError> {
    let repo = subject_field(content, "subject-repo")?;
    let reference = subject_field(content, "subject-ref")?;
    let path = subject_field(content, "subject-path")?;
    let symbol = subject_field(content, "subject-symbol")?;

    // No subject declared at all — the optional binding is simply absent.
    if repo.is_none() && reference.is_none() && path.is_none() && symbol.is_none() {
        return Ok(None);
    }

    Ok(Some(Subject {
        repo: require_subject_field(repo, "subject-repo")?.to_string(),
        reference: require_subject_field(reference, "subject-ref")?.to_string(),
        path: require_subject_field(path, "subject-path")?.to_string(),
        symbol: symbol.map(str::to_string),
    }))
}

/// The unique value for a `subject-*` field, mapping a duplicate declaration to a typed
/// [`ParseError::BadSubject`].
fn subject_field<'a>(content: &'a str, key: &str) -> Result<Option<&'a str>, ParseError> {
    unique_field(content, key)
        .map_err(|()| ParseError::BadSubject(format!("`{key}:` declared more than once")))
}

/// Require a `subject-*` field that must be present once any subject field is declared —
/// otherwise a typed [`ParseError::BadSubject`], never a half-populated subject (B2).
fn require_subject_field<'a>(value: Option<&'a str>, key: &str) -> Result<&'a str, ParseError> {
    value.ok_or_else(|| {
        ParseError::BadSubject(format!("`{key}:` is required when a subject is declared"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn seed_umbrella(property: &str) -> String {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap() // crates/
            .parent()
            .unwrap() // workspace root
            .join("examples/seed-downstream/loom")
            .join(property)
            .join("umbrella.md");
        std::fs::read_to_string(path).expect("read seed umbrella")
    }

    #[test]
    fn the_seed_corpus_parses_to_dafny() {
        for property in [
            "fsm-terminality",
            "cancel-edge-legality",
            "archive-terminality",
        ] {
            let umbrella = parse(&seed_umbrella(property))
                .unwrap_or_else(|e| panic!("seed umbrella {property} should parse: {e}"));
            assert_eq!(umbrella.substrate, Substrate::Dafny);
        }
    }

    #[test]
    fn a_minimal_wellformed_umbrella_parses() {
        let src = "# Umbrella — x\n\nsubstrate: dafny\n\n## intent\n...\n";
        assert_eq!(parse(src).unwrap().substrate, Substrate::Dafny);
    }

    #[test]
    fn missing_substrate_is_rejected() {
        assert_eq!(
            parse("# no substrate here\n").unwrap_err(),
            ParseError::Missing
        );
        assert_eq!(parse("").unwrap_err(), ParseError::Missing);
        assert_eq!(parse("substrate:\n").unwrap_err(), ParseError::Missing);
    }

    #[test]
    fn unknown_substrate_is_rejected_with_the_token() {
        assert_eq!(
            parse("substrate: coq\n").unwrap_err(),
            ParseError::Unknown("coq".to_string())
        );
    }

    #[test]
    fn duplicate_substrate_is_rejected() {
        assert_eq!(
            parse("substrate: dafny\nsubstrate: dafny\n").unwrap_err(),
            ParseError::Duplicate
        );
        // Duplicate is detected before the empty-value filter: a second `substrate:` line rejects
        // even when its value is empty. Pins the frozen ordering the `unique_field` refactor had
        // to preserve (the equivalent is proved by reference in parser-totality/model.dfy).
        assert_eq!(
            parse("substrate: dafny\nsubstrate:\n").unwrap_err(),
            ParseError::Duplicate
        );
    }

    #[test]
    fn a_subject_less_umbrella_parses_with_no_subject() {
        // The M-0016 seed shape: substrate declared, no `subject-*` fields — subject is absent,
        // not an error (additive, backward-compatible).
        let u = parse("substrate: dafny\n").unwrap();
        assert_eq!(u.substrate, Substrate::Dafny);
        assert_eq!(u.subject, None);
    }

    #[test]
    fn a_fully_declared_subject_is_parsed() {
        let src = "substrate: dafny\n\
                   subject-repo: loom\n\
                   subject-ref: v0.1.0\n\
                   subject-path: crates/loom/src/backend.rs\n\
                   subject-symbol: loom::backend::dispatch\n";
        let subject = parse(src).unwrap().subject.expect("subject populated");
        assert_eq!(subject.repo, "loom");
        assert_eq!(subject.reference, "v0.1.0");
        assert_eq!(subject.path, "crates/loom/src/backend.rs");
        assert_eq!(subject.symbol.as_deref(), Some("loom::backend::dispatch"));
    }

    #[test]
    fn a_subject_without_a_symbol_is_allowed() {
        // symbol is optional — a file-level subject (no narrower symbol) is legal.
        let src = "substrate: dafny\n\
                   subject-repo: aiwf\n\
                   subject-ref: v0.20.0\n\
                   subject-path: internal/entity/transition.go\n";
        let subject = parse(src).unwrap().subject.expect("subject populated");
        assert_eq!(subject.symbol, None);
    }

    #[test]
    fn a_partial_subject_is_rejected() {
        // A subject declared without its required fields is a typed rejection, never a silently
        // half-populated subject.
        let src = "substrate: dafny\nsubject-symbol: loom::backend::dispatch\n";
        match parse(src).unwrap_err() {
            ParseError::BadSubject(msg) => assert!(msg.contains("subject-repo")),
            other => panic!("expected BadSubject, got {other:?}"),
        }
    }

    #[test]
    fn a_duplicate_subject_field_is_rejected() {
        let src = "substrate: dafny\n\
                   subject-repo: loom\n\
                   subject-repo: aiwf\n\
                   subject-ref: v0.1.0\n\
                   subject-path: crates/loom/src/backend.rs\n";
        match parse(src).unwrap_err() {
            ParseError::BadSubject(msg) => assert!(msg.contains("declared more than once")),
            other => panic!("expected BadSubject, got {other:?}"),
        }
    }

    #[test]
    fn bad_subject_renders_a_self_explaining_message() {
        // The runner renders this Display into a parse-error report's rationale — pin the arm.
        let rendered = ParseError::BadSubject("`subject-ref:` is required".to_string()).to_string();
        assert!(
            rendered.contains("invalid subject declaration") && rendered.contains("subject-ref"),
            "operator-facing message must name the problem: {rendered:?}"
        );
    }
}
