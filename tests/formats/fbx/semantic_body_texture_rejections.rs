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
//   - Semantic body texture rejections test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Semantic body texture rejections test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Semantic body texture rejections test module.

#[path = "common/semantic_body.rs"]
mod semantic_body;

use std::path::PathBuf;

use fbx::adapters::driven::semantic_character_texture::SemanticTextureRequest;
use fbx::adapters::driven::semantic_character_texture::request::{
    ExtraMaterialRequest, GroupAddressRequest, RequestError,
};
use fbx::domain::skin::SkinInfluence;
use fbx::domain::texture::semantic::{
    BodyRegion, SemanticTextureError, plan_body_texture,
};
use png as _;
use schoenwald_filesystem as _;
use semantic_body::{BODY_COLORS, body_fixture};
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn texture_request(
    extra_materials: Vec<ExtraMaterialRequest>,
) -> SemanticTextureRequest {
    SemanticTextureRequest {
        character_name: "fixture".to_owned(),
        skeleton_path: PathBuf::new(),
        skin_paths: Vec::new(),
        mesh_paths: Vec::new(),
        composite_paths: Vec::new(),
        general_animation_paths: Vec::new(),
        character_animation_paths: Vec::new(),
        body_texture_path: PathBuf::new(),
        body_texture_mode: "preserve-source".to_owned(),
        body_texture_address_mode: "clamp".to_owned(),
        eye_frame_paths: None,
        body_groups: Vec::<GroupAddressRequest>::new(),
        eye_group: None,
        color_overrides: Vec::new(),
        hair_luminance_ratio: 0.2,
        body_atlas_width: 1,
        body_atlas_height: 1,
        body_atlas_padding: 0,
        body_atlas_background: [0, 0, 0, 255],
        eye_output_size: 1,
        extra_materials,
        untextured_materials: Vec::new(),
    }
}

fn extra(material: &str, output: &str) -> ExtraMaterialRequest {
    ExtraMaterialRequest {
        material_name: material.to_owned(),
        texture_path: PathBuf::from("unused.png"),
        output_file_name: output.to_owned(),
    }
}

#[test]
fn rejects_reserved_extra_texture_identity() {
    let request = texture_request(vec![extra("extra", "BODY-ATLAS.PNG")]);
    assert_eq!(
        request.validate_extra_texture_outputs(),
        Err(RequestError::ReservedExtraTextureOutput(
            "BODY-ATLAS.PNG".to_owned()
        ))
    );
}

#[test]
fn rejects_case_folded_extra_texture_alias() {
    let request = texture_request(vec![
        extra("first", "shared.png"),
        extra("second", "SHARED.PNG"),
    ]);
    assert_eq!(
        request.validate_extra_texture_outputs(),
        Err(RequestError::CaseAliasedExtraTextureOutput {
            first: "shared.png".to_owned(),
            second: "SHARED.PNG".to_owned(),
        })
    );
}

#[test]
fn accepts_exact_shared_extra_texture_identity() -> Result<(), String> {
    let request = texture_request(vec![
        extra("first", "shared.png"),
        extra("second", "shared.png"),
    ]);
    request
        .validate_extra_texture_outputs()
        .map_err(|error| format!("exact shared output failed: {error:?}"))
}

#[test]
#[expect(
    clippy::indexing_slicing,
    reason = "Fixed fixture constructors validate literals before indexing."
)]
fn rejects_a_triangle_that_samples_more_than_one_source_color()
-> Result<(), String> {
    let (mut character, source, recipe) = body_fixture()?;
    character.parts[0].mesh.groups[0].uvs[1] = [0.3, 0.5];
    let result = plan_body_texture(&character, &source, &recipe);
    match result {
        Err(SemanticTextureError::MixedSourceColorTriangle {
            group,
            triangle,
        }) if group.part_index == 0
            && group.group_index == 0
            && triangle == 0 =>
        {
            Ok(())
        },
        other => Err(format!(
            "expected mixed source-color rejection, got {other:?}"
        )),
    }
}

#[test]
#[expect(
    clippy::indexing_slicing,
    reason = "Fixed fixture constructors validate literals before indexing."
)]
fn seam_tie_defers_to_remaining_color_evidence() -> Result<(), String> {
    let (mut character, source, recipe) = body_fixture()?;
    let influences = &mut character.parts[0].group_influences[0];
    let head = influences
        .iter_mut()
        .find(|influence| influence.vertex_index == 0)
        .ok_or_else(|| "missing synthetic head influence".to_owned())?;
    head.weight = 0.5;
    influences.push(SkinInfluence {
        vertex_index: 0,
        bone_id: "spine".to_owned(),
        weight: 0.5,
    });
    let planned = plan_body_texture(&character, &source, &recipe)
        .map_err(|error| format!("seam tie should defer: {error:?}"))?;
    let assignment = planned
        .color_assignments
        .iter()
        .find(|assignment| assignment.color == BODY_COLORS[0])
        .ok_or_else(|| "missing automatic skin assignment".to_owned())?;
    let vote_count: u32 = assignment.family_counts.values().sum();
    if assignment.overridden || vote_count != 2 {
        return Err(format!(
            "seam tie was not deferred correctly: {assignment:?}"
        ));
    }
    Ok(())
}

#[test]
#[expect(
    clippy::indexing_slicing,
    reason = "Fixed fixture constructors validate literals before indexing."
)]
fn requires_reviewed_color_when_every_vote_is_tied() -> Result<(), String> {
    let (mut character, source, mut recipe) = body_fixture()?;
    let influences = &mut character.parts[0].group_influences[0];
    for vertex_index in 0_u32..3 {
        let head = influences
            .iter_mut()
            .find(|influence| influence.vertex_index == vertex_index)
            .ok_or_else(|| "missing synthetic head influence".to_owned())?;
        head.weight = 0.5;
        influences.push(SkinInfluence {
            vertex_index,
            bone_id: "spine".to_owned(),
            weight: 0.5,
        });
    }
    match plan_body_texture(&character, &source, &recipe) {
        Err(SemanticTextureError::AmbiguousColorEvidence(color))
            if color == BODY_COLORS[0] => {},
        other => {
            return Err(format!(
                "expected color-level tie rejection, got {other:?}"
            ));
        },
    }
    let _previous = recipe
        .color_overrides
        .insert(BODY_COLORS[0], BodyRegion::Skin);
    let planned = plan_body_texture(&character, &source, &recipe)
        .map_err(|error| format!("reviewed override failed: {error:?}"))?;
    let assignment = planned
        .color_assignments
        .iter()
        .find(|assignment| assignment.color == BODY_COLORS[0])
        .ok_or_else(|| "missing reviewed skin assignment".to_owned())?;
    if !assignment.overridden || !assignment.family_counts.is_empty() {
        return Err(format!(
            "reviewed tie evidence was not preserved: {assignment:?}"
        ));
    }
    Ok(())
}
