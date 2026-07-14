// File:
//   - package_exporter.rs
// Path:
//   - src/p3d/src/ports/package_exporter.rs
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
//   - Outbound lossless `Pure3D` package export contract.
// - Must-Not:
//   - Choose paths, print diagnostics, or expose decoder implementation types.
// - Allows:
//   - Export one caller-selected package through a replaceable mechanism.
// - Split-When:
//   - Split when decode and publication require independent providers.
// - Merge-When:
//   - Another port owns the complete package export boundary.
// - Summary:
//   - Port for lossless `Pure3D` package export.
// - Description:
//   - Isolates application orchestration from concrete decoders and storage.
// - Usage:
//   - Implemented by driven adapters and invoked by application commands.
// - Defaults:
//   - Input and output paths are always caller supplied.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for lossless `Pure3D` package export.
//!
//! Decoder and filesystem mechanics stay behind this contract so application
//! commands depend only on package-level behavior.
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
