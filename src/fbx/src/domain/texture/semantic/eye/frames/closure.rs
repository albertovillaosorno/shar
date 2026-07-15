// File:
//   - closure.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/eye/frames/closure.rs
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
//   - Monotonic symmetric lid-mask validation and per-frame evidence rows.
// - Must-Not:
//   - Classify source colors, scale images, change frame order, or access
//   - files.
// - Allows:
//   - Exact mask-set comparison and checked row-count projection.
// - Split-When:
//   - Closure validation and evidence projection need independent consumers.
// - Merge-When:
//   - Source eye evidence can own this behavior without exceeding SRP.
// - Summary:
//   - Eye closure evidence validation.
// - Description:
//   - Proves the source lid masks grow symmetrically and records frame counts.
// - Usage:
//   - Called only by source eye-frame evidence analysis.
// - Defaults:
//   - Partial closure frames are one and two.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Monotonic eye closure validation and evidence projection.
use std::collections::BTreeSet;

use super::super::super::image::RgbaImage;
use super::super::types::{EyeFrameEvidence, EyeTextureError};

/// Prove exact lid masks grow monotonically and partial frames are symmetric.
pub(super) fn validate(
    masks: &[BTreeSet<usize>],
    width: u32,
    height: u32,
) -> Result<(), EyeTextureError> {
    for earlier in 0..3 {
        let later = earlier + 1;
        if !masks[earlier].is_subset(&masks[later])
            || masks[earlier].len() >= masks[later].len()
        {
            return Err(
                EyeTextureError::NonMonotonicClosure {
                    earlier,
                    later,
                },
            );
        }
    }
    let width = usize::try_from(width)
        .map_err(|_error| EyeTextureError::NumericOverflow)?;
    let half = usize::try_from(height / 2)
        .map_err(|_error| EyeTextureError::NumericOverflow)?;
    for frame in [
        1_usize, 2,
    ] {
        let upper = masks[frame]
            .iter()
            .filter(|index| **index / width < half)
            .count();
        let lower = masks[frame].len() - upper;
        if upper != lower {
            return Err(
                EyeTextureError::AsymmetricClosure {
                    frame,
                },
            );
        }
    }
    Ok(())
}

/// Build one frame evidence row.
pub(super) fn frame_evidence(
    frame_index: usize,
    lid_indices: &BTreeSet<usize>,
    pupil_indices: &BTreeSet<usize>,
    frames: &[RgbaImage; 4],
) -> Result<EyeFrameEvidence, EyeTextureError> {
    let width = usize::try_from(frames[frame_index].width())
        .map_err(|_error| EyeTextureError::NumericOverflow)?;
    let half = usize::try_from(frames[frame_index].height() / 2)
        .map_err(|_error| EyeTextureError::NumericOverflow)?;
    let upper = lid_indices
        .iter()
        .filter(|index| **index / width < half)
        .count();
    let preserved = pupil_indices
        .iter()
        .filter(
            |index| {
                frames[frame_index].pixels()[**index]
                    == frames[0].pixels()[**index]
            },
        )
        .count();
    Ok(
        EyeFrameEvidence {
            frame_index,
            lid_pixel_count: lid_indices.len(),
            upper_lid_pixel_count: upper,
            lower_lid_pixel_count: lid_indices.len() - upper,
            preserved_pupil_pixel_count: preserved,
        },
    )
}
