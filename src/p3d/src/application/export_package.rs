// File:
//   - export_package.rs
// Path:
//   - src/p3d/src/application/export_package.rs
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
//   - The lossless package export application command.
// - Must-Not:
//   - Decode chunks, traverse directories, or write artifacts directly.
// - Allows:
//   - Invoke a package exporter port for caller-supplied paths.
// - Split-When:
//   - Split when planning and execution become independent use cases.
// - Merge-When:
//   - Another use case owns the complete package export command.
// - Summary:
//   - Application command for `Pure3D` package export.
// - Description:
//   - Delegates one explicit package request through an exporter port.
// - Usage:
//   - Invoked by single and batch driving adapters.
// - Defaults:
//   - No adapter or path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for exporting one `Pure3D` package through a port.
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
        exporter.export_package(
            input_path, output_dir,
        )
    }
}
