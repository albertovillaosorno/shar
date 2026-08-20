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

use serde::de::Deserializer;
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};

use crate::domain::{AlgorithmError, Settings};

pub(crate) const ALGORITHM_SCHEMA: &str = "shar.algorithm.v1";
const CIPHERTEXT_CHUNK_HEX_LEN: usize = 64;

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

fn serialize_ciphertext<S>(
    ciphertext: &str,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let chunk_count = ciphertext.len().div_ceil(CIPHERTEXT_CHUNK_HEX_LEN);
    let mut sequence = serializer.serialize_seq(Some(chunk_count))?;
    for bytes in ciphertext.as_bytes().chunks(CIPHERTEXT_CHUNK_HEX_LEN) {
        let chunk =
            std::str::from_utf8(bytes).map_err(serde::ser::Error::custom)?;
        sequence.serialize_element(chunk)?;
    }
    sequence.end()
}

fn deserialize_ciphertext<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum CiphertextWire {
        Legacy(String),
        Chunks(Vec<String>),
    }

    match CiphertextWire::deserialize(deserializer)? {
        CiphertextWire::Legacy(ciphertext) => Ok(ciphertext),
        CiphertextWire::Chunks(chunks) => {
            if chunks.is_empty() {
                return Err(serde::de::Error::custom(
                    "ciphertext chunks must not be empty",
                ));
            }
            for (index, chunk) in chunks.iter().enumerate() {
                let is_last = index == chunks.len().saturating_sub(1);
                let width = chunk.len();
                if width == 0
                    || width > CIPHERTEXT_CHUNK_HEX_LEN
                    || width % 2 != 0
                    || (!is_last && width != CIPHERTEXT_CHUNK_HEX_LEN)
                {
                    return Err(serde::de::Error::custom(
                        "ciphertext chunks are not canonical",
                    ));
                }
            }
            Ok(chunks.concat())
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProtectedTarget {
    #[serde(flatten)]
    pub(crate) descriptor: TargetDescriptor,
    pub(crate) nonce: String,
    #[serde(
        serialize_with = "serialize_ciphertext",
        deserialize_with = "deserialize_ciphertext"
    )]
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

fn is_hash_field_prefix(prefix: &str) -> bool {
    matches!(prefix, "\"settings_sha256\":" | "\"sha256\":")
}

fn wrap_hash_fields(text: &str) -> String {
    let mut bounded = String::with_capacity(text.len());
    for line in text.lines() {
        let trimmed = line.trim_start();
        let indent = line.strip_suffix(trimmed).unwrap_or_default();
        if line.len() > 80
            && let Some((field, field_value)) = trimmed.split_once(' ')
            && is_hash_field_prefix(field)
        {
            bounded.push_str(indent);
            bounded.push_str(field);
            bounded.push('\n');
            bounded.push_str(indent);
            bounded.push_str("  ");
            bounded.push_str(field_value);
            bounded.push('\n');
            continue;
        }
        bounded.push_str(line);
        bounded.push('\n');
    }
    bounded
}

pub(crate) fn algorithm_json_text(
    document: &AlgorithmDocument,
) -> Result<String, AlgorithmError> {
    let text = serde_json::to_string_pretty(document).map_err(|error| {
        AlgorithmError::new(format!("cannot serialize algorithm: {error}"))
    })?;
    Ok(wrap_hash_fields(&text))
}
