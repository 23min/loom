//! The umbrella parser — Contract 3 (substrate-agnostic umbrella format) / §4.5 totality.
//!
//! An umbrella is substrate-agnostic markdown carrying a declared `substrate:` field; the formal
//! lowering (e.g. `model.dfy`) is an *attached* artifact, not the source of truth. [`parse`] is
//! **total**: every umbrella file either parses to an [`Umbrella`] or is rejected with a typed
//! [`ParseError`] — none is ever silently misparsed. The parsed form grows additively (subject,
//! intent, …) without weakening that guarantee.

use crate::report::Substrate;
use std::fmt;

/// A parsed umbrella — the machine-load-bearing content of an `umbrella.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Umbrella {
    /// The declared verification substrate.
    pub substrate: Substrate,
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
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse an umbrella's markdown source. Total: returns the [`Umbrella`] or a typed
/// [`ParseError`], never a panic or a silent misparse.
pub(crate) fn parse(content: &str) -> Result<Umbrella, ParseError> {
    let mut token: Option<&str> = None;
    for line in content.lines() {
        if let Some(rest) = line.trim().strip_prefix("substrate:") {
            if token.is_some() {
                return Err(ParseError::Duplicate);
            }
            token = Some(rest.trim());
        }
    }
    let token = token.filter(|t| !t.is_empty()).ok_or(ParseError::Missing)?;
    let substrate =
        Substrate::from_token(token).ok_or_else(|| ParseError::Unknown(token.to_string()))?;
    Ok(Umbrella { substrate })
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
    }
}
