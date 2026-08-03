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
//   - Package batch exporter outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Package batch exporter outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Package batch exporter outbound port.

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
