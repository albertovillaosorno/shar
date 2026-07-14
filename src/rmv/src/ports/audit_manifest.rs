// File:
//   - audit_manifest.rs
// Path:
//   - src/rmv/src/ports/audit_manifest.rs
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
//   - Outbound publication contract for completed audit reports.
// - Must-Not:
//   - Discover movies, mutate report semantics, or choose locations.
// - Allows:
//   - Publish a complete report in an adapter-defined representation.
// - Split-When:
//   - Split when independent artifact families need distinct publication.
// - Merge-When:
//   - Another port owns the same report publication boundary.
// - Summary:
//   - Port for publishing RMV audit manifests.
// - Description:
//   - Keeps serialization and storage outside application code.
// - Usage:
//   - Supplied to the audit use case by a driving adapter.
// - Defaults:
//   - The caller always supplies the output root.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for publishing completed RMV audit reports.
use std::path::Path;

use crate::domain::{AuditReport, RmvError};

/// Publishes a complete audit report.
pub trait AuditManifestSink {
    /// Writes one report to the supplied output root.
    ///
    /// # Errors
    ///
    /// Returns an error when serialization or publication fails.
    fn write_manifest(
        &self,
        output_root: &Path,
        report: &AuditReport,
    ) -> Result<(), RmvError>;
}
