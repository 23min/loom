//! The gap report — Contract 2, loom's visible cross-process artifact.
//!
//! **Types-first** (E-0005 decision): these serde types are the single source of truth for
//! the report shape (C1). The published JSON Schema at [`SCHEMA_RELPATH`] is *generated*
//! from them (schemars) and checked in; the freeze test asserts the two agree, so they
//! cannot silently drift. `#[serde(deny_unknown_fields)]` makes strict deserialization
//! validate-on-read (B2). The runtime writer needs only serde/serde_json; schema generation
//! (schemars) and emitted-report validation (jsonschema) are test-time tooling.
//!
//! The shape is frozen once here and grows only additively under a new [`SCHEMA_VERSION`].
//! At M-0016/AC-3 the runner emits [`GapReport::pending`]; `subject`/`substrate` fill in when
//! the umbrella is parsed (AC-5) and `verdict`/`gaps`/`audit` when verification runs (AC-6) —
//! no schema change required.

use serde::{Deserialize, Serialize};

/// The frozen schema version this build emits. Bump only alongside a schema change and a new
/// checked-in `schema/gap-report.v<N>.schema.json`.
pub const SCHEMA_VERSION: &str = "1";

/// The checked-in published schema, relative to the crate manifest dir. The cross-language
/// contract a non-Rust consumer (e.g. aiwf's future Go reader) validates against.
pub const SCHEMA_RELPATH: &str = "schema/gap-report.v1.schema.json";

/// A gap report: one property's graded verification outcome, written to `<prop>/report.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct GapReport {
    /// The frozen contract version (see [`SCHEMA_VERSION`]).
    pub schema_version: String,
    /// The property id — the overlay subdirectory name.
    pub property: String,
    /// The graded outcome of verification.
    pub verdict: Verdict,
    /// The code under scrutiny, referenced by version — populated once the umbrella is parsed
    /// (AC-5); `null` at the discovery/pending stage.
    #[serde(default)]
    pub subject: Option<Subject>,
    /// The verification substrate (backend) — populated once known; `null` when pending.
    #[serde(default)]
    pub substrate: Option<Substrate>,
    /// Findings. Empty for a clean full proof; the at-risk property surfaces a `code:"B"` gap
    /// once verified (AC-6).
    pub gaps: Vec<Gap>,
    /// The audit trail: what was checked, the inputs seen, and why the verdict went that way.
    pub audit: Audit,
}

/// The graded verdict: the rung verification reached, plus operational states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    /// Full proof — the property holds unconditionally (top rung).
    Proved,
    /// Bounded proof — the property holds within explicit bounds.
    Bounded,
    /// Supported by concrete examples only — no proof (bottom rung).
    Examples,
    /// Verification found the property does not hold, or found a hole (a gap is recorded).
    Refuted,
    /// Not yet verified — the schema/seam stage before the backend runs.
    Pending,
    /// The verifier failed to produce a verdict (its nondeterminism isolated + surfaced, G1).
    Error,
}

/// The verification substrate — the backend that produced the verdict. Grows additively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum Substrate {
    /// Dafny + Z3.
    Dafny,
}

impl Substrate {
    /// Every substrate loom knows. Hand-maintained; the `substrate_all_is_complete` tripwire
    /// test forces this list to be updated when a variant is added.
    pub const ALL: &'static [Substrate] = &[Substrate::Dafny];

    /// Parse a `substrate:` token using the same vocabulary serde emits — the single source of
    /// the wire spelling (C1). An unknown token yields `None` (rejected, never silently
    /// misparsed).
    pub fn from_token(token: &str) -> Option<Substrate> {
        serde_json::from_value(serde_json::Value::String(token.to_string())).ok()
    }
}

/// The code under scrutiny, referenced by version — never read at verify time (G1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Subject {
    /// The host repository the subject lives in (e.g. `"aiwf"`).
    pub repo: String,
    /// The version the subject is pinned at — a tag/ref, referenced not read (G1).
    #[serde(rename = "ref")]
    pub reference: String,
    /// The path to the file under scrutiny, within `repo` at `ref`.
    pub path: String,
    /// The specific symbol under scrutiny, if narrower than the whole file.
    #[serde(default)]
    pub symbol: Option<String>,
}

/// One finding: a gap between the intended property and what the code guarantees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Gap {
    /// The gap class code (loom's gap taxonomy, e.g. `"B"`).
    pub code: String,
    /// A one-line human summary of the gap.
    pub summary: String,
    /// Optional detail — a counterexample, the unproven obligation, etc.
    #[serde(default)]
    pub detail: Option<String>,
}

/// The audit trail behind a verdict (E3 / G3 — observable, reproducible reasoning).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Audit {
    /// What was checked — the claim/obligation the verdict is about.
    pub checked: String,
    /// The inputs the verdict saw (e.g. the lowering file). Order-stable (G1).
    pub inputs: Vec<String>,
    /// Why the verdict went the way it did.
    pub rationale: String,
}

impl GapReport {
    /// A discovery-stage report for a property whose umbrella has not been parsed and whose
    /// verification has not run — the honest state at M-0016/AC-3. `subject`, `substrate`, and
    /// `verdict` richen in AC-5/AC-6 without a schema change.
    pub fn pending(property: &str) -> Self {
        GapReport {
            schema_version: SCHEMA_VERSION.to_string(),
            property: property.to_string(),
            verdict: Verdict::Pending,
            subject: None,
            substrate: None,
            gaps: Vec::new(),
            audit: Audit {
                checked: "property discovered in the overlay".to_string(),
                inputs: Vec::new(),
                rationale: "M-0016/AC-3: schema frozen; umbrella parse (AC-5) and verification \
                            (AC-6) pending"
                    .to_string(),
            },
        }
    }

    /// A parsed property that was routed to its backend and verified — `substrate` populated and
    /// the graded `verdict`, `gaps`, and `audit` filled from the backend run (M-0016/AC-6). The
    /// `subject` is the pinned code the umbrella declared, or `None` when it declared none
    /// (M-0017/AC-5).
    pub fn verified(
        property: &str,
        subject: Option<Subject>,
        substrate: Substrate,
        verdict: Verdict,
        gaps: Vec<Gap>,
        audit: Audit,
    ) -> Self {
        GapReport {
            schema_version: SCHEMA_VERSION.to_string(),
            property: property.to_string(),
            verdict,
            subject,
            substrate: Some(substrate),
            gaps,
            audit,
        }
    }

    /// A property whose umbrella could not be parsed — an explicit error report, never a silent
    /// skip (M-0016/AC-5 totality). The parse failure is recorded as a gap.
    pub fn parse_error(property: &str, message: &str) -> Self {
        GapReport {
            schema_version: SCHEMA_VERSION.to_string(),
            property: property.to_string(),
            verdict: Verdict::Error,
            subject: None,
            substrate: None,
            gaps: vec![Gap {
                code: "parse".to_string(),
                summary: "umbrella could not be parsed".to_string(),
                detail: Some(message.to_string()),
            }],
            audit: Audit {
                checked: "umbrella parse".to_string(),
                inputs: Vec::new(),
                rationale: format!("umbrella rejected: {message}"),
            },
        }
    }

    /// Serialize to the canonical on-disk form: pretty JSON with a trailing newline,
    /// deterministic (G1 — no time, order-stable). This exact form is what the runner writes.
    pub fn to_canonical_json(&self) -> String {
        let mut s = serde_json::to_string_pretty(self).expect("GapReport serializes");
        s.push('\n');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn schema_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(SCHEMA_RELPATH)
    }

    /// The schema schemars generates from the types, in canonical on-disk form.
    fn generated_schema_json() -> String {
        let schema = schemars::schema_for!(GapReport);
        let mut s = serde_json::to_string_pretty(&schema).expect("schema serializes");
        s.push('\n');
        s
    }

    /// A spread of reports exercising the full schema: pending, a clean proof with a subject,
    /// and the at-risk refuted case with a `(B)` gap.
    fn sample_reports() -> Vec<GapReport> {
        vec![
            GapReport::pending("archive-terminality"),
            GapReport {
                schema_version: SCHEMA_VERSION.to_string(),
                property: "fsm-terminality".to_string(),
                verdict: Verdict::Proved,
                subject: Some(Subject {
                    repo: "aiwf".to_string(),
                    reference: "v0.20.0".to_string(),
                    path: "internal/entity/transition.go".to_string(),
                    symbol: None,
                }),
                substrate: Some(Substrate::Dafny),
                gaps: Vec::new(),
                audit: Audit {
                    checked: "no transition leaves a terminal status, for every kind".to_string(),
                    inputs: vec!["fsm-terminality/model.dfy".to_string()],
                    rationale: "Dafny discharged the absorbing-terminal obligation".to_string(),
                },
            },
            GapReport {
                schema_version: SCHEMA_VERSION.to_string(),
                property: "cancel-edge-legality".to_string(),
                verdict: Verdict::Refuted,
                subject: Some(Subject {
                    repo: "aiwf".to_string(),
                    reference: "v0.20.0".to_string(),
                    path: "internal/entity/transition.go".to_string(),
                    symbol: Some("CancelTarget".to_string()),
                }),
                substrate: Some(Substrate::Dafny),
                gaps: vec![Gap {
                    code: "B".to_string(),
                    summary: "cancel may route an FSM-illegal from->target edge".to_string(),
                    detail: Some("nothing downstream re-checks the edge legality".to_string()),
                }],
                audit: Audit {
                    checked: "CancelTarget(k,s) is empty or a legal edge, for all non-terminal s"
                        .to_string(),
                    inputs: vec!["cancel-edge-legality/model.dfy".to_string()],
                    rationale: "counterexample found: the edge-legality obligation does not hold"
                        .to_string(),
                },
            },
            // The remaining rungs of the verdict ladder, so the D2 seam validates every wire
            // value — a serde<->schemars drift on an un-sampled variant would otherwise slip.
            GapReport {
                schema_version: SCHEMA_VERSION.to_string(),
                property: "bounded-example".to_string(),
                verdict: Verdict::Bounded,
                subject: None,
                substrate: Some(Substrate::Dafny),
                gaps: Vec::new(),
                audit: Audit {
                    checked: "the property holds for all inputs within the declared bounds"
                        .to_string(),
                    inputs: vec!["bounded-example/model.dfy".to_string()],
                    rationale: "dafny discharged the obligation under the stated bounds"
                        .to_string(),
                },
            },
            GapReport {
                schema_version: SCHEMA_VERSION.to_string(),
                property: "examples-only".to_string(),
                verdict: Verdict::Examples,
                subject: None,
                substrate: Some(Substrate::Dafny),
                gaps: Vec::new(),
                audit: Audit {
                    checked: "the property is supported by concrete examples only".to_string(),
                    inputs: vec!["examples-only/model.dfy".to_string()],
                    rationale: "no proof attempted; witnesses recorded".to_string(),
                },
            },
            GapReport {
                schema_version: SCHEMA_VERSION.to_string(),
                property: "verifier-error".to_string(),
                verdict: Verdict::Error,
                subject: None,
                substrate: Some(Substrate::Dafny),
                gaps: Vec::new(),
                audit: Audit {
                    checked: "the verifier failed to produce a verdict".to_string(),
                    inputs: vec!["verifier-error/model.dfy".to_string()],
                    rationale:
                        "dafny gave up (out of resource); nondeterminism surfaced, not a proof"
                            .to_string(),
                },
            },
        ]
    }

    #[test]
    fn schema_version_matches_the_published_schema_path() {
        // A version bump must move both constants together: the published filename carries the
        // version and a cross-process reader dispatches on SCHEMA_VERSION. If they drift, a bump
        // could update one and leave an old reader loading a new report against a stale schema.
        assert!(
            SCHEMA_RELPATH.contains(&format!("v{SCHEMA_VERSION}.")),
            "SCHEMA_RELPATH ({SCHEMA_RELPATH}) must carry v{SCHEMA_VERSION} to match SCHEMA_VERSION"
        );
    }

    #[test]
    fn schema_is_frozen() {
        // The checked-in schema must equal the schema schemars generates from the types. A
        // struct change regenerates a different schema -> this fails -> re-freeze with
        // UPDATE_SCHEMA=1 and bump SCHEMA_VERSION if the change is incompatible.
        let generated = generated_schema_json();
        let path = schema_path();
        if std::env::var_os("UPDATE_SCHEMA").is_some() {
            std::fs::create_dir_all(path.parent().unwrap()).expect("mkdir schema dir");
            std::fs::write(&path, &generated).expect("write schema");
        }
        let checked_in = std::fs::read_to_string(&path).unwrap_or_else(|_| {
            panic!(
                "checked-in schema missing at {}; run `UPDATE_SCHEMA=1 cargo test` to create it",
                path.display()
            )
        });
        assert_eq!(
            generated, checked_in,
            "gap-report schema drifted from the types; re-freeze with UPDATE_SCHEMA=1"
        );
    }

    #[test]
    fn emitted_reports_validate_against_the_frozen_schema() {
        // Serde output must satisfy the schemars-generated schema — catches serde<->schemars
        // representation drift, the real bug a schema-first consumer would hit.
        let schema: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(schema_path()).expect("read schema"))
                .expect("schema parses");
        let validator = jsonschema::validator_for(&schema).expect("compile schema");
        for report in sample_reports() {
            let instance = serde_json::to_value(&report).expect("report -> value");
            let errors: Vec<String> = validator
                .iter_errors(&instance)
                .map(|e| e.to_string())
                .collect();
            assert!(
                errors.is_empty(),
                "report {:?} failed schema validation: {errors:?}",
                report.property
            );
        }
    }

    #[test]
    fn unknown_fields_are_rejected_on_read() {
        // deny_unknown_fields is loom's validate-on-read (B2): a report with an extra key is
        // refused rather than silently misread.
        let mut value = serde_json::to_value(GapReport::pending("p")).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("surprise".to_string(), serde_json::json!(1));
        let bytes = serde_json::to_string(&value).unwrap();
        assert!(
            serde_json::from_str::<GapReport>(&bytes).is_err(),
            "an unknown field must be rejected on read"
        );
    }

    #[test]
    fn reports_round_trip_through_serde() {
        for report in sample_reports() {
            let json = report.to_canonical_json();
            let back: GapReport = serde_json::from_str(&json).expect("round-trips");
            assert_eq!(report, back);
        }
    }

    #[test]
    fn substrate_all_is_complete() {
        // A tripwire: the exhaustive match forces a new arm when a Substrate variant is added,
        // and the membership assert forces that variant into ALL.
        fn assert_listed(s: Substrate) {
            // Exhaustive match: a new variant forces an arm here (and thus into ALL below).
            match s {
                Substrate::Dafny => {}
            }
            assert!(
                Substrate::ALL.contains(&s),
                "Substrate::ALL is missing {s:?}"
            );
        }
        assert_listed(Substrate::Dafny);
    }

    #[test]
    fn from_token_matches_the_serde_vocabulary() {
        assert_eq!(Substrate::from_token("dafny"), Some(Substrate::Dafny));
        assert!(Substrate::from_token("coq").is_none());
        assert!(Substrate::from_token("").is_none());
    }
}
