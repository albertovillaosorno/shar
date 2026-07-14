// File:
//   - domain.rs
// Path:
//   - src/rmv/src/domain/domain.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - rmv module behavior for domain.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute domain.
// - Split-When:
//   - Split when domain contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Domain model for movie container identification and hashing.
// - Description:
//   - Defines domain data and behavior for rmv root.
// - Usage:
//   - Used by rmv root code that needs domain.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Domain model for movie evidence, audit records, and conversion decisions.
mod audit;
mod error;
mod format;
mod provenance;
/// Item.
mod runtime_source;
#[cfg(test)]
mod runtime_source_tests;
mod sha256;
mod target;

pub use audit::{AuditReport, MovieRecord};
pub use error::RmvError;
pub use format::MovieKind;
pub use provenance::ProvenanceEvidence;
pub use runtime_source::{
    MovieEvidence, RuntimeCompletionDecision, RuntimeCompletionRule,
    RuntimeMovieCandidate,
};
pub use sha256::Sha256;
pub use target::{CinematicTarget, TargetDecision};
/// Reports whether one logical name can materialize as a Windows component.
pub(crate) fn is_windows_safe_component(value: &str) -> bool {
    if value.is_empty()
        || matches!(
            value,
            "." | ".."
        )
        || value
            .encode_utf16()
            .count()
            > 255
    {
        return false;
    }
    if value.ends_with(
        [
            ' ', '.',
        ],
    ) || value
        .chars()
        .any(
            |character| {
                character.is_control()
                    || character == char::from(92)
                    || matches!(
                        character,
                        '<' | '>' | ':' | '"' | '/' | '|' | '?' | '*'
                    )
            },
        )
    {
        return false;
    }
    let base = value
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    if matches!(
        base.as_str(),
        "CON" | "PRN" | "AUX" | "NUL" | "CLOCK$" | "CONIN$" | "CONOUT$"
    ) {
        return false;
    }
    let Some(number) = base
        .strip_prefix("COM")
        .or_else(|| base.strip_prefix("LPT"))
    else {
        return true;
    };
    !matches!(
        number,
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
    )
}
