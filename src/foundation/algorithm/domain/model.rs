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
//   - Pure mechanism settings and stable algorithm errors.
// - Must-Not:
//   - Own JSON serialization, filesystem effects, or product policy.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Algorithm domain records.
// - Description:
//   - Pure mechanism settings and stable algorithm errors.
// - Usage:
//   - Used through the owning algorithm function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Generic algorithm domain records.

use core::fmt;

const SETTINGS_SCHEMA: &str = "shar.algorithm.settings.v1";

/// Generic authoring and replay resource limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub(crate) schema: String,
    pub(crate) minimum_source_files: u64,
    pub(crate) minimum_source_bytes: u64,
    pub(crate) maximum_source_files: u64,
    pub(crate) maximum_target_files: u64,
    pub(crate) maximum_file_bytes: u64,
    pub(crate) maximum_source_bytes: u64,
    pub(crate) maximum_target_bytes: u64,
}

impl Settings {
    pub(crate) fn schema(&self) -> &str {
        &self.schema
    }

    /// Returns the minimum admitted source-file count.
    #[must_use]
    pub const fn minimum_source_files(&self) -> u64 {
        self.minimum_source_files
    }

    /// Returns the minimum admitted aggregate source byte count.
    #[must_use]
    pub const fn minimum_source_bytes(&self) -> u64 {
        self.minimum_source_bytes
    }

    pub(crate) const fn maximum_source_files(&self) -> u64 {
        self.maximum_source_files
    }

    pub(crate) const fn maximum_target_files(&self) -> u64 {
        self.maximum_target_files
    }

    pub(crate) const fn maximum_file_bytes(&self) -> u64 {
        self.maximum_file_bytes
    }

    pub(crate) const fn maximum_source_bytes(&self) -> u64 {
        self.maximum_source_bytes
    }

    pub(crate) const fn maximum_target_bytes(&self) -> u64 {
        self.maximum_target_bytes
    }

    pub(crate) fn validate(&self) -> Result<(), AlgorithmError> {
        if self.schema != SETTINGS_SCHEMA {
            return Err(AlgorithmError::new(format!(
                "unsupported settings schema: {}",
                self.schema
            )));
        }
        if self.minimum_source_files == 0 {
            return Err(AlgorithmError::new(
                "minimum_source_files must be positive",
            ));
        }
        if self.minimum_source_bytes == 0 {
            return Err(AlgorithmError::new(
                "minimum_source_bytes must be positive",
            ));
        }
        if self.maximum_source_files < self.minimum_source_files {
            return Err(AlgorithmError::new(
                "maximum_source_files must admit minimum_source_files",
            ));
        }
        if self.maximum_target_files == 0 {
            return Err(AlgorithmError::new(
                "maximum_target_files must be positive",
            ));
        }
        if self.maximum_file_bytes == 0 {
            return Err(AlgorithmError::new(
                "maximum_file_bytes must be positive",
            ));
        }
        if self.maximum_source_bytes < self.minimum_source_bytes {
            return Err(AlgorithmError::new(
                "maximum_source_bytes must admit minimum_source_bytes",
            ));
        }
        let source_capacity = self
            .maximum_source_files
            .saturating_mul(self.maximum_file_bytes);
        if source_capacity < self.minimum_source_bytes {
            return Err(AlgorithmError::new(
                "source file limits cannot satisfy minimum_source_bytes",
            ));
        }
        if self.maximum_target_bytes == 0 {
            return Err(AlgorithmError::new(
                "maximum_target_bytes must be positive",
            ));
        }
        Ok(())
    }
}

/// Stable algorithm engine error without private source contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmError {
    message: String,
}

impl AlgorithmError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl fmt::Display for AlgorithmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AlgorithmError {}
