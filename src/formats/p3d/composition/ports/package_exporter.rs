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
//   - Package exporter outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Package exporter outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Package exporter outbound port.

use std::path::Path;

/// Exports one validated package through a replaceable adapter.
pub trait PackageExporter {
    /// Adapter-specific failure preserving package context.
    type Error;

    /// Exports one package into the supplied output directory.
    ///
    /// # Errors
    ///
    /// Returns an adapter error when reading, decoding, or publication fails.
    fn export_package(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<(), Self::Error>;
}
