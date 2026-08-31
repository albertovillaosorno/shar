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
//   - Clip domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Clip domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Clip domain module.

use std::collections::BTreeSet;

/// One local bone transform at one integer source frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalTransformSample {
    /// Absolute local translation in source scene units.
    pub translation: [f64; 3],
    /// Absolute local unit quaternion in W, X, Y, Z order.
    pub rotation_wxyz: [f64; 4],
}

/// One bone track sampled at every integer frame in a clip.
#[derive(Clone, Debug, PartialEq)]
pub struct BoneAnimationTrack {
    /// Stable exported skeleton bone identity.
    pub bone_id: String,
    /// Samples aligned to clip frames zero through frame count minus one.
    pub samples: Vec<LocalTransformSample>,
}

/// Return whether two validated source frame rates are exactly identical.
#[must_use]
pub const fn frame_rates_match(left: f64, right: f64) -> bool {
    left.to_bits() == right.to_bits()
}

/// One deterministic skeletal animation clip.
// The explicit domain name distinguishes clips from source adapter records.
#[expect(
    clippy::module_name_repetitions,
    reason = "AnimationClip is the stable public domain name across adapters."
)]
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationClip {
    /// Stable publication clip identity.
    pub name: String,
    /// Optional authored source clip identity retained across canonical naming.
    pub source_identity: Option<String>,
    /// Source frames per second.
    pub frame_rate: f64,
    /// Whether the source declares cyclic playback.
    pub cyclic: bool,
    /// Integer source-frame sample count.
    pub frame_count: usize,
    /// Bone tracks in exported skeleton order.
    pub tracks: Vec<BoneAnimationTrack>,
    /// Source groups intentionally not bound to deforming skeleton bones.
    pub ignored_group_ids: Vec<String>,
}

/// Animation-domain validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnimationClipError {
    /// Clip identity was empty or carried surrounding whitespace.
    InvalidClipName,
    /// Authored source clip identity was empty or non-canonical.
    InvalidSourceClipName,
    /// Frame rate was non-finite or not positive.
    InvalidFrameRate,
    /// Clip declared no integer source frames.
    EmptyClip,
    /// No skeletal track matched the exported skeleton.
    MissingTracks,
    /// One bone identity was empty or non-canonical.
    InvalidBoneId,
    /// A bone identity appeared more than once.
    DuplicateBoneId(String),
    /// One track sample count differed from the clip frame count.
    SampleCountMismatch {
        /// Bone with the mismatched sample count.
        bone: String,
        /// Clip frame count.
        expected: usize,
        /// Actual sample count.
        actual: usize,
    },
    /// One sample contained a non-finite component.
    NonFiniteSample {
        /// Bone containing the invalid sample.
        bone: String,
        /// Frame containing the invalid sample.
        frame: usize,
    },
    /// One sample quaternion was not approximately unit length.
    NonUnitQuaternion {
        /// Bone containing the invalid quaternion.
        bone: String,
        /// Frame containing the invalid quaternion.
        frame: usize,
    },
    /// An ignored group identity was empty or non-canonical.
    InvalidIgnoredGroup,
    /// An ignored group identity appeared more than once.
    DuplicateIgnoredGroup(String),
}

impl AnimationClip {
    /// Create one validated sampled skeletal clip.
    ///
    /// # Errors
    ///
    /// Returns an error when clip timing, identities, sample counts, or local
    /// transforms violate the animation contract.
    pub fn new(
        name: impl Into<String>,
        frame_rate: f64,
        cyclic: bool,
        frame_count: usize,
        tracks: Vec<BoneAnimationTrack>,
        ignored_group_ids: Vec<String>,
    ) -> Result<Self, AnimationClipError> {
        let clip_name = name.into();
        if clip_name.is_empty()
            || clip_name != clip_name.trim()
            || clip_name.chars().any(char::is_control)
        {
            return Err(AnimationClipError::InvalidClipName);
        }
        if !frame_rate.is_finite() || frame_rate <= 0f64 {
            return Err(AnimationClipError::InvalidFrameRate);
        }
        if frame_count == 0 {
            return Err(AnimationClipError::EmptyClip);
        }
        if tracks.is_empty() {
            return Err(AnimationClipError::MissingTracks);
        }
        validate_tracks(&tracks, frame_count)?;
        validate_ignored_groups(&ignored_group_ids)?;
        Ok(Self {
            name: clip_name,
            source_identity: None,
            frame_rate,
            cyclic,
            frame_count,
            tracks,
            ignored_group_ids,
        })
    }

    /// Attach one canonical authored source clip identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the identity is blank, padded, or contains control
    /// characters.
    pub fn with_source_identity(
        mut self,
        source_identity: impl Into<String>,
    ) -> Result<Self, AnimationClipError> {
        let identity = source_identity.into();
        if identity.trim().is_empty()
            || identity != identity.trim()
            || identity.chars().any(char::is_control)
        {
            return Err(AnimationClipError::InvalidSourceClipName);
        }
        self.source_identity = Some(identity);
        Ok(self)
    }
}

/// Validate track identities, sample counts, and finite local transforms.
fn validate_tracks(
    tracks: &[BoneAnimationTrack],
    frame_count: usize,
) -> Result<(), AnimationClipError> {
    let mut bone_ids = BTreeSet::new();
    for track in tracks {
        if track.bone_id.is_empty()
            || track.bone_id != track.bone_id.trim()
            || track.bone_id.chars().any(char::is_control)
        {
            return Err(AnimationClipError::InvalidBoneId);
        }
        if !bone_ids.insert(track.bone_id.clone()) {
            return Err(AnimationClipError::DuplicateBoneId(
                track.bone_id.clone(),
            ));
        }
        if track.samples.len() != frame_count {
            return Err(AnimationClipError::SampleCountMismatch {
                bone: track.bone_id.clone(),
                expected: frame_count,
                actual: track.samples.len(),
            });
        }
        for (frame, sample) in track.samples.iter().enumerate() {
            if sample
                .translation
                .iter()
                .chain(sample.rotation_wxyz.iter())
                .any(|value| !value.is_finite())
            {
                return Err(AnimationClipError::NonFiniteSample {
                    bone: track.bone_id.clone(),
                    frame,
                });
            }
            let length_squared = sample
                .rotation_wxyz
                .iter()
                .map(|value| value * value)
                .sum::<f64>();
            if (length_squared - 1f64).abs() > 1e-4_f64 {
                return Err(AnimationClipError::NonUnitQuaternion {
                    bone: track.bone_id.clone(),
                    frame,
                });
            }
        }
    }
    Ok(())
}

/// Validate deterministic unique ignored-group evidence.
fn validate_ignored_groups(
    ignored_group_ids: &[String],
) -> Result<(), AnimationClipError> {
    let mut groups = BTreeSet::new();
    for group in ignored_group_ids {
        if group.is_empty()
            || group != group.trim()
            || group.chars().any(char::is_control)
        {
            return Err(AnimationClipError::InvalidIgnoredGroup);
        }
        if !groups.insert(group.clone()) {
            return Err(AnimationClipError::DuplicateIgnoredGroup(
                group.clone(),
            ));
        }
    }
    Ok(())
}
