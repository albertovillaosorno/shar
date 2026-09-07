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
//   - World-material projection domain tests.
// - Must-Not:
//   - Read private game content or contact Unreal Editor.
// - Allows:
//   - Synthetic binding, slot, texture, and presentation evidence.
// - Split-When:
//   - Another world-material schema gains independent test ownership.
// - Merge-When:
//   - Another test module owns identical projection validation.
// - Summary:
//   - World-material projection domain tests.
// - Description:
//   - Proves exact joins, slot fan-out, presentation deduplication, and drift
//   - rejection using repository-owned synthetic values.
// - Usage:
//   - Included by the asset-conversion world-material domain module.
// - Defaults:
//   - Any ambiguous or inconsistent evidence fails closed.
//

//! World-material projection domain tests.

use super::{
    WorldMaterialBindingSource, WorldMaterialProjection,
    WorldMaterialRasterProjection, WorldMaterialSemantics,
    WorldMaterialSlotSource,
};
use crate::domain::{
    WorldMaterialBlendFamily, WorldMaterialInstanceRaster,
    WorldMaterialMasterFamily, WorldMaterialShaderFamily,
};

const BINDING_A: &str =
    "1111111111111111111111111111111111111111111111111111111111111111";
const BINDING_B: &str =
    "2222222222222222222222222222222222222222222222222222222222222222";
const PRESENTATION_A: &str =
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const PRESENTATION_B: &str =
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const SLOT_OPAQUE: &str =
    "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const SLOT_GLASS: &str =
    "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const TEXTURE: &str =
    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";

fn binding(hash: &str, presentation: &str) -> WorldMaterialBindingSource {
    WorldMaterialBindingSource {
        material_name: format!("material-{hash}"),
        texture_file_name: Some(format!("texture-{TEXTURE}.png")),
        texture_sha256: Some(TEXTURE.to_owned()),
        binding_sha256: hash.to_owned(),
        presentation_sha256: presentation.to_owned(),
        base_color_rgba8: [12, 34, 56, 255],
        raster: WorldMaterialRasterProjection {
            master: WorldMaterialMasterFamily {
                shader: WorldMaterialShaderFamily::Simple,
                blend: WorldMaterialBlendFamily::Disabled,
                alpha_compare: None,
                two_sided: false,
                lit: false,
            },
            instance: WorldMaterialInstanceRaster::default(),
        },
        semantics: WorldMaterialSemantics::default(),
    }
}

fn slot(
    binding: &WorldMaterialBindingSource,
    effective: &str,
    semantics: WorldMaterialSemantics,
) -> WorldMaterialSlotSource {
    let suffix = if semantics.glass {
        "__glass"
    } else {
        ""
    };
    WorldMaterialSlotSource {
        slot_name: format!("{}{suffix}", binding.material_name),
        source_material_name: binding.material_name.clone(),
        binding_sha256: binding.binding_sha256.clone(),
        presentation_sha256: binding.presentation_sha256.clone(),
        slot_presentation_sha256: effective.to_owned(),
        semantics,
    }
}

#[test]
fn projection_preserves_binding_identity_across_slot_fanout()
-> Result<(), String> {
    let binding = binding(BINDING_A, PRESENTATION_A);
    let opaque = slot(&binding, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let glass = slot(&binding, SLOT_GLASS, WorldMaterialSemantics {
        transparent: true,
        glass: true,
        ..WorldMaterialSemantics::default()
    });
    let projection =
        WorldMaterialProjection::build(&[binding], &[glass, opaque])?;
    assert_eq!(projection.presentations().len(), 2);
    assert_eq!(projection.assignments().len(), 2);
    assert!(
        projection
            .assignments()
            .iter()
            .all(|assignment| assignment.binding_sha256 == BINDING_A)
    );
    let mut assignments = projection.assignments().iter();
    let first = assignments.next().ok_or("first assignment is missing")?;
    let second = assignments.next().ok_or("second assignment is missing")?;
    assert!(assignments.next().is_none());
    assert!(first.slot_name < second.slot_name);
    Ok(())
}

#[test]
fn projection_deduplicates_only_equal_effective_presentations()
-> Result<(), String> {
    let first = binding(BINDING_A, PRESENTATION_A);
    let mut second = binding(BINDING_B, PRESENTATION_B);
    second.base_color_rgba8 = first.base_color_rgba8;
    let first_slot =
        slot(&first, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let second_slot =
        slot(&second, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let projection = WorldMaterialProjection::build(&[first, second], &[
        first_slot,
        second_slot,
    ])?;
    assert_eq!(projection.presentations().len(), 1);
    assert_eq!(projection.assignments().len(), 2);
    let mut assignments = projection.assignments().iter();
    let first = assignments.next().ok_or("first assignment is missing")?;
    let second = assignments.next().ok_or("second assignment is missing")?;
    assert!(assignments.next().is_none());
    assert_ne!(first.binding_sha256, second.binding_sha256);
    Ok(())
}

#[test]
fn projection_rejects_conflicting_state_for_equal_effective_hash() {
    let first = binding(BINDING_A, PRESENTATION_A);
    let mut second = binding(BINDING_B, PRESENTATION_B);
    second.base_color_rgba8 = [99, 88, 77, 255];
    let first_slot =
        slot(&first, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let second_slot =
        slot(&second, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let result = WorldMaterialProjection::build(&[first, second], &[
        first_slot,
        second_slot,
    ]);
    assert_eq!(
        result,
        Err("world effective presentation hash has conflicting state"
            .to_owned())
    );
}

#[test]
fn projection_rejects_base_semantic_suffix_drift() {
    let mut binding = binding(BINDING_A, PRESENTATION_A);
    binding.semantics.transparent = true;
    let source = WorldMaterialSlotSource {
        slot_name: format!("{}__transparent", binding.material_name),
        source_material_name: binding.material_name.clone(),
        binding_sha256: binding.binding_sha256.clone(),
        presentation_sha256: binding.presentation_sha256.clone(),
        slot_presentation_sha256: SLOT_OPAQUE.to_owned(),
        semantics: binding.semantics,
    };
    let result = WorldMaterialProjection::build(&[binding], &[source]);
    assert_eq!(
        result,
        Err("world material name does not bind its digest".to_owned())
    );
}

#[test]
fn projection_rejects_slot_join_hash_drift() {
    let binding = binding(BINDING_A, PRESENTATION_A);
    let mut source =
        slot(&binding, SLOT_OPAQUE, WorldMaterialSemantics::default());
    source.binding_sha256 = BINDING_B.to_owned();
    let result = WorldMaterialProjection::build(&[binding], &[source]);
    assert_eq!(
        result,
        Err("world material slot binding join drifted".to_owned())
    );
}

#[test]
fn projection_rejects_writer_slot_name_drift() {
    let binding = binding(BINDING_A, PRESENTATION_A);
    let mut source = slot(&binding, SLOT_GLASS, WorldMaterialSemantics {
        transparent: true,
        glass: true,
        ..WorldMaterialSemantics::default()
    });
    source.slot_name = binding.material_name.clone();
    let result = WorldMaterialProjection::build(&[binding], &[source]);
    assert_eq!(
        result,
        Err("world material slot identity drifted from writer".to_owned())
    );
}

#[test]
fn projection_rejects_incomplete_texture_identity() {
    let mut binding = binding(BINDING_A, PRESENTATION_A);
    binding.texture_sha256 = None;
    let source = slot(&binding, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let result = WorldMaterialProjection::build(&[binding], &[source]);
    assert_eq!(
        result,
        Err("world material texture evidence is incomplete".to_owned())
    );
}

#[test]
fn projection_rejects_binding_without_writer_slot() {
    let first = binding(BINDING_A, PRESENTATION_A);
    let second = binding(BINDING_B, PRESENTATION_B);
    let source = slot(&first, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let result = WorldMaterialProjection::build(&[first, second], &[source]);
    assert_eq!(
        result,
        Err("world material binding lacks an exact writer slot".to_owned())
    );
}

#[test]
fn projection_carries_raster_family_into_effective_presentation()
-> Result<(), String> {
    let mut binding = binding(BINDING_A, PRESENTATION_A);
    binding.raster = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Environment,
        1,
        0,
        4,
        None,
        1,
        1,
    )?;
    let source = slot(&binding, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let projection = WorldMaterialProjection::build(&[binding], &[source])?;
    let presentation = projection
        .presentations()
        .first()
        .ok_or("effective presentation is missing")?;
    assert_eq!(
        presentation.raster.master.identity(),
        "environment__blend-alpha__alpha-test-off__two-sided__lit"
    );
    Ok(())
}

#[test]
fn projection_rejects_equal_effective_hash_with_different_raster_state()
-> Result<(), String> {
    let first = binding(BINDING_A, PRESENTATION_A);
    let mut second = binding(BINDING_B, PRESENTATION_B);
    second.raster = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        1,
        0,
        4,
        None,
        0,
        0,
    )?;
    let first_slot =
        slot(&first, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let second_slot =
        slot(&second, SLOT_OPAQUE, WorldMaterialSemantics::default());
    let result = WorldMaterialProjection::build(&[first, second], &[
        first_slot,
        second_slot,
    ]);
    assert_eq!(
        result,
        Err("world effective presentation hash has conflicting state"
            .to_owned())
    );
    Ok(())
}
