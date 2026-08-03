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
//   - Audit manifest outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Audit manifest outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Audit manifest outbound port.

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
