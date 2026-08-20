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
//   - Output summary domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Output summary domain module.
// - Description:
//   - Implements the declared domain module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Output summary domain module.

use std::path::PathBuf;

/// File count for one caller-selected output directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectorySummary {
    /// Caller-visible directory label.
    pub name: &'static str,
    /// Number of regular files beneath the directory.
    pub files: usize,
}

/// Complete inventory for one pipeline output root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputSummary {
    /// Canonical path when available, otherwise the explicit root.
    pub root: PathBuf,
    /// Number of regular files beneath the output root.
    pub files: usize,
    /// Sum of regular-file byte lengths beneath the output root.
    pub bytes: u64,
    /// Ordered summaries for selected output directories.
    pub directories: Vec<DirectorySummary>,
}
