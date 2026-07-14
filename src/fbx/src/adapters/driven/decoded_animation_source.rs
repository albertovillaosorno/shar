// File:
//   - decoded_animation_source.rs
// Path:
//   - src/fbx/src/adapters/driven/decoded_animation_source.rs
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
//   - Typed reconstruction of decoded PTRN skeletal animation components.
// - Must-Not:
//   - Discover packages, bind helper groups as bones, or emit FBX objects.
// - Allows:
//   - JSON decoding, source-channel interpolation, and domain clip assembly.
// - Split-When:
//   - Texture-name or non-transform channels require conversion contracts.
// - Merge-When:
//   - Another decoded animation adapter owns the same source schema.
// - Summary:
//   - Converts decoded Pure3D skeletal channels into sampled domain clips.
// - Description:
//   - Reconstructs compact vectors and signed WXYZ quaternion channels.
// - Usage:
//   - Called with index-published animation paths and an exported skeleton.
// - Defaults:
//   - Samples every integer source frame and preserves helper-group evidence.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: Source schema records, channel interpolation, and clip assembly
//   - form one decoded-animation boundary; split when non-transform channels
//   - become convertible output rather than preserved capability evidence.
//

//! Converts decoded Pure3D skeletal channels into sampled domain clips.

// Channel widths and mappings are validated before fixed-array access. Frame
// conversions are bounded by finite integral checks, while explicit linear
// interpolation formulas remain visible for source-runtime parity review.
#![expect(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::default_numeric_fallback,
    clippy::indexing_slicing,
    clippy::suboptimal_flops,
    reason = "Validated channel math preserves explicit source interpolation \
              and axis mapping."
)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;
use serde::Deserialize;

use crate::domain::animation::quaternion::{
    Error as QuaternionError, decode_signed_i16_wxyz, from_row_matrix,
    normalize, slerp,
};
use crate::domain::animation::{
    AnimationClip, AnimationClipError, BoneAnimationTrack, LocalTransformSample,
};
use crate::domain::skeleton::Bone;
use crate::domain::transform::matrix::widen;

/// Decoded animation adapter failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodedAnimationError {
    /// Component file could not be read.
    Read {
        /// Component path.
        path: String,
        /// IO error text.
        source: String,
    },
    /// Component JSON could not be parsed.
    Parse {
        /// Component path.
        path: String,
        /// JSON error text.
        source: String,
    },
    /// Decoded schema was not `animation`.
    UnsupportedSchema(String),
    /// Decoded animation was not a skeletal PTRN clip.
    UnsupportedAnimationType(String),
    /// Fixed-width identity padding appeared before the end of an identity.
    InvalidIdentityPadding,
    /// Decoded identity contained a non-printing control character.
    InvalidIdentityCharacter,
    /// Clip frame count was non-integral, non-finite, or out of range.
    InvalidFrameCount,
    /// One group list declared a count different from its decoded groups.
    InvalidGroupCount {
        /// Count declared by the decoded source.
        declared: usize,
        /// Number of decoded group records.
        actual: usize,
    },
    /// One group declared a count different from its decoded channels.
    InvalidChannelCount {
        /// Group containing the contradictory declaration.
        group: String,
        /// Count declared by the decoded source.
        declared: usize,
        /// Number of decoded channel records.
        actual: usize,
    },
    /// One channel declared a count different from its decoded key frames.
    InvalidKeyCount {
        /// Group containing the contradictory channel.
        group: String,
        /// Transform parameter carried by the channel.
        parameter: String,
        /// Count declared by the decoded source.
        declared: usize,
        /// Number of decoded frame indices.
        actual: usize,
    },
    /// One decoded group identity appeared more than once.
    DuplicateGroup(String),
    /// One transform parameter appeared more than once in a group.
    DuplicateChannel {
        /// Group containing the duplicate channel.
        group: String,
        /// Repeated channel parameter.
        parameter: String,
    },
    /// One channel kind was unsupported for its transform parameter.
    UnsupportedChannelKind {
        /// Channel parameter.
        parameter: String,
        /// Decoded channel kind.
        kind: String,
    },
    /// One compact vector mapping was missing or outside X/Y/Z.
    InvalidVectorMapping,
    /// One compact vector omitted its three constant components.
    MissingVectorConstants,
    /// Key frames and values had inconsistent lengths or invalid ordering.
    InvalidKeySeries,
    /// One key value had the wrong component width.
    InvalidValueWidth,
    /// One interpolation metadata value was unsupported or duplicated.
    InvalidInterpolationMode,
    /// Quaternion reconstruction failed.
    Quaternion(QuaternionError),
    /// Domain clip validation failed.
    Clip(AnimationClipError),
}

/// Decoded top-level animation component.
#[derive(Deserialize)]
struct DecodedAnimation {
    /// Decoded component schema marker.
    schema: String,
    /// Fixed-width source clip identity.
    name: String,
    /// `FourCC` animation family marker.
    #[serde(rename = "type")]
    animation_type: String,
    /// Declared source-frame sample count.
    frames: f64,
    /// Declared source frames per second.
    frame_rate: f64,
    /// Nonzero when the source clip loops.
    cyclic: u32,
    /// Group-list containers in decoded source order.
    #[serde(default)]
    group_lists: Vec<DecodedGroupList>,
}

/// Decoded group-list container.
#[derive(Deserialize)]
struct DecodedGroupList {
    /// Number of groups declared by the decoded source.
    #[serde(rename = "num_groups")]
    group_count: usize,
    /// Target groups in decoded source order.
    #[serde(default)]
    groups: Vec<DecodedGroup>,
}

/// One decoded animation target group.
#[derive(Deserialize)]
struct DecodedGroup {
    /// Fixed-width target group identity.
    name: String,
    /// Number of channels declared by the decoded source.
    #[serde(rename = "num_channels")]
    channel_count: usize,
    /// Transform channels owned by this target group.
    #[serde(default)]
    channels: Vec<DecodedChannel>,
}

/// One decoded transform channel.
#[derive(Deserialize)]
struct DecodedChannel {
    /// Decoded channel representation family.
    kind: String,
    /// `FourCC` target property marker.
    param: String,
    /// Compact-vector axis mapping when present.
    #[serde(default)]
    mapping: Option<u16>,
    /// Compact-vector constant X, Y, and Z components.
    #[serde(default)]
    constants: Option<[f64; 3]>,
    /// Number of key frames declared by the decoded source.
    #[serde(rename = "num_frames")]
    key_count: usize,
    /// Strictly increasing integer source frames.
    #[serde(default)]
    frames: Vec<u16>,
    /// Uncompressed scalar, vector, or quaternion values.
    #[serde(default)]
    values: Vec<Vec<f64>>,
    /// Signed-normalized quaternion words in WXYZ order.
    #[serde(default)]
    compressed_values: Vec<[u16; 4]>,
    /// Channel interpolation and future metadata records.
    #[serde(default)]
    channel_metadata: Vec<DecodedChannelMetadata>,
}

/// Decoded interpolation metadata.
#[derive(Deserialize)]
struct DecodedChannelMetadata {
    /// Metadata record family.
    kind: String,
    /// Source interpolation-mode value.
    mode: u32,
}

/// Load skeletal PTRN clips from exact index-published component paths.
///
/// # Errors
///
/// Returns an error when a file, schema, channel series, skeleton binding, or
/// domain clip violates the decoded-animation contract.
pub fn load_animation_clips(
    paths: &[&Path],
    bones: &[Bone],
) -> Result<Vec<AnimationClip>, DecodedAnimationError> {
    let rest_transforms = rest_transforms(bones)?;
    paths
        .iter()
        .map(
            |path| {
                load_clip(
                    path,
                    bones,
                    &rest_transforms,
                )
            },
        )
        .collect()
}

/// Load and bind one decoded clip.
fn load_clip(
    path: &Path,
    bones: &[Bone],
    rest_transforms: &BTreeMap<String, LocalTransformSample>,
) -> Result<AnimationClip, DecodedAnimationError> {
    let decoded: DecodedAnimation = read_json(path)?;
    if decoded.schema != "animation" {
        return Err(DecodedAnimationError::UnsupportedSchema(decoded.schema));
    }
    if decoded.animation_type != "PTRN" {
        return Err(
            DecodedAnimationError::UnsupportedAnimationType(
                decoded.animation_type,
            ),
        );
    }
    let frame_count = frame_count(decoded.frames)?;
    let groups = decoded_groups(decoded.group_lists)?;
    let bone_ids: BTreeSet<&str> = bones
        .iter()
        .map(
            |bone| {
                bone.id
                    .as_str()
            },
        )
        .collect();
    let ignored_group_ids = groups
        .keys()
        .filter(|group| !bone_ids.contains(group.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let mut tracks = Vec::new();
    for bone in bones {
        let Some(group) = groups.get(&bone.id) else {
            continue;
        };
        let rest = *rest_transforms
            .get(&bone.id)
            .ok_or(DecodedAnimationError::InvalidKeySeries)?;
        tracks.push(
            sample_track(
                bone,
                group,
                rest,
                frame_count,
            )?,
        );
    }
    AnimationClip::new(
        trim_identity(&decoded.name)?,
        decoded.frame_rate,
        decoded.cyclic != 0,
        frame_count,
        tracks,
        ignored_group_ids,
    )
    .map_err(DecodedAnimationError::Clip)
}

/// Convert decoded groups into a unique canonical identity map.
fn decoded_groups(
    group_lists: Vec<DecodedGroupList>
) -> Result<BTreeMap<String, DecodedGroup>, DecodedAnimationError> {
    let mut groups = BTreeMap::new();
    for list in group_lists {
        let actual = list
            .groups
            .len();
        if list.group_count != actual {
            return Err(
                DecodedAnimationError::InvalidGroupCount {
                    declared: list.group_count,
                    actual,
                },
            );
        }
        for group in list.groups {
            let identity = trim_identity(&group.name)?;
            let actual_channels = group
                .channels
                .len();
            if group.channel_count != actual_channels {
                return Err(
                    DecodedAnimationError::InvalidChannelCount {
                        group: identity,
                        declared: group.channel_count,
                        actual: actual_channels,
                    },
                );
            }
            for channel in &group.channels {
                let actual_keys = channel
                    .frames
                    .len();
                if channel.key_count != actual_keys {
                    return Err(
                        DecodedAnimationError::InvalidKeyCount {
                            group: identity,
                            parameter: channel
                                .param
                                .clone(),
                            declared: channel.key_count,
                            actual: actual_keys,
                        },
                    );
                }
            }
            if groups
                .insert(
                    identity.clone(),
                    group,
                )
                .is_some()
            {
                return Err(DecodedAnimationError::DuplicateGroup(identity));
            }
        }
    }
    Ok(groups)
}

/// Build one sampled bone track from matching decoded channels.
fn sample_track(
    bone: &Bone,
    group: &DecodedGroup,
    rest: LocalTransformSample,
    frame_count: usize,
) -> Result<BoneAnimationTrack, DecodedAnimationError> {
    let translation = unique_channel(
        group, "TRAN",
    )?;
    let rotation = unique_channel(
        group, "ROT_",
    )?;
    let mut samples = Vec::with_capacity(frame_count);
    for frame in 0..frame_count {
        let translation_value = translation.map_or(
            Ok(rest.translation),
            |channel| {
                sample_translation(
                    channel, frame,
                )
            },
        )?;
        let rotation_value = rotation.map_or(
            Ok(rest.rotation_wxyz),
            |channel| {
                sample_rotation(
                    channel, frame,
                )
            },
        )?;
        samples.push(
            LocalTransformSample {
                translation: translation_value,
                rotation_wxyz: rotation_value,
            },
        );
    }
    Ok(
        BoneAnimationTrack {
            bone_id: bone
                .id
                .clone(),
            samples,
        },
    )
}

/// Return a unique channel for one group parameter.
fn unique_channel<'group>(
    group: &'group DecodedGroup,
    parameter: &str,
) -> Result<Option<&'group DecodedChannel>, DecodedAnimationError> {
    let mut channels = group
        .channels
        .iter()
        .filter(|channel| channel.param == parameter);
    let channel = channels.next();
    if channels
        .next()
        .is_some()
    {
        return Err(
            DecodedAnimationError::DuplicateChannel {
                group: trim_identity(&group.name)?,
                parameter: parameter.to_owned(),
            },
        );
    }
    Ok(channel)
}

/// Sample one decoded translation channel at an integer source frame.
fn sample_translation(
    channel: &DecodedChannel,
    frame: usize,
) -> Result<[f64; 3], DecodedAnimationError> {
    match channel
        .kind
        .as_str()
    {
        "vector1" => sample_vector1(
            channel, frame,
        ),
        "vector2" => sample_vector2(
            channel, frame,
        ),
        "vector3" => sample_vector3(
            channel, frame,
        ),
        kind => Err(
            DecodedAnimationError::UnsupportedChannelKind {
                parameter: channel
                    .param
                    .clone(),
                kind: kind.to_owned(),
            },
        ),
    }
}

/// Sample one decoded rotation channel at an integer source frame.
fn sample_rotation(
    channel: &DecodedChannel,
    frame: usize,
) -> Result<[f64; 4], DecodedAnimationError> {
    let interpolate = interpolation_enabled(channel)?;
    match channel
        .kind
        .as_str()
    {
        "compressed_quaternion" => {
            ensure_key_series(
                &channel.frames,
                channel
                    .compressed_values
                    .len(),
            )?;
            let values = channel
                .compressed_values
                .iter()
                .copied()
                .map(decode_signed_i16_wxyz)
                .collect::<Result<Vec<_>, _>>()
                .map_err(DecodedAnimationError::Quaternion)?;
            sample_quaternion_series(
                &channel.frames,
                &values,
                frame,
                interpolate,
            )
        }
        "quaternion" => {
            ensure_key_series(
                &channel.frames,
                channel
                    .values
                    .len(),
            )?;
            let values = channel
                .values
                .iter()
                .map(
                    |value| {
                        if value.len() != 4 {
                            return Err(
                                DecodedAnimationError::InvalidValueWidth,
                            );
                        }
                        normalize(
                            [
                                value[0], value[1], value[2], value[3],
                            ],
                        )
                        .map_err(DecodedAnimationError::Quaternion)
                    },
                )
                .collect::<Result<Vec<_>, _>>()?;
            sample_quaternion_series(
                &channel.frames,
                &values,
                frame,
                interpolate,
            )
        }
        kind => Err(
            DecodedAnimationError::UnsupportedChannelKind {
                parameter: channel
                    .param
                    .clone(),
                kind: kind.to_owned(),
            },
        ),
    }
}

/// Sample one one-degree-of-freedom vector channel.
fn sample_vector1(
    channel: &DecodedChannel,
    frame: usize,
) -> Result<[f64; 3], DecodedAnimationError> {
    let mapping = mapping_index(channel)?;
    let mut result = channel
        .constants
        .ok_or(DecodedAnimationError::MissingVectorConstants)?;
    let values = scalar_values(channel)?;
    result[mapping] = sample_scalar_series(
        &channel.frames,
        &values,
        frame,
        interpolation_enabled(channel)?,
    )?;
    Ok(result)
}

/// Sample one two-degree-of-freedom vector channel.
fn sample_vector2(
    channel: &DecodedChannel,
    frame: usize,
) -> Result<[f64; 3], DecodedAnimationError> {
    let static_index = mapping_index(channel)?;
    let mut result = channel
        .constants
        .ok_or(DecodedAnimationError::MissingVectorConstants)?;
    ensure_key_series(
        &channel.frames,
        channel
            .values
            .len(),
    )?;
    let values = channel
        .values
        .iter()
        .map(
            |value| {
                if value.len() != 2 {
                    return Err(DecodedAnimationError::InvalidValueWidth);
                }
                Ok(
                    [
                        value[0], value[1],
                    ],
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let sampled = sample_vector2_series(
        &channel.frames,
        &values,
        frame,
        interpolation_enabled(channel)?,
    )?;
    let dynamic_indices = match static_index {
        0 => [
            1, 2,
        ],
        1 => [
            0, 2,
        ],
        2 => [
            0, 1,
        ],
        _ => return Err(DecodedAnimationError::InvalidVectorMapping),
    };
    result[dynamic_indices[0]] = sampled[0];
    result[dynamic_indices[1]] = sampled[1];
    Ok(result)
}

/// Sample one full three-degree-of-freedom vector channel.
fn sample_vector3(
    channel: &DecodedChannel,
    frame: usize,
) -> Result<[f64; 3], DecodedAnimationError> {
    ensure_key_series(
        &channel.frames,
        channel
            .values
            .len(),
    )?;
    let values = channel
        .values
        .iter()
        .map(
            |value| {
                if value.len() != 3 {
                    return Err(DecodedAnimationError::InvalidValueWidth);
                }
                Ok(
                    [
                        value[0], value[1], value[2],
                    ],
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    sample_vector3_series(
        &channel.frames,
        &values,
        frame,
        interpolation_enabled(channel)?,
    )
}

/// Extract scalar values from one one-DOF channel.
fn scalar_values(
    channel: &DecodedChannel
) -> Result<Vec<f64>, DecodedAnimationError> {
    ensure_key_series(
        &channel.frames,
        channel
            .values
            .len(),
    )?;
    channel
        .values
        .iter()
        .map(
            |value| {
                value
                    .first()
                    .copied()
                    .filter(|_| value.len() == 1)
                    .ok_or(DecodedAnimationError::InvalidValueWidth)
            },
        )
        .collect()
}

/// Resolve one compact-vector mapping into an axis index.
fn mapping_index(
    channel: &DecodedChannel
) -> Result<usize, DecodedAnimationError> {
    let mapping = channel
        .mapping
        .ok_or(DecodedAnimationError::InvalidVectorMapping)?;
    let index = usize::from(mapping);
    if index > 2 {
        return Err(DecodedAnimationError::InvalidVectorMapping);
    }
    Ok(index)
}

/// Return whether one channel interpolates between keys.
fn interpolation_enabled(
    channel: &DecodedChannel
) -> Result<bool, DecodedAnimationError> {
    let modes = channel
        .channel_metadata
        .iter()
        .filter(|metadata| metadata.kind == "interpolation_mode")
        .map(|metadata| metadata.mode)
        .collect::<Vec<_>>();
    match modes.as_slice() {
        [] | [1] => Ok(true),
        [0] => Ok(false),
        _ => Err(DecodedAnimationError::InvalidInterpolationMode),
    }
}

/// Ensure one frame/value series is non-empty and strictly increasing.
fn ensure_key_series(
    frames: &[u16],
    value_count: usize,
) -> Result<(), DecodedAnimationError> {
    if frames.is_empty() || frames.len() != value_count {
        return Err(DecodedAnimationError::InvalidKeySeries);
    }
    if frames
        .windows(2)
        .any(|pair| pair[0] >= pair[1])
    {
        return Err(DecodedAnimationError::InvalidKeySeries);
    }
    Ok(())
}

/// Locate source keys surrounding one integer frame.
fn bracket(
    frames: &[u16],
    frame: usize,
) -> Result<
    (
        usize,
        usize,
        f64,
    ),
    DecodedAnimationError,
> {
    let first = usize::from(
        *frames
            .first()
            .ok_or(DecodedAnimationError::InvalidKeySeries)?,
    );
    if frame <= first {
        return Ok(
            (
                0, 0, 0.0,
            ),
        );
    }
    let last_index = frames
        .len()
        .checked_sub(1)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    let last_frame = usize::from(frames[last_index]);
    if frame >= last_frame {
        return Ok(
            (
                last_index, last_index, 0.0,
            ),
        );
    }
    for (start, pair) in frames
        .windows(2)
        .enumerate()
    {
        let start_frame = usize::from(pair[0]);
        let end_frame = usize::from(pair[1]);
        if frame >= start_frame && frame <= end_frame {
            let span = end_frame
                .checked_sub(start_frame)
                .ok_or(DecodedAnimationError::InvalidKeySeries)?;
            let elapsed = frame
                .checked_sub(start_frame)
                .ok_or(DecodedAnimationError::InvalidKeySeries)?;
            return Ok(
                (
                    start,
                    start + 1,
                    elapsed as f64 / span as f64,
                ),
            );
        }
    }
    Err(DecodedAnimationError::InvalidKeySeries)
}

/// Sample one scalar key series.
fn sample_scalar_series(
    frames: &[u16],
    values: &[f64],
    frame: usize,
    interpolate: bool,
) -> Result<f64, DecodedAnimationError> {
    let (start, end, amount) = bracket(
        frames, frame,
    )?;
    if start == end || !interpolate {
        return values
            .get(start)
            .copied()
            .ok_or(DecodedAnimationError::InvalidKeySeries);
    }
    let start_value = *values
        .get(start)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    let end_value = *values
        .get(end)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    Ok(start_value + (end_value - start_value) * amount)
}

/// Sample one two-component key series.
fn sample_vector2_series(
    frames: &[u16],
    values: &[[f64; 2]],
    frame: usize,
    interpolate: bool,
) -> Result<[f64; 2], DecodedAnimationError> {
    let (start, end, amount) = bracket(
        frames, frame,
    )?;
    let start_value = *values
        .get(start)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    if start == end || !interpolate {
        return Ok(start_value);
    }
    let end_value = *values
        .get(end)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    Ok(
        [
            start_value[0] + (end_value[0] - start_value[0]) * amount,
            start_value[1] + (end_value[1] - start_value[1]) * amount,
        ],
    )
}

/// Sample one three-component key series.
fn sample_vector3_series(
    frames: &[u16],
    values: &[[f64; 3]],
    frame: usize,
    interpolate: bool,
) -> Result<[f64; 3], DecodedAnimationError> {
    let (start, end, amount) = bracket(
        frames, frame,
    )?;
    let start_value = *values
        .get(start)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    if start == end || !interpolate {
        return Ok(start_value);
    }
    let end_value = *values
        .get(end)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    Ok(
        [
            start_value[0] + (end_value[0] - start_value[0]) * amount,
            start_value[1] + (end_value[1] - start_value[1]) * amount,
            start_value[2] + (end_value[2] - start_value[2]) * amount,
        ],
    )
}

/// Sample one quaternion key series.
fn sample_quaternion_series(
    frames: &[u16],
    values: &[[f64; 4]],
    frame: usize,
    interpolate: bool,
) -> Result<[f64; 4], DecodedAnimationError> {
    let (start, end, amount) = bracket(
        frames, frame,
    )?;
    let start_value = *values
        .get(start)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    if start == end || !interpolate {
        return Ok(start_value);
    }
    let end_value = *values
        .get(end)
        .ok_or(DecodedAnimationError::InvalidKeySeries)?;
    Ok(
        slerp(
            start_value,
            end_value,
            amount,
        ),
    )
}

/// Convert skeleton rest matrices into local transform defaults.
fn rest_transforms(
    bones: &[Bone]
) -> Result<BTreeMap<String, LocalTransformSample>, DecodedAnimationError> {
    bones
        .iter()
        .map(
            |bone| {
                let matrix = widen(&bone.rest_matrix);
                let rotation_wxyz = from_row_matrix(&matrix)
                    .map_err(DecodedAnimationError::Quaternion)?;
                Ok(
                    (
                        bone.id
                            .clone(),
                        LocalTransformSample {
                            translation: [
                                matrix[12], matrix[13], matrix[14],
                            ],
                            rotation_wxyz,
                        },
                    ),
                )
            },
        )
        .collect()
}

/// Validate and convert a decoded floating frame count.
fn frame_count(value: f64) -> Result<usize, DecodedAnimationError> {
    if !value.is_finite()
        || value < 1.0
        || value
            .fract()
            .abs()
            > 0.0_f64
    {
        return Err(DecodedAnimationError::InvalidFrameCount);
    }
    let exclusive_max = usize::MAX as f64 + 1.0_f64;
    if value >= exclusive_max {
        return Err(DecodedAnimationError::InvalidFrameCount);
    }
    Ok(value as usize)
}

/// Trim fixed-width decoded identity padding and surrounding whitespace.
fn trim_identity(value: &str) -> Result<String, DecodedAnimationError> {
    let without_padding = value.trim_end_matches('\0');
    if without_padding.contains('\0') {
        return Err(DecodedAnimationError::InvalidIdentityPadding);
    }
    if without_padding
        .chars()
        .any(char::is_control)
    {
        return Err(DecodedAnimationError::InvalidIdentityCharacter);
    }
    Ok(
        without_padding
            .trim()
            .to_owned(),
    )
}

/// Read one strict decoded JSON component.
fn read_json<T>(path: &Path) -> Result<T, DecodedAnimationError>
where
    T: for<'de> Deserialize<'de>,
{
    let text = local::read_utf8(path).map_err(
        |source| DecodedAnimationError::Read {
            path: path
                .display()
                .to_string(),
            source: source.to_string(),
        },
    )?;
    let json_text = text
        .strip_prefix('\u{feff}')
        .unwrap_or(&text);
    serde_json::from_str(json_text).map_err(
        |source| DecodedAnimationError::Parse {
            path: path
                .display()
                .to_string(),
            source: source.to_string(),
        },
    )
}

#[cfg(test)]
#[test]
fn rejects_first_unrepresentable_frame_count() {
    let exclusive_max = usize::MAX as f64 + 1.0_f64;

    assert_eq!(
        frame_count(exclusive_max),
        Err(DecodedAnimationError::InvalidFrameCount)
    );
}

#[cfg(test)]
#[test]
fn rejects_epsilon_fractional_frame_count() {
    assert_eq!(
        frame_count(1.0_f64 + f64::EPSILON),
        Err(DecodedAnimationError::InvalidFrameCount)
    );
}

#[cfg(test)]
#[test]
fn rejects_declared_group_count_mismatch() {
    let error = decoded_groups(
        vec![
            DecodedGroupList {
                group_count: 1,
                groups: Vec::new(),
            },
        ],
    )
    .err();

    assert_eq!(
        error,
        Some(
            DecodedAnimationError::InvalidGroupCount {
                declared: 1,
                actual: 0,
            }
        )
    );
}

#[cfg(test)]
#[test]
fn rejects_declared_channel_count_mismatch() {
    let error = decoded_groups(
        vec![
            DecodedGroupList {
                group_count: 1,
                groups: vec![
                    DecodedGroup {
                        name: "Root".to_owned(),
                        channel_count: 1,
                        channels: Vec::new(),
                    },
                ],
            },
        ],
    )
    .err();

    assert_eq!(
        error,
        Some(
            DecodedAnimationError::InvalidChannelCount {
                group: "Root".to_owned(),
                declared: 1,
                actual: 0,
            }
        )
    );
}

#[cfg(test)]
#[test]
fn rejects_declared_key_count_mismatch() {
    let error = decoded_groups(
        vec![
            DecodedGroupList {
                group_count: 1,
                groups: vec![
                    DecodedGroup {
                        name: "Root".to_owned(),
                        channel_count: 1,
                        channels: vec![
                            DecodedChannel {
                                kind: "vector1".to_owned(),
                                param: "TRAN".to_owned(),
                                mapping: None,
                                constants: None,
                                key_count: 1,
                                frames: Vec::new(),
                                values: Vec::new(),
                                compressed_values: Vec::new(),
                                channel_metadata: Vec::new(),
                            },
                        ],
                    },
                ],
            },
        ],
    )
    .err();

    assert_eq!(
        error,
        Some(
            DecodedAnimationError::InvalidKeyCount {
                group: "Root".to_owned(),
                parameter: "TRAN".to_owned(),
                declared: 1,
                actual: 0,
            }
        )
    );
}

#[cfg(test)]
#[test]
fn rejects_embedded_identity_padding() {
    assert_eq!(
        trim_identity("Root\0Alias"),
        Err(DecodedAnimationError::InvalidIdentityPadding)
    );
}

#[cfg(test)]
#[test]
fn rejects_identity_control_characters() {
    assert_eq!(
        trim_identity("Root\nAlias"),
        Err(DecodedAnimationError::InvalidIdentityCharacter)
    );
}
