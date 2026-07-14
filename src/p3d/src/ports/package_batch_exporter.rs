// File:
//   - package_batch_exporter.rs
// Path:
//   - src/p3d/src/ports/package_batch_exporter.rs
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
//   - Outbound contract for deterministic multi-root package export.
// - Must-Not:
//   - Parse CLI arguments or expose filesystem implementation details.
// - Allows:
//   - Export caller-selected roots and return process-neutral counters.
// - Split-When:
//   - Split when discovery and publication need independent providers.
// - Merge-When:
//   - Another port owns the complete batch export boundary.
// - Summary:
//   - Port for batch `Pure3D` package export.
// - Description:
//   - Isolates application orchestration from traversal, cache, and reporting.
// - Usage:
//   - Implemented by driven adapters and supplied by driving adapters.
// - Defaults:
//   - Input and output roots remain explicit.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for deterministic batch package export.
//!
//! Application code depends on this contract instead of local traversal.
use std::path::{Path, PathBuf};

use crate::domain::PackageExportReport;

/// Exports packages discovered beneath caller-selected roots.
pub trait PackageBatchExporter {
    /// Provider-specific failure preserving batch context.
    type Error;

    /// Executes one deterministic batch export pass.
    ///
    /// # Errors
    ///
    /// Returns the provider failure when discovery or publication cannot
    /// finish.
    fn export_batch(
        &self,
        output_root: &Path,
        input_roots: &[PathBuf],
    ) -> Result<PackageExportReport, Self::Error>;
}
