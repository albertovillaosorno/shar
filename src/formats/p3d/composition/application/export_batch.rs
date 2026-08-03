// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Export batch application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Export batch application service.
// - Description:
//   - Implements the declared application service responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Export batch application service.

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
        exporter.export_batch(output_root, input_roots)
    }
}
