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
//   - Output summary application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Output summary application service.
// - Description:
//   - Implements the declared application service responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Output summary application service.

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
        inventory.summarize(root, STANDARD_OUTPUT_DIRECTORIES)
    }
}
