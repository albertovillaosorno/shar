// File:
//   - export_batch.rs
// Path:
//   - src/p3d/src/application/export_batch.rs
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
//   - Application command for one explicit multi-root package export.
// - Must-Not:
//   - Traverse storage, maintain cache files, or select concrete exporters.
// - Allows:
//   - Delegate caller-selected roots through a batch-export port.
// - Split-When:
//   - Split when batch planning and execution become independent use cases.
// - Merge-When:
//   - Another use case owns the complete batch export command.
// - Summary:
//   - Application command for batch `Pure3D` export.
// - Description:
//   - Keeps inbound adapters independent from local batch mechanisms.
// - Usage:
//   - Invoked by the batch CLI with an explicit driven provider.
// - Defaults:
//   - No path or adapter is inferred.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for batch `Pure3D` package export.
//!
//! The command delegates explicit roots without selecting a provider.
use std::path::{Path, PathBuf};

use crate::domain::PackageExportReport;
use crate::ports::PackageBatchExporter;

/// Stateless batch package-export use case.
#[derive(Debug, Clone, Copy)]
pub struct ExportPackageBatch;

impl ExportPackageBatch {
    /// Executes one explicit multi-root export pass.
    ///
    /// # Errors
    ///
    /// Returns the selected batch exporter failure.
    pub fn execute<E: PackageBatchExporter>(
        exporter: &E,
        output_root: &Path,
        input_roots: &[PathBuf],
    ) -> Result<PackageExportReport, E::Error> {
        exporter.export_batch(
            output_root,
            input_roots,
        )
    }
}
