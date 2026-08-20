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
//   - Audit application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Audit application service.
// - Description:
//   - Implements the declared application service responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Audit application service.

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
        let report = auditor.audit_roots(roots, output_root)?;
        manifest_sink.write_manifest(output_root, &report)?;
        Ok(report)
    }
}
