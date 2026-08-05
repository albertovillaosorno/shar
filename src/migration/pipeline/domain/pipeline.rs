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
//   - Pipeline domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Pipeline domain module.
// - Description:
//   - Implements the declared domain module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Pipeline domain module.

use std::path::PathBuf;

/// Explicit configuration for one pipeline execution.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Root containing the caller's lawful game copy.
    pub game_root: PathBuf,
    /// Root receiving generated extraction artifacts.
    pub extracted_root: PathBuf,
    /// Whether existing extracted output is removed before execution.
    pub clean_extracted: bool,
    /// Whether the caller explicitly approved applying optional packages.
    pub approve_optional_mods: bool,
}

/// Complete ordered report for one pipeline execution.
#[derive(Debug, Clone, Default)]
pub struct PipelineReport {
    /// Ordered stage reports.
    pub stages: Vec<StageReport>,
}

/// Observable result of one pipeline stage.
#[derive(Debug, Clone)]
pub struct StageReport {
    /// Stable stage name.
    pub name: &'static str,
    /// Number of files produced or inspected.
    pub files: usize,
    /// Number of bytes produced or inspected.
    pub bytes: u64,
    /// Human-readable deterministic stage note.
    pub note: String,
}

impl StageReport {
    /// Adds one file count without saturating a stage report.
    pub(crate) fn checked_file_total(
        stage: &'static str,
        total: usize,
        files: usize,
    ) -> PipelineOutcome<usize> {
        let message = format!("{stage} file count overflowed");
        total
            .checked_add(files)
            .map_or_else(|| Err(PipelineError::new(message)), Ok)
    }

    /// Adds one byte length without saturating a stage report.
    pub(crate) fn checked_byte_total(
        stage: &'static str,
        total: u64,
        bytes: u64,
    ) -> PipelineOutcome<u64> {
        let message = format!("{stage} byte total overflowed");
        total
            .checked_add(bytes)
            .map_or_else(|| Err(PipelineError::new(message)), Ok)
    }
}

/// Stable pipeline failure presented to callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineError {
    /// Human-readable failure message without machine-local evidence details.
    message: String,
}

impl PipelineError {
    /// Creates one pipeline failure from a public-safe message.
    #[must_use]
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Returns untrusted diagnostic text without raw control characters.
fn escaped_diagnostic_text(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character.is_control() {
            output.extend(character.escape_default());
        } else {
            output.push(character);
        }
    }
    output
}

impl core::fmt::Display for PipelineError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let rendered_message = escaped_diagnostic_text(&self.message);
        formatter.write_str(&rendered_message)
    }
}

impl std::error::Error for PipelineError {}

/// Result type used by pipeline application use cases.
pub type PipelineOutcome<T> = Result<T, PipelineError>;

#[cfg(test)]
#[path = "../../../../tests/migration/pipeline/unit/domain/pipeline_tests.rs"]
mod tests;
