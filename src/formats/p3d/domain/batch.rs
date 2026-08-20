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
//   - Batch domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Batch domain module.
// - Description:
//   - Implements the declared domain module responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Batch domain module.

/// Deterministic report for one exported P3D package.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PackageExportReport {
    /// Number of scanned input packages.
    pub scanned: usize,
    /// Number of complete cached outputs skipped.
    pub skipped: usize,
    /// Number of packages extracted in this pass.
    pub extracted: usize,
    /// Number of package exports that failed.
    pub failed: usize,
}
