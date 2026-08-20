// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Export application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Export application service.
// - Description:
//   - Implements the declared application service responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Export application service.

use std::path::{Path, PathBuf};

use crate::domain::ExportReport;
use crate::ports::Exporter;

/// Stateless batch-export use case.
#[derive(Debug, Clone, Copy)]
pub struct ExportRoots;

impl ExportRoots {
    /// Executes one explicit batch export.
    ///
    /// # Errors
    ///
    /// Returns the selected exporter failure or invalid report evidence.
    pub fn execute<E: Exporter>(
        exporter: &E,
        roots: &[PathBuf],
        output_root: &Path,
    ) -> Result<ExportReport, E::Error> {
        let report = exporter.export_roots(roots, output_root)?;
        report.validate().map_err(E::Error::from)?;
        Ok(report)
    }
}

#[cfg(test)]
#[path = "../../../../tests/formats/rsd/unit/application/export/tests.rs"]
mod tests;
