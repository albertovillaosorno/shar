// File:
//   - output_summary.rs
// Path:
//   - src/pipeline/src/domain/output_summary.rs
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
//   - Process-neutral pipeline output inventory values.
// - Must-Not:
//   - Traverse storage or decide command presentation formatting.
// - Allows:
//   - Represent total and named-directory file counts and byte totals.
// - Split-When:
//   - Split when artifact families need independently versioned summaries.
// - Merge-When:
//   - Another domain module owns the same output inventory values.
// - Summary:
//   - Pipeline output inventory domain values.
// - Description:
//   - Separates output evidence from filesystem and CLI adapters.
// - Usage:
//   - Produced through an output-inventory port and rendered by the CLI.
// - Defaults:
//   - Named directory summaries remain ordered by caller policy.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Process-neutral output inventory values.
//!
//! Storage traversal and presentation remain outside the domain.
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
