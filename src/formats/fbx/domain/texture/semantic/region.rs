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
//   - Region domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Region domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Region domain module.

#![expect(
    clippy::module_name_repetitions,
    reason = "Region names remain explicit across semantic texture manifests."
)]

/// Stable non-eye semantic body regions; individual characters may omit lanes.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BodyRegion {
    /// Exposed skin and skin-colored surfaces.
    Skin,
    /// Hair and other source-supported dark hair detail.
    Hair,
    /// Integrated upper-body clothing.
    Torso,
    /// Integrated lower-body clothing.
    Legs,
    /// Integrated footwear.
    Shoes,
}

impl BodyRegion {
    /// Canonical region order used by atlas columns and manifests.
    pub const ALL: [Self; 5] =
        [Self::Skin, Self::Hair, Self::Torso, Self::Legs, Self::Shoes];

    /// Return the stable manifest identity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Skin => "skin",
            Self::Hair => "hair",
            Self::Torso => "torso",
            Self::Legs => "legs",
            Self::Shoes => "shoes",
        }
    }

    /// Return the fixed atlas-column ordinal.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        match self {
            Self::Skin => 0,
            Self::Hair => 1,
            Self::Torso => 2,
            Self::Legs => 3,
            Self::Shoes => 4,
        }
    }
}

/// Conservative anatomical family inferred from one dominant skin bone.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BoneFamily {
    /// Head, neck, arm, hand, and finger evidence.
    Exposed,
    /// Spine, chest, clavicle, and shoulder evidence.
    Torso,
    /// Pelvis, hip, knee, thigh, calf, and leg evidence.
    LowerBody,
    /// Ankle, foot, ball, and toe evidence.
    Foot,
    /// Root, support, or otherwise unrecognized evidence.
    Unsupported,
}

impl BoneFamily {
    /// Classify one canonical bone identity by complete lowercase tokens.
    #[must_use]
    pub fn from_bone_id(bone_id: &str) -> Self {
        let tokens = bone_id
            .split(|character: char| !character.is_ascii_alphanumeric())
            .filter(|token| !token.is_empty())
            .map(str::to_ascii_lowercase)
            .collect::<Vec<_>>();
        if contains_any(&tokens, &["ankle", "ball", "foot", "toe"]) {
            return Self::Foot;
        }
        if contains_any(&tokens, &[
            "ass", "pelvis", "hip", "knee", "thigh", "calf", "leg",
        ]) {
            return Self::LowerBody;
        }
        if contains_any(&tokens, &["spine", "chest", "clavicle", "shoulder"]) {
            return Self::Torso;
        }
        if contains_any(&tokens, &[
            "head", "jaw", "neck", "arm", "elbow", "wrist", "hand", "middle",
            "thumb", "finger",
        ]) {
            return Self::Exposed;
        }
        Self::Unsupported
    }

    /// Return the stable evidence identity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exposed => "exposed",
            Self::Torso => "torso",
            Self::LowerBody => "lower-body",
            Self::Foot => "foot",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Return whether one token list contains any complete candidate token.
fn contains_any(tokens: &[String], candidates: &[&str]) -> bool {
    candidates
        .iter()
        .any(|candidate| tokens.iter().any(|token| token == candidate))
}
