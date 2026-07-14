// File:
//   - domain.rs
// Path:
//   - src/rcf/src/domain/domain.rs
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
//   - rcf module behavior for domain.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute domain.
// - Split-When:
//   - Split when domain contains two independently testable contracts.
// - Merge-When:
//   - Another rcf module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Pure RCF archive domain model.
// - Description:
//   - Defines domain data and behavior for rcf root.
// - Usage:
//   - Used by rcf root code that needs domain.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Pure RCF archive domain model.
//!
//! This boundary keeps pure rcf archive domain model explicit and returns
//! deterministic results to rcf callers.
mod failure;
pub(crate) mod path_policy;
mod records;

pub use failure::ArchiveError;
pub use records::{Archive, ArchiveEntry, ArchiveHeader, IndexRecord};
