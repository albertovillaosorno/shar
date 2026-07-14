// File:
//   - audit.rs
// Path:
//   - src/rmv/src/application/audit.rs
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
//   - The audit command use case and publication sequence.
// - Must-Not:
//   - Traverse filesystems, serialize TSV, or print diagnostics.
// - Allows:
//   - Invoke audit and manifest ports in deterministic success order.
// - Split-When:
//   - Split when audit and publication become independent commands.
// - Merge-When:
//   - Another use case owns the complete audit sequence.
// - Summary:
//   - Application use case for RMV audit publication.
// - Description:
//   - Audits roots, publishes the complete report, and returns it.
// - Usage:
//   - Invoked by driving adapters after concrete ports are selected.
// - Defaults:
//   - Publication occurs only after a complete successful audit.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for auditing movies and publishing one complete report.
use std::path::{Path, PathBuf};

use crate::domain::{AuditReport, RmvError};
use crate::ports::{AuditManifestSink, MovieAuditor};

/// Coordinates movie auditing and manifest publication through explicit ports.
#[derive(Debug, Clone, Copy)]
pub struct RunMovieAudit;

impl RunMovieAudit {
    /// Executes one complete audit and publishes its report.
    ///
    /// # Errors
    ///
    /// Returns the first audit or publication failure without reporting partial
    /// success.
    pub fn execute(
        auditor: &impl MovieAuditor,
        manifest_sink: &impl AuditManifestSink,
        roots: &[PathBuf],
        output_root: &Path,
    ) -> Result<AuditReport, RmvError> {
        let report = auditor.audit_roots(
            roots,
            output_root,
        )?;
        manifest_sink.write_manifest(
            output_root,
            &report,
        )?;
        Ok(report)
    }
}
