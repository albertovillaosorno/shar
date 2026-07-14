// File:
//   - matching.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text/matching.rs
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
//   - The matching contract for pipeline phase two minor units language
//   - text.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute matching.
// - Split-When:
//   - Split when matching contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Matching for pipeline phase two minor units language text.
// - Description:
//   - Defines matching data and behavior for pipeline phase two minor units
//   - language text.
// - Usage:
//   - Used by pipeline phase two minor units language text code that needs
//   - matching.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Matching for pipeline phase two minor units language text.
//!
//! This boundary keeps matching for pipeline phase two minor units language
//! text explicit and returns deterministic results to pipeline callers.
/// Supports the `has_any` operation within this deterministic classification
/// boundary.
pub(super) fn has_any(
    value: &str,
    needles: &[&str],
) -> bool {
    needles
        .iter()
        .any(|needle| value.contains(needle))
}
