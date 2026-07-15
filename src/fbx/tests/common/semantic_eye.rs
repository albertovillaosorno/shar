// File:
//   - semantic_eye.rs
// Path:
//   - src/fbx/tests/common/semantic_eye.rs
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
//   - Redistributable two-component eye geometry and four-frame blink fixtures.
// - Must-Not:
//   - Read extracted assets, contain proprietary pixels, or duplicate eye
//   - analysis logic.
// - Allows:
//   - Explicit public-domain mesh and image construction.
// - Split-When:
//   - Geometry and frame fixtures need independent behavior contracts.
// - Merge-When:
//   - Another test-support module owns the same eye fixtures.
// - Summary:
//   - Synthetic semantic eye fixtures.
// - Description:
//   - Builds two disconnected eye components and symmetric monotonic closure.
// - Usage:
//   - Imported by semantic eye and PNG integration tests.
// - Defaults:
//   - Pupil pixels remain unchanged through both partial-closure frames.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Redistributable synthetic semantic eye fixtures.
use fbx::domain::mesh::PrimitiveGroup;
use fbx::domain::texture::semantic::{Rgba8, RgbaImage};

/// Build one primitive group with two disconnected eye components.
pub fn eye_group() -> Result<PrimitiveGroup, String> {
    PrimitiveGroup::new(
        0,
        "synthetic-eyes",
        vec![
            [
                -2.0, 0.0, 0.0,
            ],
            [
                -1.0, 0.0, 0.0,
            ],
            [
                -2.0, 1.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
            [
                2.0, 0.0, 0.0,
            ],
            [
                1.0, 1.0, 0.0,
            ],
        ],
        vec![
            [
                0.5, 0.5
            ];
            6
        ],
        &[
            0, 1, 2, 3, 4, 5,
        ],
    )
    .map_err(|error| format!("eye group failed: {error:?}"))
}

/// Build four symmetric monotonic blink frames with persistent pupil pixels.
pub fn eye_frames() -> Result<[RgbaImage; 4], String> {
    let white = Rgba8::new(
        255, 255, 255, 255,
    );
    let black = Rgba8::new(
        0, 0, 0, 255,
    );
    let lid = Rgba8::new(
        255, 210, 0, 255,
    );
    let mut open = RgbaImage::filled(
        8, 8, white,
    )
    .map_err(|error| format!("open frame failed: {error:?}"))?;
    for (x, y) in [
        (
            2, 3,
        ),
        (
            5, 3,
        ),
        (
            2, 4,
        ),
        (
            5, 4,
        ),
    ] {
        open.set_pixel(
            x, y, black,
        )
        .map_err(|error| format!("pupil failed: {error:?}"))?;
    }
    let mut half = open.clone();
    paint_lid_rows(
        &mut half,
        lid,
        &[
            0, 1, 6, 7,
        ],
    )?;
    let mut near_closed = open.clone();
    paint_lid_rows(
        &mut near_closed,
        lid,
        &[
            0, 1, 2, 5, 6, 7,
        ],
    )?;
    let closed = RgbaImage::filled(
        8, 8, lid,
    )
    .map_err(|error| format!("closed frame failed: {error:?}"))?;
    Ok(
        [
            open,
            half,
            near_closed,
            closed,
        ],
    )
}

/// Paint exact full-width lid rows in one synthetic frame.
fn paint_lid_rows(
    frame: &mut RgbaImage,
    color: Rgba8,
    rows: &[u32],
) -> Result<(), String> {
    for y in rows {
        for x in 0..frame.width() {
            frame
                .set_pixel(
                    x, *y, color,
                )
                .map_err(|error| format!("lid pixel failed: {error:?}"))?;
        }
    }
    Ok(())
}
