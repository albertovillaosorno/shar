// File:
//   - adapters.rs
// Path:
//   - src/rcf/src/adapters.rs
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
//   - rcf module behavior for adapters.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute adapters.
// - Split-When:
//   - Split when adapters contains two independently testable contracts.
// - Merge-When:
//   - Another rcf module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Filesystem adapters for RCF archive extraction.
// - Description:
//   - Defines adapters data and behavior for rcf root.
// - Usage:
//   - Used by rcf root code that needs adapters.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Inbound and outbound adapters for RCF archive workflows.
//!
//! Driving adapters translate operator requests into application use cases,
//! while driven adapters implement the storage ports selected by composition.
/// Outbound adapters called through RCF ports.
pub mod driven;
/// Inbound adapters such as command-line request handling.
pub mod driving;

pub use driven::{FileArchiveSource, FileEntrySink};
