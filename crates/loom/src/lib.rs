//! loom-light — an opt-in, contained verification-overlay runner.
//!
//! The engine reads a downstream repo's [`OVERLAY_DIR`] overlay, verifies each property's
//! umbrella against its lowering, and writes gap reports. loom generates no target code
//! (ADR-0017); code generation, where wanted, is the LLM's role.
//!
//! The five frozen contracts (E-0005 / M-0016) are established behind this crate's API and
//! do not move; substrates, authoring, and recognition grow additively.

mod atomic;
mod backend;
pub mod report;
pub mod runner;
mod umbrella;

/// The single directory that holds a downstream repo's entire loom footprint.
///
/// Containment contract (M-0016/AC-1): everything loom-related lives under this directory,
/// so removing it leaves the host repo byte-identical.
pub const OVERLAY_DIR: &str = "loom";

/// The umbrella file that marks a subdirectory of the overlay as a property (M-0016/AC-2).
///
/// A property is any immediate subdirectory of [`OVERLAY_DIR`] carrying this file; the
/// runner discovers properties by its presence.
pub const UMBRELLA_FILE: &str = "umbrella.md";

/// The generated gap report written beside each property's umbrella (M-0016/AC-2).
///
/// One report per property, in the property's own subdirectory (so it lives under
/// [`OVERLAY_DIR`] and disappears with the overlay). The report *schema* is frozen in
/// M-0016/AC-3; the write is made atomic and reproducible in AC-4.
pub const REPORT_FILE: &str = "report.json";
