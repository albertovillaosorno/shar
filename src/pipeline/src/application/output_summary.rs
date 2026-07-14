// File:
//   - output_summary.rs
// Path:
//   - src/pipeline/src/application/output_summary.rs
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
//   - Application orchestration for pipeline output inventory.
// - Must-Not:
//   - Traverse storage or render command-line diagnostics.
// - Allows:
//   - Request output evidence through an explicit port.
// - Split-When:
//   - Split when inventory policy gains another independent use case.
// - Merge-When:
//   - Another application module owns the same inventory request.
// - Summary:
//   - Pipeline output-summary use case.
// - Description:
//   - Separates CLI presentation from filesystem evidence gathering.
// - Usage:
//   - Called by driving adapters after successful pipeline execution.
// - Defaults:
//   - Standard output directory families remain one explicit ordered
//   - constant.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Application use case for output inventory evidence.
//!
//! The use case depends only on the output-inventory port.
use std::path::Path;

use crate::domain::{OutputSummary, PipelineOutcome};
use crate::ports::OutputInventory;

/// Output directory families presented by the pipeline CLI.
pub const STANDARD_OUTPUT_DIRECTORIES: &[&str] = &[
    "art", "movies", "sound", "ambience", "dialog", "music", "carsound",
];

/// Stateless pipeline output-summary use case.
#[derive(Debug, Clone, Copy)]
pub struct SummarizeOutput;

impl SummarizeOutput {
    /// Inventories one generated output root.
    ///
    /// # Errors
    ///
    /// Returns a pipeline failure when the provider cannot inspect storage.
    pub fn execute(
        inventory: &impl OutputInventory,
        root: &Path,
    ) -> PipelineOutcome<OutputSummary> {
        inventory.summarize(
            root,
            STANDARD_OUTPUT_DIRECTORIES,
        )
    }
}
