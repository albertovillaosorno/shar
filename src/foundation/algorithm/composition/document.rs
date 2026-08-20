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
//   - Algorithm JSON wire records and settings serialization.
// - Must-Not:
//   - Own source admission, filesystem effects, or product-specific policy.
// - Allows:
//   - Parse and serialize mechanism-only algorithm records.
// - Split-When:
//   - Settings and algorithm document codecs gain independent lifecycles.
// - Merge-When:
//   - Another composition module owns the identical serialization boundary.
// - Summary:
//   - Algorithm serialization records and settings codec.
// - Description:
//   - Keeps serde and JSON ownership outside the pure domain model.
// - Usage:
//   - Used by algorithm authoring, replay, and the settings facade.
// - Defaults:
//   - Unknown fields and invalid settings fail explicitly.
//

//! Algorithm serialization records and settings codec.

use serde::{Deserialize, Serialize};

use crate::domain::{AlgorithmError, Settings};

pub(crate) const ALGORITHM_SCHEMA: &str = "shar.algorithm.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SettingsDocument {
    schema: String,
    minimum_source_files: u64,
    minimum_source_bytes: u64,
    maximum_source_files: u64,
    maximum_target_files: u64,
    maximum_file_bytes: u64,
    maximum_source_bytes: u64,
    maximum_target_bytes: u64,
}

impl From<&Settings> for SettingsDocument {
    fn from(settings: &Settings) -> Self {
        Self {
            schema: settings.schema().to_owned(),
            minimum_source_files: settings.minimum_source_files(),
            minimum_source_bytes: settings.minimum_source_bytes(),
            maximum_source_files: settings.maximum_source_files(),
            maximum_target_files: settings.maximum_target_files(),
            maximum_file_bytes: settings.maximum_file_bytes(),
            maximum_source_bytes: settings.maximum_source_bytes(),
            maximum_target_bytes: settings.maximum_target_bytes(),
        }
    }
}

impl Settings {
    /// Parses and validates settings from UTF-8 JSON text.
    ///
    /// # Errors
    /// Returns an error for malformed JSON or inconsistent limits.
    pub fn from_json(text: &str) -> Result<Self, AlgorithmError> {
        let decoded = serde_json::from_str(text);
        let document: SettingsDocument = decoded.map_err(|error| {
            AlgorithmError::new(format!("invalid settings JSON: {error}"))
        })?;
        let settings = Self {
            schema: document.schema,
            minimum_source_files: document.minimum_source_files,
            minimum_source_bytes: document.minimum_source_bytes,
            maximum_source_files: document.maximum_source_files,
            maximum_target_files: document.maximum_target_files,
            maximum_file_bytes: document.maximum_file_bytes,
            maximum_source_bytes: document.maximum_source_bytes,
            maximum_target_bytes: document.maximum_target_bytes,
        };
        settings.validate()?;
        Ok(settings)
    }
}

pub(crate) fn settings_json_bytes(
    settings: &Settings,
) -> Result<Vec<u8>, AlgorithmError> {
    serde_json::to_vec(&SettingsDocument::from(settings)).map_err(|error| {
        AlgorithmError::new(format!("cannot serialize settings: {error}"))
    })
}

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
