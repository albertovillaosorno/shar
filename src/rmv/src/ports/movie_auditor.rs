// File:
//   - movie_auditor.rs
// Path:
//   - src/rmv/src/ports/movie_auditor.rs
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
//   - Outbound movie discovery and audit contract.
// - Must-Not:
//   - Select roots, print diagnostics, or publish manifests.
// - Allows:
//   - Return deterministic reports from caller-supplied roots.
// - Split-When:
//   - Split when discovery and output probing need separate providers.
// - Merge-When:
//   - Another port owns the complete movie-audit contract.
// - Summary:
//   - Port for auditing movie roots.
// - Description:
//   - Isolates application orchestration from concrete storage.
// - Usage:
//   - Implemented by driven adapters and invoked by application code.
// - Defaults:
//   - No roots or output locations are inferred.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for deterministic movie-root auditing.
//!
//! The contract keeps source discovery replaceable without exposing storage
//! details to application orchestration.
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
