// File:
//   - domain.rs
// Path:
//   - src/p3d/src/domain/domain.rs
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
//   - p3d module behavior for domain.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute domain.
// - Split-When:
//   - Split when domain contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Defines chunk for this module boundary.
// - Description:
//   - Defines domain data and behavior for p3d root.
// - Usage:
//   - Used by p3d root code that needs domain.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Defines chunk for this module boundary.
//!
//! This boundary keeps defines chunk for this module boundary explicit and
//! returns deterministic results to p3d callers.
pub mod batch;
pub mod chunk;
/// Item.
pub mod extract;

pub use batch::PackageExportReport;
pub use chunk::{ChunkKind, ChunkRecord, P3dDocument, P3dError, analyze_p3d};
pub use extract::{PreparedP3d, prepare_p3d_bytes};
