// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Domain domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Domain domain module.
// - Description:
//   - Implements the declared domain module responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain domain module.

mod audit;
mod error;
mod escaped_path;
mod format;
mod provenance;
/// Item.
mod runtime_source;
#[cfg(test)]
#[path = "../../../../tests/formats/rmv/unit/domain/runtime_source_tests.rs"]
mod runtime_source_tests;
mod sha256;
mod target;

pub use audit::{AuditReport, MovieRecord};
pub use error::{IoFailure, RmvError};
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
        || matches!(value, "." | "..")
        || value.encode_utf16().count() > 255
    {
        return false;
    }
    if value.ends_with([' ', '.'])
        || value.chars().any(|character| {
            character.is_control()
                || character == char::from(92)
                || matches!(
                    character,
                    '<' | '>' | ':' | '"' | '/' | '|' | '?' | '*'
                )
        })
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
