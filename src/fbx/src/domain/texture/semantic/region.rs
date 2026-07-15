// File:
//   - region.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/region.rs
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
//   - Stable body-region identities and bone-name evidence families.
// - Must-Not:
//   - Read files, sample textures, infer color ownership, or modify geometry.
// - Allows:
//   - Deterministic region ordering and conservative bone-token matching.
// - Split-When:
//   - Outfit or accessory regions require an independent semantic taxonomy.
// - Merge-When:
//   - Body planning becomes the sole owner of these identities.
// - Summary:
//   - Semantic body and bone-family taxonomy.
// - Description:
//   - Provides stable names and ordering for character texture evidence.
// - Usage:
//   - Used by body color classification, atlas grouping, and manifests.
// - Defaults:
//   - Unrecognized bone identities remain unsupported and fail closed.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Stable body-region and bone-evidence identities.
/// Minimum non-eye semantic body regions.
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
    pub const ALL: [Self; 5] = [
        Self::Skin,
        Self::Hair,
        Self::Torso,
        Self::Legs,
        Self::Shoes,
    ];

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
        if contains_any(
            &tokens,
            &[
                "ankle", "ball", "foot", "toe",
            ],
        ) {
            return Self::Foot;
        }
        if contains_any(
            &tokens,
            &[
                "pelvis", "hip", "knee", "thigh", "calf", "leg",
            ],
        ) {
            return Self::LowerBody;
        }
        if contains_any(
            &tokens,
            &[
                "spine", "chest", "clavicle", "shoulder",
            ],
        ) {
            return Self::Torso;
        }
        if contains_any(
            &tokens,
            &[
                "head", "jaw", "neck", "arm", "elbow", "wrist", "hand",
                "middle", "thumb", "finger",
            ],
        ) {
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
fn contains_any(
    tokens: &[String],
    candidates: &[&str],
) -> bool {
    candidates
        .iter()
        .any(
            |candidate| {
                tokens
                    .iter()
                    .any(|token| token == candidate)
            },
        )
}
