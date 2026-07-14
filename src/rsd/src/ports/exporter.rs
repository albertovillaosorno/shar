// File:
//   - exporter.rs
// Path:
//   - src/rsd/src/ports/exporter.rs
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
//   - Outbound batch RSD export contract.
// - Must-Not:
//   - Choose roots, print diagnostics, or prescribe filesystem mechanics.
// - Allows:
//   - Return deterministic export evidence from caller-supplied paths.
// - Split-When:
//   - Split when discovery, conversion, and publication need separate ports.
// - Merge-When:
//   - Another port owns the complete batch-export boundary.
// - Summary:
//   - Port for exporting RSD roots.
// - Description:
//   - Isolates application orchestration from concrete storage transactions.
// - Usage:
//   - Implemented by driven adapters and invoked by application commands.
// - Defaults:
//   - No source or output paths are inferred.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for deterministic RSD batch export.
//!
//! The application depends on this contract while traversal and transactional
//! writes remain replaceable adapter concerns.
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
