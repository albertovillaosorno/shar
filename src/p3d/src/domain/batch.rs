// File:
//   - batch.rs
// Path:
//   - src/p3d/src/domain/batch.rs
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
//   - Process-neutral counters for one batch package export.
// - Must-Not:
//   - Traverse storage, decode packages, or format operator diagnostics.
// - Allows:
//   - Represent scanned, skipped, extracted, and failed package counts.
// - Split-When:
//   - Split when batch evidence gains independently versioned record families.
// - Merge-When:
//   - Another domain type owns the same complete batch result.
// - Summary:
//   - Batch package export report.
// - Description:
//   - Carries deterministic counters between application and driving adapters.
// - Usage:
//   - Returned by the batch-export port and presented by the CLI adapter.
// - Defaults:
//   - Every counter starts at zero.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Process-neutral result of one batch `Pure3D` export.
//!
//! Counters remain independent from discovery, cache, and CLI mechanisms.

/// Deterministic counters for one batch export pass.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PackageExportReport {
    /// Number of scanned input packages.
    pub scanned: usize,
    /// Number of complete cached outputs skipped.
    pub skipped: usize,
    /// Number of packages extracted in this pass.
    pub extracted: usize,
    /// Number of package exports that failed.
    pub failed: usize,
}
