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
//   - Decoded billboard animation source-evidence adapter.
// - Must-Not:
//   - Interpret BQG channels as skeletal FBX or runtime presentation semantics.
// - Allows:
//   - Strict validation and exact typed retention of decoded BQG evidence.
// - Split-When:
//   - Another non-skeletal animation family gains independent semantics.
// - Merge-When:
//   - Another adapter owns the identical decoded BQG evidence contract.
// - Summary:
//   - Decoded billboard animation source evidence.
// - Description:
//   - Validates decoded BQG timing, grouping, and channel payloads without
//   - sampling, flattening, or interpreting them.
// - Usage:
//   - Used by deferred world billboard provenance.
// - Defaults:
//   - Unsupported, contradictory, or malformed evidence fails explicitly.
//

//! Decoded billboard animation source-evidence adapter.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Exact decoded source evidence for one BQG animation.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BillboardAnimationSourceEvidence {
    /// Authored animation identity after fixed-width NUL removal.
    pub identity: String,
    /// Supported source animation version.
    pub version: u32,
    /// Authored source-frame span.
    pub frame_count: f32,
    /// Authored source frames per second.
    pub frame_rate: f32,
    /// Whether the decoded source marks the clip cyclic.
    pub cyclic: bool,
    /// Authored platform-size evidence in source order.
    pub sizes: Vec<BillboardAnimationSizeEvidence>,
    /// Authored animation group lists in source order.
    pub group_lists: Vec<BillboardAnimationGroupListEvidence>,
}

/// One decoded platform-size record retained without interpretation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BillboardAnimationSizeEvidence {
    /// Supported size-record version.
    pub version: u32,
    /// Authored PC size value.
    pub pc: u32,
    /// Authored `PlayStation 2` size value.
    pub ps2: u32,
    /// Authored Xbox size value.
    pub xbox: u32,
    /// Authored `GameCube` size value.
    pub gc: u32,
}

/// One BQG group-list container in authored order.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BillboardAnimationGroupListEvidence {
    /// Supported source group-list version.
    pub version: u32,
    /// Declared number of child groups.
    pub declared_group_count: usize,
    /// Decoded child groups in authored order.
    pub groups: Vec<BillboardAnimationGroupEvidence>,
}

/// One BQG target group in authored order.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BillboardAnimationGroupEvidence {
    /// Supported source group version.
    pub version: u32,
    /// Authored target identity after fixed-width NUL removal.
    pub identity: String,
    /// Authored numeric group id.
    pub group_id: u32,
    /// Declared number of child channels.
    pub declared_channel_count: usize,
    /// Decoded channels in authored order.
    pub channels: Vec<BillboardAnimationChannelEvidence>,
}

/// One decoded interpolation metadata record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BillboardAnimationMetadataEvidence {
    /// Decoded metadata record family.
    pub kind: String,
    /// Supported metadata version.
    pub version: u32,
    /// Authored interpolation mode.
    pub mode: u32,
}

/// One exact decoded BQG channel payload.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum BillboardAnimationChannelEvidence {
    /// One-component floating channel.
    #[serde(rename = "float1")]
    Float1 {
        /// Supported channel version.
        version: u32,
        /// Authored `FourCC` parameter.
        param: String,
        /// Declared key count.
        declared_key_count: usize,
        /// Authored key-frame indices.
        frames: Vec<u16>,
        /// Authored scalar values.
        values: Vec<[f32; 1]>,
        /// Authored channel metadata.
        metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Two-component floating channel.
    #[serde(rename = "float2")]
    Float2 {
        /// Supported channel version.
        version: u32,
        /// Authored `FourCC` parameter.
        param: String,
        /// Declared key count.
        declared_key_count: usize,
        /// Authored key-frame indices.
        frames: Vec<u16>,
        /// Authored two-component values.
        values: Vec<[f32; 2]>,
        /// Authored channel metadata.
        metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Three-component vector channel.
    #[serde(rename = "vector3")]
    Vector3 {
        /// Supported channel version.
        version: u32,
        /// Authored `FourCC` parameter.
        param: String,
        /// Declared key count.
        declared_key_count: usize,
        /// Authored key-frame indices.
        frames: Vec<u16>,
        /// Authored vector values.
        values: Vec<[f32; 3]>,
        /// Authored channel metadata.
        metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Four-component quaternion channel.
    #[serde(rename = "quaternion")]
    Quaternion {
        /// Supported channel version.
        version: u32,
        /// Authored `FourCC` parameter.
        param: String,
        /// Declared key count.
        declared_key_count: usize,
        /// Authored key-frame indices.
        frames: Vec<u16>,
        /// Authored WXYZ values without normalization.
        values: Vec<[f32; 4]>,
        /// Authored channel metadata.
        metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Boolean state channel without explicit frame indices.
    #[serde(rename = "bool")]
    Bool {
        /// Supported channel version.
        version: u32,
        /// Authored `FourCC` parameter.
        param: String,
        /// Authored initial state.
        start_state: u32,
        /// Declared state count.
        declared_key_count: usize,
        /// Authored state values.
        values: Vec<u32>,
        /// Authored channel metadata.
        metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Packed colour channel.
    #[serde(rename = "colour")]
    Colour {
        /// Supported channel version.
        version: u32,
        /// Authored `FourCC` parameter.
        param: String,
        /// Declared key count.
        declared_key_count: usize,
        /// Authored key-frame indices.
        frames: Vec<u16>,
        /// Authored packed colour values.
        values: Vec<u32>,
        /// Authored channel metadata.
        metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
}

/// Decode one extracted BQG animation as exact source evidence.
///
/// # Errors
///
/// Returns an error when JSON shape, identity, timing, counts, channel kinds,
/// key ordering, or finite numeric evidence violates the decoded BQG contract.
pub fn read_billboard_animation_source_evidence(
    path: &Path,
    requested_id: &str,
) -> Result<BillboardAnimationSourceEvidence, DecodedBillboardAnimationError> {
    let bytes = fs::read(path).map_err(|error| {
        DecodedBillboardAnimationError::Read {
            path: path.display().to_string(),
            source: error.to_string(),
        }
    })?;
    let document: AnimationDocument =
        serde_json::from_slice(&bytes).map_err(|error| {
            DecodedBillboardAnimationError::Parse {
                path: path.display().to_string(),
                source: error.to_string(),
            }
        })?;
    validate_document(document, requested_id)
}

/// Validate one strict decoded BQG document without assigning runtime meaning.
fn validate_document(
    document: AnimationDocument,
    requested_id: &str,
) -> Result<BillboardAnimationSourceEvidence, DecodedBillboardAnimationError> {
    if document.schema != "animation"
        || document.version != 0
        || document.animation_type != "BQG_"
    {
        return Err(DecodedBillboardAnimationError::UnsupportedDocument);
    }
    let identity = clean_identity(&document.name)?;
    if !identity.eq_ignore_ascii_case(requested_id) {
        return Err(DecodedBillboardAnimationError::IdentityMismatch {
            requested: requested_id.to_owned(),
            decoded: identity,
        });
    }
    if !document.frames.is_finite()
        || document.frames <= 0.
        || !document.frame_rate.is_finite()
        || document.frame_rate <= 0.
        || document.cyclic > 1
    {
        return Err(DecodedBillboardAnimationError::InvalidTiming);
    }
    if !document.loose_channels.is_empty()
        || !document.legacy_animation_extras.is_empty()
    {
        return Err(DecodedBillboardAnimationError::UnsupportedTopLevelPayload);
    }
    if document.sizes.iter().any(|size| size.version != 1) {
        return Err(DecodedBillboardAnimationError::UnsupportedDocument);
    }
    let group_lists = document
        .group_lists
        .into_iter()
        .map(|list| validate_group_list(list, document.frames))
        .collect::<Result<Vec<_>, _>>()?;
    validate_unique_groups(&group_lists)?;
    Ok(BillboardAnimationSourceEvidence {
        identity,
        version: document.version,
        frame_count: document.frames,
        frame_rate: document.frame_rate,
        cyclic: document.cyclic != 0,
        sizes: document.sizes,
        group_lists,
    })
}

/// Validate one group-list count and child records.
fn validate_group_list(
    list: GroupListDocument,
    clip_frames: f32,
) -> Result<BillboardAnimationGroupListEvidence, DecodedBillboardAnimationError>
{
    if list.version != 0 || list.num_groups != list.groups.len() {
        return Err(DecodedBillboardAnimationError::InvalidGroupList);
    }
    let groups = list
        .groups
        .into_iter()
        .map(|group| validate_group(group, clip_frames))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BillboardAnimationGroupListEvidence {
        version: list.version,
        declared_group_count: list.num_groups,
        groups,
    })
}

/// Validate one group and its authored channel sequence.
fn validate_group(
    group: GroupDocument,
    clip_frames: f32,
) -> Result<BillboardAnimationGroupEvidence, DecodedBillboardAnimationError> {
    let identity = clean_identity(&group.name)?;
    if group.version != 0 || group.num_channels != group.channels.len() {
        return Err(DecodedBillboardAnimationError::InvalidGroup(identity));
    }
    let mut params = BTreeSet::new();
    let mut channels = Vec::with_capacity(group.channels.len());
    for channel in group.channels {
        let evidence = validate_channel(channel, clip_frames, &identity)?;
        let param = channel_param(&evidence);
        if !params.insert(param.to_owned()) {
            return Err(DecodedBillboardAnimationError::InvalidChannel {
                group: identity,
                param: param.to_owned(),
            });
        }
        channels.push(evidence);
    }
    Ok(BillboardAnimationGroupEvidence {
        version: group.version,
        identity,
        group_id: group.group_id,
        declared_channel_count: group.num_channels,
        channels,
    })
}

/// Validate one exact BQG channel without sampling or flattening it.
fn validate_channel(
    channel: ChannelDocument,
    clip_frames: f32,
    group: &str,
) -> Result<BillboardAnimationChannelEvidence, DecodedBillboardAnimationError> {
    match channel {
        ChannelDocument::Float1 {
            version,
            param,
            num_frames,
            frames,
            values,
            channel_metadata,
        } => {
            validate_channel_header(version, &param, group, &[
                "\x57\x44\x54\x5f",
                "HGT_",
                "DIST",
                "ERNG",
                "SRNG",
            ])?;
            validate_float_series(
                num_frames,
                &frames,
                &values,
                clip_frames,
                group,
                &param,
            )?;
            validate_metadata(&channel_metadata, group, &param)?;
            Ok(BillboardAnimationChannelEvidence::Float1 {
                version,
                param,
                declared_key_count: num_frames,
                frames,
                values,
                metadata: channel_metadata,
            })
        },
        ChannelDocument::Float2 {
            version,
            param,
            num_frames,
            frames,
            values,
            channel_metadata,
        } => {
            validate_channel_header(version, &param, group, &["OFF_", "ORNG"])?;
            validate_float_series(
                num_frames,
                &frames,
                &values,
                clip_frames,
                group,
                &param,
            )?;
            validate_metadata(&channel_metadata, group, &param)?;
            Ok(BillboardAnimationChannelEvidence::Float2 {
                version,
                param,
                declared_key_count: num_frames,
                frames,
                values,
                metadata: channel_metadata,
            })
        },
        ChannelDocument::Vector3 {
            version,
            param,
            num_frames,
            frames,
            values,
            channel_metadata,
        } => {
            validate_channel_header(version, &param, group, &["TRAN"])?;
            validate_float_series(
                num_frames,
                &frames,
                &values,
                clip_frames,
                group,
                &param,
            )?;
            validate_metadata(&channel_metadata, group, &param)?;
            Ok(BillboardAnimationChannelEvidence::Vector3 {
                version,
                param,
                declared_key_count: num_frames,
                frames,
                values,
                metadata: channel_metadata,
            })
        },
        ChannelDocument::Quaternion {
            version,
            param,
            num_frames,
            frames,
            values,
            channel_metadata,
        } => {
            validate_channel_header(version, &param, group, &["ROT_"])?;
            validate_float_series(
                num_frames,
                &frames,
                &values,
                clip_frames,
                group,
                &param,
            )?;
            validate_metadata(&channel_metadata, group, &param)?;
            Ok(BillboardAnimationChannelEvidence::Quaternion {
                version,
                param,
                declared_key_count: num_frames,
                frames,
                values,
                metadata: channel_metadata,
            })
        },
        ChannelDocument::Bool {
            version,
            param,
            start_state,
            num_frames,
            values,
            channel_metadata,
        } => {
            validate_channel_header(version, &param, group, &["VIS_"])?;
            if num_frames == 0
                || num_frames != values.len()
                || start_state > 1
                || values.iter().any(|value| *value > 1)
            {
                return invalid_channel(group, &param);
            }
            validate_metadata(&channel_metadata, group, &param)?;
            Ok(BillboardAnimationChannelEvidence::Bool {
                version,
                param,
                start_state,
                declared_key_count: num_frames,
                values,
                metadata: channel_metadata,
            })
        },
        ChannelDocument::Colour {
            version,
            param,
            num_frames,
            frames,
            values,
            channel_metadata,
        } => {
            validate_channel_header(version, &param, group, &[
                "\x43\x4c\x52\x5f",
            ])?;
            validate_key_series(
                num_frames,
                &frames,
                values.len(),
                clip_frames,
                group,
                &param,
            )?;
            validate_metadata(&channel_metadata, group, &param)?;
            Ok(BillboardAnimationChannelEvidence::Colour {
                version,
                param,
                declared_key_count: num_frames,
                frames,
                values,
                metadata: channel_metadata,
            })
        },
    }
}

/// Validate one supported channel version and `FourCC` parameter.
fn validate_channel_header(
    version: u32,
    param: &str,
    group: &str,
    accepted: &[&str],
) -> Result<(), DecodedBillboardAnimationError> {
    if version != 0 || !accepted.contains(&param) {
        return invalid_channel(group, param);
    }
    Ok(())
}

/// Validate keyed floating values without changing their decoded bits.
fn validate_float_series<const WIDTH: usize>(
    declared: usize,
    frames: &[u16],
    values: &[[f32; WIDTH]],
    clip_frames: f32,
    group: &str,
    param: &str,
) -> Result<(), DecodedBillboardAnimationError> {
    validate_key_series(
        declared,
        frames,
        values.len(),
        clip_frames,
        group,
        param,
    )?;
    if values.iter().flatten().any(|value| !value.is_finite()) {
        return invalid_channel(group, param);
    }
    Ok(())
}

/// Validate one strictly increasing keyed series within the clip frame span.
fn validate_key_series(
    declared: usize,
    frames: &[u16],
    value_count: usize,
    clip_frames: f32,
    group: &str,
    param: &str,
) -> Result<(), DecodedBillboardAnimationError> {
    if declared == 0
        || declared != frames.len()
        || declared != value_count
        || frames.windows(2).any(|pair| match pair {
            [left, right] => left >= right,
            _ => false,
        })
        || frames.iter().any(|frame| f32::from(*frame) >= clip_frames)
    {
        return invalid_channel(group, param);
    }
    Ok(())
}

/// Validate retained interpolation metadata without assigning channel meaning.
fn validate_metadata(
    metadata: &[BillboardAnimationMetadataEvidence],
    group: &str,
    param: &str,
) -> Result<(), DecodedBillboardAnimationError> {
    if metadata.iter().any(|entry| {
        entry.kind != "interpolation_mode"
            || entry.version != 0
            || entry.mode > 1
    }) {
        return invalid_channel(group, param);
    }
    Ok(())
}

/// Reject duplicate group identities and ids across authored group lists.
fn validate_unique_groups(
    lists: &[BillboardAnimationGroupListEvidence],
) -> Result<(), DecodedBillboardAnimationError> {
    let mut identities = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for group in lists.iter().flat_map(|list| list.groups.iter()) {
        if !identities.insert(group.identity.as_str())
            || !ids.insert(group.group_id)
        {
            return Err(DecodedBillboardAnimationError::InvalidGroup(
                group.identity.clone(),
            ));
        }
    }
    Ok(())
}

/// Return the exact `FourCC` parameter carried by one validated channel.
fn channel_param(channel: &BillboardAnimationChannelEvidence) -> &str {
    match channel {
        BillboardAnimationChannelEvidence::Float1 { param, .. }
        | BillboardAnimationChannelEvidence::Float2 { param, .. }
        | BillboardAnimationChannelEvidence::Vector3 { param, .. }
        | BillboardAnimationChannelEvidence::Quaternion { param, .. }
        | BillboardAnimationChannelEvidence::Bool { param, .. }
        | BillboardAnimationChannelEvidence::Colour { param, .. } => param,
    }
}

/// Build one stable invalid-channel error.
fn invalid_channel<T>(
    group: &str,
    param: &str,
) -> Result<T, DecodedBillboardAnimationError> {
    Err(DecodedBillboardAnimationError::InvalidChannel {
        group: group.to_owned(),
        param: param.to_owned(),
    })
}

/// Remove fixed-width NUL padding without repairing authored text.
fn clean_identity(
    value: &str,
) -> Result<String, DecodedBillboardAnimationError> {
    let clean = value.trim_end_matches('\0');
    if clean.is_empty()
        || clean.contains('\0')
        || clean != clean.trim()
        || clean.chars().any(char::is_control)
    {
        return Err(DecodedBillboardAnimationError::InvalidIdentity(
            value.to_owned(),
        ));
    }
    Ok(clean.to_owned())
}

/// Strict decoded top-level animation document.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnimationDocument {
    /// Stable decoded schema identity.
    schema: String,
    /// Authored animation identity.
    name: String,
    /// Decoded source animation version.
    version: u32,
    /// Decoded `FourCC` animation family.
    #[serde(rename = "type")]
    animation_type: String,
    /// Authored frame span.
    frames: f32,
    /// Authored frame rate.
    frame_rate: f32,
    /// Authored cyclic flag.
    cyclic: u32,
    /// Platform-size records in source order.
    sizes: Vec<BillboardAnimationSizeEvidence>,
    /// Group-list records in source order.
    group_lists: Vec<GroupListDocument>,
    /// Unsupported top-level loose channels.
    loose_channels: Vec<Value>,
    /// Unsupported legacy top-level animation payloads.
    legacy_animation_extras: Vec<Value>,
}

/// Strict decoded BQG group-list document.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GroupListDocument {
    /// Decoded group-list version.
    version: u32,
    /// Declared number of groups.
    num_groups: usize,
    /// Child groups in source order.
    groups: Vec<GroupDocument>,
}

/// Strict decoded BQG group document.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GroupDocument {
    /// Decoded group version.
    version: u32,
    /// Authored target identity.
    name: String,
    /// Authored numeric group id.
    group_id: u32,
    /// Declared number of channels.
    num_channels: usize,
    /// Child channels in source order.
    channels: Vec<ChannelDocument>,
}

/// Strict decoded BQG channel shapes supported by current source evidence.
#[derive(Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
enum ChannelDocument {
    /// One-component floating channel.
    #[serde(rename = "float1")]
    Float1 {
        version: u32,
        param: String,
        num_frames: usize,
        frames: Vec<u16>,
        values: Vec<[f32; 1]>,
        channel_metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Two-component floating channel.
    #[serde(rename = "float2")]
    Float2 {
        version: u32,
        param: String,
        num_frames: usize,
        frames: Vec<u16>,
        values: Vec<[f32; 2]>,
        channel_metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Three-component vector channel.
    #[serde(rename = "vector3")]
    Vector3 {
        version: u32,
        param: String,
        num_frames: usize,
        frames: Vec<u16>,
        values: Vec<[f32; 3]>,
        channel_metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Four-component quaternion channel.
    #[serde(rename = "quaternion")]
    Quaternion {
        version: u32,
        param: String,
        num_frames: usize,
        frames: Vec<u16>,
        values: Vec<[f32; 4]>,
        channel_metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Boolean state channel.
    #[serde(rename = "bool")]
    Bool {
        version: u32,
        param: String,
        start_state: u32,
        num_frames: usize,
        values: Vec<u32>,
        channel_metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
    /// Packed colour channel.
    #[serde(rename = "colour")]
    Colour {
        version: u32,
        param: String,
        num_frames: usize,
        frames: Vec<u16>,
        values: Vec<u32>,
        channel_metadata: Vec<BillboardAnimationMetadataEvidence>,
    },
}

/// Decoded BQG evidence failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodedBillboardAnimationError {
    /// Source JSON file could not be read.
    Read {
        /// Portable path shown in diagnostics.
        path: String,
        /// Underlying filesystem failure.
        source: String,
    },
    /// Source JSON file could not be decoded strictly.
    Parse {
        /// Portable path shown in diagnostics.
        path: String,
        /// Underlying JSON failure.
        source: String,
    },
    /// Source schema, version, family, or size metadata is unsupported.
    UnsupportedDocument,
    /// Requested identity differs from the decoded authored identity.
    IdentityMismatch {
        /// Requested logical identity.
        requested: String,
        /// Decoded authored identity.
        decoded: String,
    },
    /// Authored identity cannot be preserved without repair.
    InvalidIdentity(String),
    /// Frame span, frame rate, or cyclic flag is invalid.
    InvalidTiming,
    /// Unsupported loose or legacy top-level payload was present.
    UnsupportedTopLevelPayload,
    /// Group-list version or declared group count is invalid.
    InvalidGroupList,
    /// Group version, count, identity uniqueness, or id uniqueness is invalid.
    InvalidGroup(String),
    /// Channel shape, semantics, count, key series, or metadata is invalid.
    InvalidChannel {
        /// Owning group identity.
        group: String,
        /// Authored channel parameter.
        param: String,
    },
}
