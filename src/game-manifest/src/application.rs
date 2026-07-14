// File:
//   - application.rs
// Path:
//   - src/game-manifest/src/application.rs
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
//   - Game-manifest application commands and reports.
// - Must-Not:
//   - Implement filesystem traversal, storage, or CLI presentation.
// - Allows:
//   - Coordinate domain behavior through explicit ports.
// - Split-When:
//   - Split when a use-case family becomes independently deployable.
// - Merge-When:
//   - Another facade owns the same application commands.
// - Summary:
//   - Application facade for manifest workflows.
// - Description:
//   - Exposes generation, validation, expansion, and structural audit commands.
// - Usage:
//   - Called by driving adapters and library clients.
// - Defaults:
//   - No concrete adapter is selected.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Game-manifest application use cases.
//!
//! Commands depend on ports and leave tree traversal and storage to adapters.
mod diagnostic_path;
mod error;
mod generate_expanded;
mod generate_manifest;
mod path_evidence;
mod rcf_evidence;
mod structural_audit;
mod validate_manifest;

pub use error::ManifestError;
pub use generate_expanded::{
    EXPANDED_SCHEMA_LINE, GenerateExpandedManifest, GenerateExpandedReport,
};
pub use generate_manifest::{GenerateManifest, GenerateManifestReport};
pub use structural_audit::{StructuralAudit, StructuralAuditReport};
pub use validate_manifest::{ValidateManifest, ValidateManifestReport};
