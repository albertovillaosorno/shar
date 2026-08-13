//! Generic algorithm domain records.

use core::fmt;
use serde::{Deserialize, Serialize};

const SETTINGS_SCHEMA: &str = "shar.algorithm.settings.v1";
pub(crate) const ALGORITHM_SCHEMA: &str = "shar.algorithm.v1";

/// Generic authoring and replay resource limits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    schema: String,
    minimum_source_files: u64,
    minimum_source_bytes: u64,
    maximum_source_files: u64,
    maximum_target_files: u64,
    maximum_file_bytes: u64,
    maximum_source_bytes: u64,
    maximum_target_bytes: u64,
}

impl Settings {
    /// Parses and validates settings from UTF-8 JSON text.
    ///
    /// # Errors
    /// Returns an error for malformed JSON or inconsistent limits.
    pub fn from_json(text: &str) -> Result<Self, AlgorithmError> {
        let settings: Self = serde_json::from_str(text)
            .map_err(|error| AlgorithmError::new(format!("invalid settings JSON: {error}")))?;
        settings.validate()?;
        Ok(settings)
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

    fn validate(&self) -> Result<(), AlgorithmError> {
        if self.schema != SETTINGS_SCHEMA {
            return Err(AlgorithmError::new(format!(
                "unsupported settings schema: {}",
                self.schema
            )));
        }
        if self.minimum_source_files == 0 {
            return Err(AlgorithmError::new("minimum_source_files must be positive"));
        }
        if self.minimum_source_bytes == 0 {
            return Err(AlgorithmError::new("minimum_source_bytes must be positive"));
        }
        if self.maximum_source_files < self.minimum_source_files {
            return Err(AlgorithmError::new(
                "maximum_source_files must admit minimum_source_files",
            ));
        }
        if self.maximum_target_files == 0 {
            return Err(AlgorithmError::new("maximum_target_files must be positive"));
        }
        if self.maximum_file_bytes == 0 {
            return Err(AlgorithmError::new("maximum_file_bytes must be positive"));
        }
        if self.maximum_source_bytes < self.minimum_source_bytes {
            return Err(AlgorithmError::new(
                "maximum_source_bytes must admit minimum_source_bytes",
            ));
        }
        if self.maximum_target_bytes == 0 {
            return Err(AlgorithmError::new("maximum_target_bytes must be positive"));
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
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for AlgorithmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AlgorithmError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum TargetKind {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceRecord {
    pub(crate) input: u64,
    pub(crate) path: String,
    pub(crate) bytes: u64,
    pub(crate) sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TargetDescriptor {
    pub(crate) path: String,
    pub(crate) bytes: u64,
    pub(crate) sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProtectedTarget {
    #[serde(flatten)]
    pub(crate) descriptor: TargetDescriptor,
    pub(crate) nonce: String,
    pub(crate) ciphertext: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AlgorithmDocument {
    pub(crate) schema: String,
    pub(crate) settings_sha256: String,
    pub(crate) source: Vec<SourceRecord>,
    pub(crate) target_kind: TargetKind,
    pub(crate) target: Vec<ProtectedTarget>,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthenticatedMetadata<'a> {
    pub(crate) schema: &'a str,
    pub(crate) settings_sha256: &'a str,
    pub(crate) source: &'a [SourceRecord],
    pub(crate) target_kind: TargetKind,
    pub(crate) target: &'a [TargetDescriptor],
}
