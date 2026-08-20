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
//   - Movie auditor outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Movie auditor outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Movie auditor outbound port.

use std::path::{Path, PathBuf};

use crate::domain::{AuditReport, RmvError};

/// Audits movie sources without exposing their storage mechanism.
pub trait MovieAuditor {
    /// Audits all supplied roots against one expected output root.
    ///
    /// # Errors
    ///
    /// Returns an error when source access, paths, or output identities violate
    /// the audit contract.
    fn audit_roots(
        &self,
        roots: &[PathBuf],
        output_root: &Path,
    ) -> Result<AuditReport, RmvError>;
}
