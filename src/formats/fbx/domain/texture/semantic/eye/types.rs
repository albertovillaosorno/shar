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
//   - Types domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Types domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Types domain module.

#![expect(
    missing_copy_implementations,
    reason = "Semantic errors intentionally remain non-Copy to preserve \
              future diagnostic payload flexibility."
)]

use super::super::color::Rgba8;
use super::super::image::{RgbaImage, RgbaImageError};

/// Stable eye side derived from connected-component horizontal centroids.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EyeSide {
    /// Component with the lower horizontal centroid.
    NegativeX,
    /// Component with the higher horizontal centroid.
    PositiveX,
}

impl EyeSide {
    /// Return the stable manifest identity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NegativeX => "negative-x",
            Self::PositiveX => "positive-x",
        }
    }
}

/// Independently addressable semantic region within one eye.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EyeRegion {
    /// Upper half of the source-supported eyelid closure.
    UpperLid,
    /// Lower half of the source-supported eyelid closure.
    LowerLid,
    /// Open eye surface surrounding the pupil or iris.
    Surface,
    /// Pupil or iris evidence in the open frame.
    PupilIris,
}

impl EyeRegion {
    /// Return the stable manifest identity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpperLid => "upper-lid",
            Self::LowerLid => "lower-lid",
            Self::Surface => "surface",
            Self::PupilIris => "pupil-iris",
        }
    }
}

/// One connected eye geometry component.
#[derive(Clone, Debug, PartialEq)]
pub struct EyeComponent {
    /// Stable side identity.
    pub side: EyeSide,
    /// Sorted source vertex ordinals.
    pub vertex_indices: Vec<usize>,
    /// Horizontal source centroid used to assign side identity.
    pub centroid_x: f32,
}

/// One source blink frame's observable closure evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EyeFrameEvidence {
    /// Dense frame ordinal from zero through three.
    pub frame_index: usize,
    /// Exact source pixels matching the closed-lid color.
    pub lid_pixel_count: usize,
    /// Closed-lid pixels in the upper half.
    pub upper_lid_pixel_count: usize,
    /// Closed-lid pixels in the lower half.
    pub lower_lid_pixel_count: usize,
    /// Dark pupil or iris pixels unchanged from the open frame.
    pub preserved_pupil_pixel_count: usize,
}

/// Three eye images used by the prepared character package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EyeTextureLayers {
    /// Derived open-eye compatibility texture referenced by the FBX.
    pub composite: RgbaImage,
    /// Transparent texture containing only pupil pixels.
    pub pupil: RgbaImage,
    /// One lid-color square split into upper and lower semantic halves.
    pub lids: RgbaImage,
}

/// Complete two-eye, four-frame semantic plan.
#[derive(Clone, Debug, PartialEq)]
pub struct EyeSemanticPlan {
    /// Two connected geometry components in stable side order.
    pub components: Vec<EyeComponent>,
    /// Four source-frame evidence rows.
    pub frame_evidence: Vec<EyeFrameEvidence>,
    /// Four nearest-neighbor compatibility frames preserving source behavior.
    pub modern_frames: [RgbaImage; 4],
    /// Three canonical textures for sclera, pupil, and paired lids.
    pub layers: EyeTextureLayers,
    /// Exact fully closed lid color.
    pub lid_color: Rgba8,
    /// Dominant open eye surface color.
    pub surface_color: Rgba8,
    /// Darkest open pupil or iris color.
    pub pupil_color: Rgba8,
    /// Two eyes multiplied by four semantic regions.
    pub semantic_region_count: usize,
}

/// Eye semantic analysis failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EyeTextureError {
    /// Image validation or scaling failed.
    Image(RgbaImageError),
    /// Eye group did not resolve to exactly two connected components.
    ComponentCount {
        /// Actual connected component count.
        actual: usize,
    },
    /// One vertex was not referenced by any triangle.
    UncoveredVertex {
        /// Source vertex ordinal.
        vertex: usize,
    },
    /// Eye component centroids could not establish distinct sides.
    AmbiguousComponentSides,
    /// Output dimension was zero or too small for modernization.
    InvalidOutputSize,
    /// Source frames used different dimensions.
    FrameDimensionMismatch,
    /// Source frame dimensions could not support upper and lower lid evidence.
    InvalidFrameDimensions,
    /// Fully closed frame contained more than one color.
    NonUniformClosedFrame,
    /// Open frame contained no eye surface color distinct from the lid.
    MissingEyeSurface,
    /// Open frame contained no dark pupil or iris evidence.
    MissingPupilEvidence,
    /// Pupil evidence occupied an implausibly large part of the open frame.
    ExcessivePupilEvidence,
    /// Exact lid coverage did not increase monotonically across frames.
    NonMonotonicClosure {
        /// Earlier frame ordinal.
        earlier: usize,
        /// Later frame ordinal.
        later: usize,
    },
    /// One partial frame did not close symmetrically from top and bottom.
    AsymmetricClosure {
        /// Source frame ordinal.
        frame: usize,
    },
    /// One partial closure changed a pupil pixel to a non-lid color.
    PupilChangedBeforeClosure {
        /// Source frame ordinal.
        frame: usize,
    },
    /// Checked integer conversion or arithmetic failed.
    NumericOverflow,
}

impl From<RgbaImageError> for EyeTextureError {
    fn from(error: RgbaImageError) -> Self {
        Self::Image(error)
    }
}
