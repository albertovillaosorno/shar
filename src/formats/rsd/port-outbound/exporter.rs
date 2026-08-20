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
//   - Exporter outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Exporter outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Exporter outbound port.

use std::path::{Path, PathBuf};

use crate::domain::{ExportReport, RsdError};

/// Exports source roots through a replaceable mechanism.
pub trait Exporter {
    /// Adapter-specific failure preserving boundary and domain context.
    type Error: From<RsdError>;

    /// Exports every source root into one output tree.
    ///
    /// # Errors
    ///
    /// Returns an adapter error when discovery, conversion, or publication
    /// fails.
    fn export_roots(
        &self,
        roots: &[PathBuf],
        output_root: &Path,
    ) -> Result<ExportReport, Self::Error>;
}
