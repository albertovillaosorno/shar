// File:
//   - application.rs
// Path:
//   - src/rcf/src/application.rs
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
//   - rcf module behavior for application.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute application.
// - Split-When:
//   - Split when application contains two independently testable contracts.
// - Merge-When:
//   - Another rcf module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Application use cases for RCF archives.
// - Description:
//   - Defines application data and behavior for rcf root.
// - Usage:
//   - Used by rcf root code that needs application.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application use cases for RCF archives.
//!
//! This boundary keeps application use cases for rcf archives explicit and
//! returns deterministic results to rcf callers.
pub mod extract;
pub mod parse;

pub use extract::{ExtractionReport, Extractor};
pub use parse::{ArchiveParser, ListArchive};
