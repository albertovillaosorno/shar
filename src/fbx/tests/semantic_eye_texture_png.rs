// File:
//   - semantic_eye_texture_png.rs
// Path:
//   - src/fbx/tests/semantic_eye_texture_png.rs
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
//   - Behavioral regression coverage for two-eye components, four-frame blink
//   - evidence, modernization, and PNG byte conversion.
// - Must-Not:
//   - Read extracted assets, invoke external authoring applications, or invent
//   - an alternate eye animation mechanism.
// - Allows:
//   - Synthetic indexed and RGBA PNG bytes and synthetic eye evidence.
// - Split-When:
//   - Eye semantics and PNG conversion no longer share one artifact boundary.
// - Merge-When:
//   - Another integration test owns the same observable contracts.
// - Summary:
//   - Semantic eye and PNG behavioral regression.
// - Description:
//   - Proves source frame order and pixels remain authoritative through
//   - scaling.
// - Usage:
//   - Runs through the standard fbx integration test suite.
// - Defaults:
//   - Every fixture is independently authored and redistributable.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: eye component, frame, negative-path, RGBA, and indexed-PNG
//   - behavior form one end-to-end semantic texture container family.
//

//! Behavioral regression for semantic eyes and deterministic PNG conversion.
#[path = "common/semantic_eye.rs"]
mod semantic_eye;

use fbx::adapters::driven::semantic_texture_png::{
    decode_png_bytes, encode_png_bytes,
};
use fbx::domain::texture::semantic::{
    EyeSemanticPlan, EyeSide, EyeTextureError, Rgba8, RgbaImage,
    analyze_eye_frames,
};
use schoenwald_filesystem as _;
use semantic_eye::{eye_frames, eye_group};
use serde as _;
use serde_json as _;
use shar_sha256 as _;

#[test]
fn preserves_two_eye_components_and_four_frame_closure() -> Result<(), String> {
    let group = eye_group()?;
    let frames = eye_frames()?;
    let first = analyze_eye_frames(
        &group, &frames, 16,
    )
    .map_err(|error| format!("first eye plan failed: {error:?}"))?;
    let second = analyze_eye_frames(
        &group, &frames, 16,
    )
    .map_err(|error| format!("second eye plan failed: {error:?}"))?;
    if first != second {
        return Err("equivalent eye analysis was not deterministic".to_owned());
    }
    validate_eye_components(&first)?;
    validate_blink_evidence(&first)?;
    validate_eye_layers(&first)
}

/// Validate stable two-component eye ownership.
fn validate_eye_components(plan: &EyeSemanticPlan) -> Result<(), String> {
    let sides = plan
        .components
        .iter()
        .map(|component| component.side)
        .collect::<Vec<_>>();
    if sides
        != [
            EyeSide::NegativeX,
            EyeSide::PositiveX,
        ]
    {
        return Err(format!("unexpected eye sides: {sides:?}"));
    }
    if plan
        .components
        .iter()
        .any(
            |component| {
                component
                    .vertex_indices
                    .len()
                    != 3
            },
        )
    {
        return Err(
            format!(
                "unexpected eye components: {:?}",
                plan.components
            ),
        );
    }
    if plan.semantic_region_count != 8 {
        return Err(
            format!(
                "expected eight semantic eye regions, got {}",
                plan.semantic_region_count,
            ),
        );
    }
    Ok(())
}

/// Validate monotonic four-frame lid and pupil evidence.
fn validate_blink_evidence(plan: &EyeSemanticPlan) -> Result<(), String> {
    let lid_counts = plan
        .frame_evidence
        .iter()
        .map(|evidence| evidence.lid_pixel_count)
        .collect::<Vec<_>>();
    if lid_counts
        != [
            0, 32, 48, 64,
        ]
    {
        return Err(format!("unexpected lid counts: {lid_counts:?}"));
    }
    let upper = plan
        .frame_evidence
        .iter()
        .map(|evidence| evidence.upper_lid_pixel_count)
        .collect::<Vec<_>>();
    let lower = plan
        .frame_evidence
        .iter()
        .map(|evidence| evidence.lower_lid_pixel_count)
        .collect::<Vec<_>>();
    if upper
        != [
            0, 16, 24, 32,
        ]
        || lower != upper
    {
        return Err(
            format!(
                "asymmetric lid evidence: upper={upper:?}, lower={lower:?}",
            ),
        );
    }
    let preserved = plan
        .frame_evidence
        .iter()
        .map(|evidence| evidence.preserved_pupil_pixel_count)
        .collect::<Vec<_>>();
    if preserved
        != [
            4, 4, 4, 0,
        ]
    {
        return Err(format!("unexpected pupil preservation: {preserved:?}"));
    }
    Ok(())
}

/// Validate modern dimensions and independent white/pupil layers.
fn validate_eye_layers(plan: &EyeSemanticPlan) -> Result<(), String> {
    if plan
        .modern_frames
        .iter()
        .any(|frame| frame.width() != 16 || frame.height() != 16)
    {
        return Err(
            "modernized eye frames have incorrect dimensions".to_owned(),
        );
    }
    let white = Rgba8::new(
        255, 255, 255, 255,
    );
    if plan
        .layers
        .composite
        .pixels()
        .iter()
        .any(|color| *color != white)
    {
        return Err("eye compatibility texture was not pure white".to_owned());
    }
    let alpha = plan
        .layers
        .pupil
        .pixels()
        .iter()
        .map(|color| color.alpha)
        .collect::<std::collections::BTreeSet<_>>();
    if alpha
        != [
            0, 255,
        ]
        .into_iter()
        .collect()
    {
        return Err(format!("unexpected pupil-layer alpha: {alpha:?}"));
    }
    Ok(())
}

#[test]
#[expect(
    clippy::indexing_slicing,
    reason = "Fixed fixture constructors validate literals before indexing."
)]
fn accepts_symmetric_lid_occlusion_of_pupil_pixels() -> Result<(), String> {
    let group = eye_group()?;
    let mut frames = eye_frames()?;
    let lid = Rgba8::new(
        255, 210, 0, 255,
    );
    for frame_index in [
        1_usize, 2,
    ] {
        for y in [
            3_u32, 4,
        ] {
            frames[frame_index]
                .set_pixel(
                    2, y, lid,
                )
                .map_err(
                    |error| format!("lid occlusion fixture failed: {error:?}"),
                )?;
        }
    }
    let planned = analyze_eye_frames(
        &group, &frames, 16,
    )
    .map_err(|error| format!("lid occlusion should be accepted: {error:?}"))?;
    let preserved = planned
        .frame_evidence
        .iter()
        .map(|evidence| evidence.preserved_pupil_pixel_count)
        .collect::<Vec<_>>();
    if preserved
        != [
            4, 2, 2, 0,
        ]
    {
        return Err(
            format!("unexpected occluded pupil evidence: {preserved:?}"),
        );
    }
    Ok(())
}

#[test]
fn rejects_pupil_changes_before_the_fully_closed_frame() -> Result<(), String> {
    let group = eye_group()?;
    let mut frames = eye_frames()?;
    frames[2]
        .set_pixel(
            2,
            3,
            Rgba8::new(
                32, 32, 32, 255,
            ),
        )
        .map_err(|error| format!("fixture mutation failed: {error:?}"))?;
    match analyze_eye_frames(
        &group, &frames, 16,
    ) {
        Err(EyeTextureError::PupilChangedBeforeClosure {
            frame: 2,
        }) => Ok(()),
        other => Err(format!("expected pupil-change rejection, got {other:?}")),
    }
}

#[test]
fn png_round_trip_and_indexed_expansion_are_stable() -> Result<(), String> {
    let image = RgbaImage::new(
        2,
        1,
        vec![
            Rgba8::new(
                10, 20, 30, 255,
            ),
            Rgba8::new(
                40, 50, 60, 128,
            ),
        ],
    )
    .map_err(|error| format!("image failed: {error:?}"))?;
    let first = encode_png_bytes(&image)
        .map_err(|error| format!("first PNG encode failed: {error:?}"))?;
    let second = encode_png_bytes(&image)
        .map_err(|error| format!("second PNG encode failed: {error:?}"))?;
    if first != second {
        return Err("equivalent PNG encoding changed bytes".to_owned());
    }
    let decoded = decode_png_bytes(&first)
        .map_err(|error| format!("RGBA PNG decode failed: {error:?}"))?;
    if decoded != image {
        return Err(format!("RGBA PNG round trip changed pixels: {decoded:?}"));
    }
    let indexed = indexed_png()?;
    let expanded = decode_png_bytes(&indexed)
        .map_err(|error| format!("indexed PNG decode failed: {error:?}"))?;
    let expected = RgbaImage::new(
        2,
        1,
        vec![
            Rgba8::new(
                255, 0, 0, 255,
            ),
            Rgba8::new(
                0, 255, 0, 128,
            ),
        ],
    )
    .map_err(|error| format!("expected image failed: {error:?}"))?;
    if expanded != expected {
        return Err(
            format!("indexed PNG expansion changed pixels: {expanded:?}"),
        );
    }
    Ok(())
}

/// Build one two-color indexed PNG with palette alpha.
fn indexed_png() -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(
            &mut bytes, 2, 1,
        );
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_palette(
            vec![
                255, 0, 0, 0, 255, 0,
            ],
        );
        encoder.set_trns(
            vec![
                255, 128,
            ],
        );
        let mut writer = encoder
            .write_header()
            .map_err(|error| format!("indexed header failed: {error}"))?;
        writer
            .write_image_data(
                &[
                    0, 1,
                ],
            )
            .map_err(|error| format!("indexed data failed: {error}"))?;
    }
    Ok(bytes)
}
