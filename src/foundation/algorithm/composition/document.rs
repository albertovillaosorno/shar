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

use crate::domain::{AlgorithmError, Settings, SourceProjection};

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

const SOURCE_PROJECTION_KIND: &str = "offset-mask-set-v1";

fn encode_hex(bytes: &[u8]) -> String {
    use core::fmt::Write as _;
    let mut output = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        if write!(output, "{byte:02x}").is_err() {
            return output;
        }
    }
    output
}

fn projection_hex_value(byte: u8) -> Result<u8, AlgorithmError> {
    match byte {
        b'0'..=b'9' => Ok(byte.saturating_sub(b'0')),
        b'a'..=b'f' => Ok(byte.saturating_sub(b'a').saturating_add(10)),
        _ => Err(AlgorithmError::new(
            "source projection mask must be canonical lowercase hexadecimal",
        )),
    }
}

fn decode_lower_hex(text: &str) -> Result<Vec<u8>, AlgorithmError> {
    if !text.len().is_multiple_of(2)
        || !text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(AlgorithmError::new(
            "source projection mask must be canonical lowercase hexadecimal",
        ));
    }
    let bytes = text.as_bytes();
    let (pairs, remainder) = bytes.as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(AlgorithmError::new(
            "source projection mask has odd hexadecimal length",
        ));
    }
    let mut output = Vec::with_capacity(pairs.len());
    for pair in pairs {
        let [high_byte, low_byte] = *pair;
        let high = projection_hex_value(high_byte)?;
        let low = projection_hex_value(low_byte)?;
        output.push((high << 4) | low);
    }
    Ok(output)
}

fn encode_mask_chunks(mask: &[u8]) -> Vec<String> {
    let encoded = encode_hex(mask);
    encoded
        .as_bytes()
        .chunks(CIPHERTEXT_CHUNK_HEX_LEN)
        .map(|chunk| String::from_utf8_lossy(chunk).into_owned())
        .collect()
}

fn decode_mask_chunks(chunks: &[String]) -> Result<Vec<u8>, AlgorithmError> {
    if chunks.is_empty() {
        return Err(AlgorithmError::new(
            "source projection mask chunks must not be empty",
        ));
    }
    for (index, chunk) in chunks.iter().enumerate() {
        let is_last = index.checked_add(1) == Some(chunks.len());
        if chunk.is_empty()
            || chunk.len() > CIPHERTEXT_CHUNK_HEX_LEN
            || chunk.len() % 2 != 0
            || (!is_last && chunk.len() != CIPHERTEXT_CHUNK_HEX_LEN)
        {
            return Err(AlgorithmError::new(
                "source projection mask chunks are not canonical",
            ));
        }
    }
    decode_lower_hex(&chunks.concat())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceProjectionAlternativeDocument {
    span_bytes: u64,
    mask: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceProjectionDocument {
    kind: String,
    alternatives: Vec<SourceProjectionAlternativeDocument>,
}

impl SourceProjectionDocument {
    pub(crate) fn from_projection(projection: &SourceProjection) -> Self {
        let alternatives = projection
            .alternatives()
            .map(|(span_bytes, mask)| SourceProjectionAlternativeDocument {
                span_bytes,
                mask: encode_mask_chunks(mask),
            })
            .collect();
        Self {
            kind: SOURCE_PROJECTION_KIND.to_owned(),
            alternatives,
        }
    }

    pub(crate) fn to_projection(
        &self,
    ) -> Result<SourceProjection, AlgorithmError> {
        if self.kind != SOURCE_PROJECTION_KIND || self.alternatives.is_empty() {
            return Err(AlgorithmError::new(
                "unsupported or empty source projection",
            ));
        }
        let alternatives = self
            .alternatives
            .iter()
            .map(|alternative| {
                Ok((
                    alternative.span_bytes,
                    decode_mask_chunks(&alternative.mask)?,
                ))
            })
            .collect::<Result<Vec<_>, AlgorithmError>>()?;
        let projection = SourceProjection::offset_masks(alternatives)?;
        if Self::from_projection(&projection) != *self {
            return Err(AlgorithmError::new(
                "source projection is not canonical",
            ));
        }
        Ok(projection)
    }
}

impl SourceProjection {
    /// Parses one source-projection descriptor from JSON text.
    ///
    /// # Errors
    /// Returns an error when JSON or projection metadata is invalid.
    pub fn from_json(text: &str) -> Result<Self, AlgorithmError> {
        let document: SourceProjectionDocument = serde_json::from_str(text)
            .map_err(|error| {
                AlgorithmError::new(format!(
                    "invalid source projection JSON: {error}"
                ))
            })?;
        document.to_projection()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceRecord {
    pub(crate) input: u64,
    pub(crate) path: String,
    pub(crate) bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) projection: Option<SourceProjectionDocument>,
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
