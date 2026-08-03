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
//   - Export package application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Export package application service.
// - Description:
//   - Implements the declared application service responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Export package application service.

use std::path::Path;

use crate::ports::PackageExporter;

/// Stateless lossless package-export use case.
#[derive(Debug, Clone, Copy)]
pub struct ExportPackage;

impl ExportPackage {
    /// Executes one explicit package export.
    ///
    /// # Errors
    ///
    /// Returns the selected exporter port failure.
    pub fn execute<E: PackageExporter>(
        exporter: &E,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<(), E::Error> {
        exporter.export_package(input_path, output_dir)
    }
}
