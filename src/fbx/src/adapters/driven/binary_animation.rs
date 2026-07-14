// File:
//   - binary_animation.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_animation.rs
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
//   - Standard FBX 7.7 animation objects, curves, connections, and takes.
// - Must-Not:
//   - Decode source files, resolve packages, or alter skeleton bind transforms.
// - Allows:
//   - Deterministic animation ids, KTime conversion, and Euler curve emission.
// - Split-When:
//   - Non-transform animation channels gain an independent FBX object family.
// - Merge-When:
//   - The binary character writer owns the same animation object graph.
// - Summary:
//   - Builds one deterministic FBX animation plan for sampled skeletal clips.
// - Description:
//   - Emits stacks, layers, curve nodes, XYZ curves, links, and take metadata.
// - Usage:
//   - Consumed by the binary character writer before document encoding.
// - Defaults:
//   - One layer per clip and one translation and rotation node per bone track.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: Standard animation objects, ids, curves, connections, and takes
//   - share one deterministic graph; split when another animation object family
//   - is added independently of skeletal transform curves.
//

//! Standard FBX 7.7 animation object and connection planning.
//!
//! The plan allocates stable ids and emits stacks, layers, transform curves,
//! and takes without changing source clip or skeleton semantics.

// Reserved object-id ranges, bounded KTime conversion, fixed XYZ arrays, and
// Euler continuity are validated by domain construction and deterministic
// Blender import regressions before this serialization boundary is accepted.
#![expect(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::default_numeric_fallback,
    clippy::doc_markdown,
    clippy::indexing_slicing,
    clippy::map_err_ignore,
    clippy::missing_const_for_fn,
    clippy::shadow_reuse,
    clippy::too_many_lines,
    clippy::while_float,
    reason = "Checked FBX animation serialization preserves explicit \
              deterministic numeric contracts."
)]

use std::collections::BTreeMap;

use super::binary_fbx::{BinaryNode, BinaryProperty};
use super::binary_identity::{BinaryIdentityError, bone_ids};
use crate::domain::animation::AnimationClip;
use crate::domain::animation::clip::frame_rates_match;
use crate::domain::animation::quaternion::to_row_matrix;
use crate::domain::transform::matrix::{MatrixError, decompose};

/// FBX 7.x time units in one second.
const KTIME_PER_SECOND: f64 = 46_186_158_000.0;
/// Exclusive positive bound for a signed 64-bit FBX time field.
const I64_EXCLUSIVE_MAX: f64 = 9_223_372_036_854_775_808.0;
/// FBX animation key record version.
const ANIMATION_KEY_VERSION: i32 = 4_009;
/// Deterministic object-id base reserved for animation objects.
const ANIMATION_ID_BASE: u64 = 7_000_000_000;
/// Collision-free object-id range reserved for each clip.
const CLIP_ID_STRIDE: u64 = 1_000_000;
/// First per-track object offset inside one clip range.
const TRACK_ID_OFFSET: u64 = 100;
/// Collision-free object-id range reserved for each bone track.
const TRACK_ID_STRIDE: u64 = 16;
/// Linear key flag with deterministic tangent metadata.
const LINEAR_KEY_FLAGS: i32 = (1 << 2) | (1 << 10);

/// Counts for FBX animation object definitions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct BinaryAnimationCounts {
    /// Animation stack objects.
    pub(super) stacks: usize,
    /// Animation layer objects.
    pub(super) layers: usize,
    /// Animation curve-node objects.
    pub(super) curve_nodes: usize,
    /// Animation curve objects.
    pub(super) curves: usize,
}

/// Complete deterministic animation contribution to one FBX document.
pub(super) struct BinaryAnimationPlan {
    /// Definition counts for emitted object families.
    pub(super) counts: BinaryAnimationCounts,
    /// Animation objects appended to the Objects section.
    pub(super) objects: Vec<BinaryNode>,
    /// Animation links appended to the Connections section.
    pub(super) connections: Vec<BinaryNode>,
    /// Complete top-level Takes section.
    pub(super) takes: BinaryNode,
    /// Greatest animation stop time in FBX KTime units.
    pub(super) max_stop_time: i64,
    /// Shared source frame rate when clips are present.
    pub(super) frame_rate: Option<f64>,
    /// First deterministic clip selected as the active animation stack.
    pub(super) active_stack_name: String,
}

/// Binary animation planning failure.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum BinaryAnimationError {
    /// Deterministic object-id arithmetic overflowed its reserved range.
    IdOverflow,
    /// Object id could not fit the signed FBX integer field.
    SignedIdOverflow(u64),
    /// Bone track did not reference an exported skeleton bone.
    UnknownBone(String),
    /// Two clips used frame rates that cannot share one FBX scene setting.
    MixedFrameRate,
    /// Frame-time conversion overflowed the signed FBX time field.
    TimeOverflow,
    /// Source frames were closer together than one FBX KTime tick.
    TimeResolution,
    /// One curve value could not fit the FBX 32-bit float representation.
    ValueOverflow,
    /// Quaternion-to-Euler conversion rejected one sample matrix.
    Matrix(MatrixError),
    /// Shared character object-id derivation failed.
    Identity(BinaryIdentityError),
}

/// Build all animation objects, links, and takes for sampled skeletal clips.
///
/// # Errors
///
/// Returns an error when object ids, frame timing, bone targets, or transform
/// conversion cannot be represented by the deterministic FBX 7.7 contract.
pub(super) fn build_animation_plan(
    clips: &[AnimationClip],
    bone_ordinals: &BTreeMap<&str, usize>,
) -> Result<BinaryAnimationPlan, BinaryAnimationError> {
    let frame_rate = shared_frame_rate(clips)?;
    let mut active_stack_name = String::new();
    if let Some(clip) = clips.first() {
        active_stack_name.clone_from(&clip.name);
    }
    let mut objects = Vec::new();
    let mut connections = Vec::new();
    let mut take_nodes = vec![
        string_node(
            "Current",
            &active_stack_name,
        ),
    ];
    let mut counts = BinaryAnimationCounts::default();
    let mut max_stop_time = 0_i64;
    for (clip_ordinal, clip) in clips
        .iter()
        .enumerate()
    {
        let clip_ids = clip_ids(clip_ordinal)?;
        let final_frame = clip
            .frame_count
            .checked_sub(1)
            .ok_or(BinaryAnimationError::TimeOverflow)?;
        let stop_time = frame_time(
            final_frame,
            clip.frame_rate,
        )?;
        max_stop_time = max_stop_time.max(stop_time);
        objects.push(
            animation_stack_node(
                clip_ids.stack,
                clip,
                stop_time,
            )?,
        );
        objects.push(animation_layer_node(clip_ids.layer)?);
        connections.push(
            object_connection(
                clip_ids.layer,
                clip_ids.stack,
            )?,
        );
        counts.stacks += 1;
        counts.layers += 1;
        for (track_ordinal, track) in clip
            .tracks
            .iter()
            .enumerate()
        {
            let bone_ordinal = *bone_ordinals
                .get(
                    track
                        .bone_id
                        .as_str(),
                )
                .ok_or_else(
                    || {
                        BinaryAnimationError::UnknownBone(
                            track
                                .bone_id
                                .clone(),
                        )
                    },
                )?;
            let bone_model = bone_ids(bone_ordinal)
                .map_err(BinaryAnimationError::Identity)?
                .model;
            let ids = track_ids(
                clip_ordinal,
                track_ordinal,
            )?;
            let times = key_times(clip)?;
            let translations = translation_values(
                clip,
                track_ordinal,
            )?;
            let rotations = rotation_eulers(
                clip,
                track_ordinal,
            )?;
            append_transform_animation(
                &mut objects,
                &mut connections,
                &mut counts,
                ids.translation,
                ids.translation_curves,
                "T",
                "Lcl Translation",
                bone_model,
                &times,
                &translations,
            )?;
            append_transform_animation(
                &mut objects,
                &mut connections,
                &mut counts,
                ids.rotation,
                ids.rotation_curves,
                "R",
                "Lcl Rotation",
                bone_model,
                &times,
                &rotations,
            )?;
            connections.push(
                object_connection(
                    ids.translation,
                    clip_ids.layer,
                )?,
            );
            connections.push(
                object_connection(
                    ids.rotation,
                    clip_ids.layer,
                )?,
            );
        }
        take_nodes.push(
            take_node(
                clip, stop_time,
            ),
        );
    }
    Ok(
        BinaryAnimationPlan {
            counts,
            objects,
            connections,
            takes: BinaryNode::branch(
                "Takes", take_nodes,
            ),
            max_stop_time,
            frame_rate,
            active_stack_name,
        },
    )
}

/// Deterministic ids for one animation stack and layer.
struct ClipIds {
    /// Animation stack object id.
    stack: u64,
    /// Animation layer object id.
    layer: u64,
}

/// Deterministic ids for one bone's translation and rotation curves.
struct TrackIds {
    /// Translation curve-node id.
    translation: u64,
    /// Translation X, Y, and Z curve ids.
    translation_curves: [u64; 3],
    /// Rotation curve-node id.
    rotation: u64,
    /// Rotation X, Y, and Z curve ids.
    rotation_curves: [u64; 3],
}

/// Derive deterministic ids for one clip ordinal.
fn clip_ids(ordinal: usize) -> Result<ClipIds, BinaryAnimationError> {
    let ordinal =
        u64::try_from(ordinal).map_err(|_| BinaryAnimationError::IdOverflow)?;
    let base = ordinal
        .checked_mul(CLIP_ID_STRIDE)
        .and_then(|offset| ANIMATION_ID_BASE.checked_add(offset))
        .ok_or(BinaryAnimationError::IdOverflow)?;
    Ok(
        ClipIds {
            stack: base
                .checked_add(1)
                .ok_or(BinaryAnimationError::IdOverflow)?,
            layer: base
                .checked_add(2)
                .ok_or(BinaryAnimationError::IdOverflow)?,
        },
    )
}

/// Derive deterministic ids for one clip-and-track pair.
fn track_ids(
    clip_ordinal: usize,
    track_ordinal: usize,
) -> Result<TrackIds, BinaryAnimationError> {
    let clip = clip_ids(clip_ordinal)?;
    let clip_base = clip
        .stack
        .checked_sub(1)
        .ok_or(BinaryAnimationError::IdOverflow)?;
    let track = u64::try_from(track_ordinal)
        .map_err(|_| BinaryAnimationError::IdOverflow)?;
    let base = track
        .checked_mul(TRACK_ID_STRIDE)
        .and_then(|offset| TRACK_ID_OFFSET.checked_add(offset))
        .and_then(|offset| clip_base.checked_add(offset))
        .ok_or(BinaryAnimationError::IdOverflow)?;
    Ok(
        TrackIds {
            translation: base,
            translation_curves: [
                base.checked_add(1)
                    .ok_or(BinaryAnimationError::IdOverflow)?,
                base.checked_add(2)
                    .ok_or(BinaryAnimationError::IdOverflow)?,
                base.checked_add(3)
                    .ok_or(BinaryAnimationError::IdOverflow)?,
            ],
            rotation: base
                .checked_add(4)
                .ok_or(BinaryAnimationError::IdOverflow)?,
            rotation_curves: [
                base.checked_add(5)
                    .ok_or(BinaryAnimationError::IdOverflow)?,
                base.checked_add(6)
                    .ok_or(BinaryAnimationError::IdOverflow)?,
                base.checked_add(7)
                    .ok_or(BinaryAnimationError::IdOverflow)?,
            ],
        },
    )
}

/// Require one frame rate shared by every clip in the FBX scene.
fn shared_frame_rate(
    clips: &[AnimationClip]
) -> Result<Option<f64>, BinaryAnimationError> {
    let Some(first) = clips.first() else {
        return Ok(None);
    };
    if clips
        .iter()
        .any(
            |clip| {
                !frame_rates_match(
                    clip.frame_rate,
                    first.frame_rate,
                )
            },
        )
    {
        return Err(BinaryAnimationError::MixedFrameRate);
    }
    Ok(Some(first.frame_rate))
}

/// Convert one integer source frame to FBX 7.x KTime.
pub(super) fn frame_time(
    frame: usize,
    frame_rate: f64,
) -> Result<i64, BinaryAnimationError> {
    if frame_rate > KTIME_PER_SECOND {
        return Err(BinaryAnimationError::TimeResolution);
    }
    let frame =
        u32::try_from(frame).map_err(|_| BinaryAnimationError::TimeOverflow)?;
    let value = (f64::from(frame) * KTIME_PER_SECOND / frame_rate).round();
    if !value.is_finite()
        || value < i64::MIN as f64
        || value >= I64_EXCLUSIVE_MAX
    {
        return Err(BinaryAnimationError::TimeOverflow);
    }
    Ok(value as i64)
}

/// Build key times for every authored integer sample in one clip.
pub(super) fn key_times(
    clip: &AnimationClip
) -> Result<Vec<i64>, BinaryAnimationError> {
    (0..clip.frame_count)
        .map(
            |frame| {
                frame_time(
                    frame,
                    clip.frame_rate,
                )
            },
        )
        .collect()
}

/// Copy authored source translations without adding a terminal sample.
pub(super) fn translation_values(
    clip: &AnimationClip,
    track_ordinal: usize,
) -> Result<Vec<[f64; 3]>, BinaryAnimationError> {
    let track = clip
        .tracks
        .get(track_ordinal)
        .ok_or(BinaryAnimationError::IdOverflow)?;
    Ok(
        track
            .samples
            .iter()
            .map(|sample| sample.translation)
            .collect(),
    )
}

/// Convert one track's absolute local quaternions into continuous XYZ angles.
pub(super) fn rotation_eulers(
    clip: &AnimationClip,
    track_ordinal: usize,
) -> Result<Vec<[f64; 3]>, BinaryAnimationError> {
    let track = clip
        .tracks
        .get(track_ordinal)
        .ok_or(BinaryAnimationError::IdOverflow)?;
    let mut rotations = Vec::with_capacity(
        track
            .samples
            .len(),
    );
    for sample in &track.samples {
        let matrix = to_row_matrix(
            sample.rotation_wxyz,
            sample.translation,
        );
        let mut rotation = decompose(&matrix)
            .map_err(BinaryAnimationError::Matrix)?
            .rotation_degrees;
        if let Some(previous) = rotations
            .last()
            .copied()
        {
            unwrap_euler(
                &mut rotation,
                previous,
            );
        }
        rotations.push(rotation);
    }
    Ok(rotations)
}

/// Keep consecutive Euler samples on the nearest equivalent angle branch.
fn unwrap_euler(
    current: &mut [f64; 3],
    previous: [f64; 3],
) {
    for axis in 0..3 {
        while current[axis] - previous[axis] > 180.0 {
            current[axis] -= 360.0;
        }
        while current[axis] - previous[axis] < -180.0 {
            current[axis] += 360.0;
        }
    }
}

/// Append one XYZ transform curve node, its curves, and property links.
// The explicit arguments form one ordered FBX transform-curve transaction.
#[expect(
    clippy::too_many_arguments,
    reason = "The arguments are one explicit FBX transform-curve transaction."
)]
fn append_transform_animation(
    objects: &mut Vec<BinaryNode>,
    connections: &mut Vec<BinaryNode>,
    counts: &mut BinaryAnimationCounts,
    node_id: u64,
    curve_ids: [u64; 3],
    node_name: &str,
    target_property: &str,
    bone_model: u64,
    times: &[i64],
    values: &[[f64; 3]],
) -> Result<(), BinaryAnimationError> {
    let defaults = values
        .first()
        .copied()
        .ok_or(BinaryAnimationError::TimeOverflow)?;
    objects.push(
        animation_curve_node(
            node_id, node_name, defaults,
        )?,
    );
    connections.push(
        property_connection(
            node_id,
            bone_model,
            target_property,
        )?,
    );
    for (axis, property) in [
        "d|X", "d|Y", "d|Z",
    ]
    .iter()
    .enumerate()
    {
        let axis_values = values
            .iter()
            .map(|value| checked_f32(value[axis]))
            .collect::<Result<Vec<_>, _>>()?;
        objects.push(
            animation_curve_node_values(
                curve_ids[axis],
                defaults[axis],
                times,
                axis_values,
            )?,
        );
        connections.push(
            property_connection(
                curve_ids[axis],
                node_id,
                property,
            )?,
        );
        counts.curves += 1;
    }
    counts.curve_nodes += 1;
    Ok(())
}

/// Build one FBX animation stack object.
fn animation_stack_node(
    id: u64,
    clip: &AnimationClip,
    stop_time: i64,
) -> Result<BinaryNode, BinaryAnimationError> {
    Ok(
        BinaryNode::new(
            "AnimationStack",
            vec![
                id_property(id)?,
                name_class(
                    &clip.name,
                    "AnimStack",
                ),
                string(""),
            ],
            vec![
                BinaryNode::branch(
                    "Properties70",
                    vec![
                        time_property(
                            "LocalStart",
                            0,
                        ),
                        time_property(
                            "LocalStop",
                            stop_time,
                        ),
                        time_property(
                            "ReferenceStart",
                            0,
                        ),
                        time_property(
                            "ReferenceStop",
                            stop_time,
                        ),
                    ],
                ),
            ],
        ),
    )
}

/// Build one FBX animation layer object.
fn animation_layer_node(id: u64) -> Result<BinaryNode, BinaryAnimationError> {
    Ok(
        BinaryNode::new(
            "AnimationLayer",
            vec![
                id_property(id)?,
                name_class(
                    "BaseLayer",
                    "AnimLayer",
                ),
                string(""),
            ],
            Vec::new(),
        ),
    )
}

/// Build one FBX XYZ animation curve-node object.
fn animation_curve_node(
    id: u64,
    name: &str,
    defaults: [f64; 3],
) -> Result<BinaryNode, BinaryAnimationError> {
    Ok(
        BinaryNode::new(
            "AnimationCurveNode",
            vec![
                id_property(id)?,
                name_class(
                    name,
                    "AnimCurveNode",
                ),
                string(""),
            ],
            vec![
                BinaryNode::branch(
                    "Properties70",
                    vec![
                        number_property(
                            "d|X",
                            defaults[0],
                        ),
                        number_property(
                            "d|Y",
                            defaults[1],
                        ),
                        number_property(
                            "d|Z",
                            defaults[2],
                        ),
                    ],
                ),
            ],
        ),
    )
}

/// Build one FBX scalar animation curve object.
fn animation_curve_node_values(
    id: u64,
    default_value: f64,
    times: &[i64],
    values: Vec<f32>,
) -> Result<BinaryNode, BinaryAnimationError> {
    if times.len() != values.len() {
        return Err(BinaryAnimationError::TimeOverflow);
    }
    let key_count = values.len();
    let key_flags = vec![LINEAR_KEY_FLAGS; key_count];
    let key_ref_counts = vec![1; key_count];
    let key_attribute_data = [
        0.0,
        0.0,
        9.419_963e-30,
        0.0,
    ]
    .repeat(key_count);
    Ok(
        BinaryNode::new(
            "AnimationCurve",
            vec![
                id_property(id)?,
                name_class(
                    "",
                    "AnimCurve",
                ),
                string(""),
            ],
            vec![
                BinaryNode::leaf(
                    "Default",
                    vec![BinaryProperty::F64(default_value)],
                ),
                BinaryNode::leaf(
                    "KeyVer",
                    vec![BinaryProperty::I32(ANIMATION_KEY_VERSION)],
                ),
                BinaryNode::leaf(
                    "KeyTime",
                    vec![BinaryProperty::I64Array(times.to_vec())],
                ),
                BinaryNode::leaf(
                    "KeyValueFloat",
                    vec![BinaryProperty::F32Array(values)],
                ),
                BinaryNode::leaf(
                    "KeyAttrFlags",
                    vec![BinaryProperty::I32Array(key_flags)],
                ),
                BinaryNode::leaf(
                    "KeyAttrDataFloat",
                    vec![BinaryProperty::F32Array(key_attribute_data)],
                ),
                BinaryNode::leaf(
                    "KeyAttrRefCount",
                    vec![BinaryProperty::I32Array(key_ref_counts)],
                ),
            ],
        ),
    )
}

/// Build one take entry for a clip.
fn take_node(
    clip: &AnimationClip,
    stop_time: i64,
) -> BinaryNode {
    BinaryNode::new(
        "Take",
        vec![string(&clip.name)],
        vec![
            string_node(
                "FileName",
                &format!(
                    "{}.tak",
                    clip.name
                ),
            ),
            BinaryNode::leaf(
                "LocalTime",
                vec![
                    BinaryProperty::I64(0),
                    BinaryProperty::I64(stop_time),
                ],
            ),
            BinaryNode::leaf(
                "ReferenceTime",
                vec![
                    BinaryProperty::I64(0),
                    BinaryProperty::I64(stop_time),
                ],
            ),
        ],
    )
}

/// Build one object-to-object connection.
fn object_connection(
    child: u64,
    parent: u64,
) -> Result<BinaryNode, BinaryAnimationError> {
    Ok(
        BinaryNode::leaf(
            "C",
            vec![
                string("OO"),
                id_property(child)?,
                id_property(parent)?,
            ],
        ),
    )
}

/// Build one object-to-property connection.
fn property_connection(
    child: u64,
    parent: u64,
    property: &str,
) -> Result<BinaryNode, BinaryAnimationError> {
    Ok(
        BinaryNode::leaf(
            "C",
            vec![
                string("OP"),
                id_property(child)?,
                id_property(parent)?,
                string(property),
            ],
        ),
    )
}

/// Build one animatable number property.
fn number_property(
    name: &str,
    value: f64,
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("Number"),
            string(""),
            string("A"),
            BinaryProperty::F64(value),
        ],
    )
}

/// Build one KTime property.
fn time_property(
    name: &str,
    value: i64,
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("KTime"),
            string("Time"),
            string(""),
            BinaryProperty::I64(value),
        ],
    )
}

/// Build one scalar string node.
fn string_node(
    name: &str,
    value: &str,
) -> BinaryNode {
    BinaryNode::leaf(
        name,
        vec![string(value)],
    )
}

/// Build one typed binary name-class property.
fn name_class(
    name: &str,
    class_name: &str,
) -> BinaryProperty {
    BinaryProperty::String(format!("{name}\0\u{1}{class_name}"))
}

/// Build one UTF-8 string property.
fn string(value: &str) -> BinaryProperty {
    BinaryProperty::String(value.to_owned())
}

/// Convert one deterministic unsigned id into the FBX signed integer field.
fn id_property(id: u64) -> Result<BinaryProperty, BinaryAnimationError> {
    i64::try_from(id)
        .map(BinaryProperty::I64)
        .map_err(|_| BinaryAnimationError::SignedIdOverflow(id))
}

/// Narrow one finite animation value to the FBX float curve representation.
fn checked_f32(value: f64) -> Result<f32, BinaryAnimationError> {
    let narrowed = value as f32;
    if !narrowed.is_finite() {
        return Err(BinaryAnimationError::ValueOverflow);
    }
    Ok(narrowed)
}

#[cfg(test)]
#[test]
fn shared_frame_rate_rejects_distinct_exact_values() {
    let clips = [
        AnimationClip {
            name: "first".to_owned(),
            frame_rate: 30.0_f64,
            cyclic: false,
            frame_count: 1,
            tracks: Vec::new(),
            ignored_group_ids: Vec::new(),
        },
        AnimationClip {
            name: "second".to_owned(),
            frame_rate: 30.000_000_000_5_f64,
            cyclic: false,
            frame_count: 1,
            tracks: Vec::new(),
            ignored_group_ids: Vec::new(),
        },
    ];

    assert_eq!(
        shared_frame_rate(&clips),
        Err(BinaryAnimationError::MixedFrameRate)
    );
}
