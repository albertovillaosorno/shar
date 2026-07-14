// File:
//   - adapters.rs
// Path:
//   - src/rmv/src/adapters.rs
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
//   - rmv module behavior for adapters.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute adapters.
// - Split-When:
//   - Split when adapters contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Filesystem adapter for movie audit.
// - Description:
//   - Defines adapters data and behavior for rmv root.
// - Usage:
//   - Used by rmv root code that needs adapters.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Inbound and outbound adapters for RMV audit workflows.
//!
//! Driving adapters compose use cases while driven adapters implement the
//! source discovery and artifact publication ports.
pub mod driven;
/// Inbound adapters such as the command-line composition root.
pub mod driving;

pub use driven::{FilesystemMovieAuditor, TsvAuditManifestSink};
