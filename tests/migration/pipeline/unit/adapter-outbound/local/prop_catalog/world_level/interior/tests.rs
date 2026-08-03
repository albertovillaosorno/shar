// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

use super::{
    InteriorGeometryOwnership, geometry_key, identity_for_package,
    is_halloween_package, movement_for_package, retain_unowned_triangles,
    reviewed_movement_for_package,
};
use crate::adapters::driven::local::prop_catalog::world_level::movement::{
    LEGACY_REVIEWED_HEIGHT_OFFSET_METERS, WORLD_HEIGHT_OFFSET_METERS,
};

#[test]
fn all_nineteen_source_packages_have_reviewed_movements() {
    let packages = [
        "extracted-art-l1i00",
        "extracted-art-l1i01",
        "extracted-art-l1i02",
        "extracted-art-l2i03",
        "extracted-art-l2i04",
        "extracted-art-l3i05",
        "extracted-art-l3i06",
        "extracted-art-l4i00",
        "extracted-art-l4i01",
        "extracted-art-l4i02",
        "extracted-art-l4i07",
        "extracted-art-l5i03",
        "extracted-art-l5i04",
        "extracted-art-l6i05",
        "extracted-art-l6i06",
        "extracted-art-l7i00",
        "extracted-art-l7i01",
        "extracted-art-l7i02",
        "extracted-art-l7i07",
    ];
    assert_eq!(packages.len(), 19);
    for package in packages {
        assert!(identity_for_package(package).is_some(), "{package}");
        assert!(movement_for_package(package).is_some(), "{package}");
    }
}

#[test]
fn only_level_seven_halloween_packages_are_overlays() {
    assert!(is_halloween_package("extracted-art-l7i00"));
    assert!(is_halloween_package("extracted-art-l7i01"));
    assert!(is_halloween_package("extracted-art-l7i02"));
    assert!(is_halloween_package("extracted-art-l7i07"));
    assert!(!is_halloween_package("extracted-art-l4i07"));
    assert!(!is_halloween_package("extracted-art-l6i06"));
}

#[test]
fn kwik_e_mart_reviewed_movement_preserves_fbx_import_basis()
-> Result<(), String> {
    let (_, matrix) =
        movement_for_package("extracted-art-l4i01").ok_or_else(|| {
            String::from("Level 4 Kwik-E-Mart movement is missing")
        })?;
    let source = [492.979_58_f32, -20.000_023_f32, -307.126_68_f32];
    let moved = [
        source[0].mul_add(
            matrix[0],
            source[1]
                .mul_add(matrix[4], source[2].mul_add(matrix[8], matrix[12])),
        ),
        source[0].mul_add(
            matrix[1],
            source[1]
                .mul_add(matrix[5], source[2].mul_add(matrix[9], matrix[13])),
        ),
        source[0].mul_add(
            matrix[2],
            source[1]
                .mul_add(matrix[6], source[2].mul_add(matrix[10], matrix[14])),
        ),
    ];
    let blender_import = [-moved[0], moved[2], moved[1]];
    let expected = [
        203.703_92_f32,
        -301.955_6_f32,
        5.173_32_f32 + WORLD_HEIGHT_OFFSET_METERS,
    ];
    if blender_import
        .iter()
        .zip(expected)
        .any(|(actual, wanted)| (*actual - wanted).abs() > 0.001)
    {
        return Err(format!(
            "Kwik-E-Mart import basis changed: {blender_import:?}"
        ));
    }
    let (_, level_one_matrix) = movement_for_package("extracted-art-l1i01")
        .ok_or_else(|| {
            String::from("Level 1 Kwik-E-Mart movement is missing")
        })?;
    if level_one_matrix
        .iter()
        .zip(matrix)
        .any(|(left, right)| (*left - right).abs() > 0.000_001)
    {
        return Err(String::from(
            "Level 1 and Level 4 Kwik-E-Mart placements diverged",
        ));
    }
    Ok(())
}

#[test]
fn recurring_interior_family_origins_are_cancelled() -> Result<(), String> {
    for (package, translation_index, expected) in [
        ("extracted-art-l2i03", 14_usize, -611.094_24_f32),
        ("extracted-art-l2i04", 12_usize, -915.175_8_f32),
        ("extracted-art-l3i05", 14_usize, 267.007_8_f32),
        ("extracted-art-l3i06", 14_usize, 276.835_94_f32),
        ("extracted-art-l5i03", 14_usize, -611.094_7_f32),
        ("extracted-art-l5i04", 12_usize, -915.175_8_f32),
        ("extracted-art-l6i05", 14_usize, 267.007_8_f32),
        ("extracted-art-l6i06", 14_usize, 276.835_94_f32),
    ] {
        let (_, matrix) = movement_for_package(package).ok_or_else(|| {
            format!("interior movement is missing: {package}")
        })?;
        let actual = *matrix.get(translation_index).ok_or_else(|| {
            format!("interior translation index is missing: {package}")
        })?;
        if (actual - expected).abs() > 0.001 {
            return Err(format!(
                "recurring family origin remained for {package}: \
                     {actual} != {expected}"
            ));
        }
    }
    Ok(())
}

#[test]
fn geometry_key_ignores_vertex_order_but_preserves_position()
-> Result<(), String> {
    let first_group = PrimitiveGroup::new(
        0,
        "material",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("first group failed: {error:?}"))?;
    let reordered_group = PrimitiveGroup::new(
        0,
        "other-material",
        vec![[0., 1., 0.], [0., 0., 0.], [1., 0., 0.]],
        Vec::new(),
        &[1, 2, 0],
    )
    .map_err(|error| format!("reordered group failed: {error:?}"))?;
    let shifted_group = PrimitiveGroup::new(
        0,
        "material",
        vec![[0.002, 0., 0.], [1.002, 0., 0.], [0.002, 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("shifted group failed: {error:?}"))?;
    let first = MeshAsset::new("first", vec![first_group])
        .map_err(|error| format!("first mesh failed: {error:?}"))?;
    let reordered = MeshAsset::new("reordered", vec![reordered_group])
        .map_err(|error| format!("reordered mesh failed: {error:?}"))?;
    let shifted = MeshAsset::new("shifted", vec![shifted_group])
        .map_err(|error| format!("shifted mesh failed: {error:?}"))?;
    let first_key = geometry_key(&first).map_err(|error| error.to_string())?;
    let reordered_key =
        geometry_key(&reordered).map_err(|error| error.to_string())?;
    let shifted_key =
        geometry_key(&shifted).map_err(|error| error.to_string())?;
    if first_key != reordered_key {
        return Err(String::from("vertex ordering changed the geometry key"));
    }
    if first_key == shifted_key {
        return Err(String::from(
            "world placement did not change the geometry key",
        ));
    }
    Ok(())
}

#[test]
fn final_movement_uses_only_the_canonical_world_height() -> Result<(), String> {
    let (_, reviewed) = reviewed_movement_for_package("extracted-art-l1i00")
        .ok_or_else(|| String::from("reviewed movement is missing"))?;
    let (_, final_matrix) = movement_for_package("extracted-art-l1i00")
        .ok_or_else(|| String::from("final movement is missing"))?;
    for (index, (reviewed_value, final_value)) in
        reviewed.into_iter().zip(final_matrix).enumerate()
    {
        let expected_delta = if index == 13 {
            WORLD_HEIGHT_OFFSET_METERS - LEGACY_REVIEWED_HEIGHT_OFFSET_METERS
        } else {
            0.
        };
        if (final_value - reviewed_value - expected_delta).abs() > 0.000_01 {
            return Err(format!(
                "movement component {index} changed unexpectedly"
            ));
        }
    }
    Ok(())
}

#[test]
fn halloween_mixed_mesh_retains_only_new_triangles() -> Result<(), String> {
    let base_group = PrimitiveGroup::new(
        0,
        "base-material",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("base group failed: {error:?}"))?;
    let mixed_group = PrimitiveGroup::new(
        0,
        "halloween-material",
        vec![[0.004, 0., 0.], [1.004, 0., 0.], [0.004, 1., 0.], [
            1., 1., 0.,
        ]],
        Vec::new(),
        &[0, 1, 2, 1, 3, 2],
    )
    .map_err(|error| format!("mixed group failed: {error:?}"))?;
    let base = MeshAsset::new("base", vec![base_group])
        .map_err(|error| format!("base mesh failed: {error:?}"))?;
    let mixed = MeshAsset::new("mixed", vec![mixed_group])
        .map_err(|error| format!("mixed mesh failed: {error:?}"))?;
    let mut owned = InteriorGeometryOwnership::default();
    let (retained_base, removed_base) =
        retain_unowned_triangles(base, &mut owned)
            .map_err(|error| error.to_string())?;
    if retained_base.is_none() || removed_base != 0 {
        return Err(String::from("canonical base triangle was not retained"));
    }
    let (retained_overlay_option, removed_overlay) =
        retain_unowned_triangles(mixed, &mut owned)
            .map_err(|error| error.to_string())?;
    let retained_overlay = retained_overlay_option
        .ok_or_else(|| String::from("Halloween addition was removed"))?;
    if removed_overlay != 1 {
        return Err(format!(
            "expected one repeated base triangle, found \
                 {removed_overlay}"
        ));
    }
    let retained_group = retained_overlay
        .groups
        .first()
        .ok_or_else(|| String::from("Halloween overlay group is missing"))?;
    if retained_overlay.groups.len() != 1 || retained_group.triangles.len() != 1
    {
        return Err(String::from(
            "Halloween overlay did not retain exactly one new triangle",
        ));
    }
    if retained_group.shader != "halloween-material" {
        return Err(String::from(
            "Halloween triangle lost its original material authority",
        ));
    }
    Ok(())
}

#[test]
fn alternate_planar_triangulation_reuses_owned_surface() -> Result<(), String> {
    let base_group = PrimitiveGroup::new(
        0,
        "base-material",
        vec![[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2, 0, 2, 3],
    )
    .map_err(|error| format!("base square failed: {error:?}"))?;
    let alternate_group = PrimitiveGroup::new(
        0,
        "halloween-material",
        vec![[0.004, 0., 0.], [1.004, 0., 0.], [1.004, 1., 0.], [
            0.004, 1., 0.,
        ]],
        Vec::new(),
        &[0, 1, 3, 1, 2, 3],
    )
    .map_err(|error| format!("alternate square failed: {error:?}"))?;
    let base = MeshAsset::new("base-square", vec![base_group])
        .map_err(|error| format!("base mesh failed: {error:?}"))?;
    let alternate =
        MeshAsset::new("alternate-square", vec![alternate_group])
            .map_err(|error| format!("alternate mesh failed: {error:?}"))?;
    let mut owned = InteriorGeometryOwnership::default();
    let (retained_base, removed_base) =
        retain_unowned_triangles(base, &mut owned)
            .map_err(|error| error.to_string())?;
    if retained_base.is_none() || removed_base != 0 {
        return Err(String::from("canonical square was not retained"));
    }
    let (retained_alternate, removed_alternate) =
        retain_unowned_triangles(alternate, &mut owned)
            .map_err(|error| error.to_string())?;
    if retained_alternate.is_some() || removed_alternate != 2 {
        return Err(format!(
            "alternate planar triangulation remained: retained={}, \
                 removed={removed_alternate}",
            retained_alternate.is_some(),
        ));
    }
    Ok(())
}

#[test]
fn uncovered_planar_span_with_owned_vertices_is_retained() -> Result<(), String>
{
    let base_group = PrimitiveGroup::new(
        0,
        "base-material",
        vec![
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 1., 0.],
            [3., 0., 0.],
            [4., 0., 0.],
            [4., 1., 0.],
        ],
        Vec::new(),
        &[0, 1, 2, 3, 4, 5],
    )
    .map_err(|error| format!("separated base failed: {error:?}"))?;
    let spanning_group = PrimitiveGroup::new(
        0,
        "new-material",
        vec![[1., 0., 0.], [3., 0., 0.], [4., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("planar span failed: {error:?}"))?;
    let base = MeshAsset::new("separated-base", vec![base_group])
        .map_err(|error| format!("base mesh failed: {error:?}"))?;
    let spanning = MeshAsset::new("planar-span", vec![spanning_group])
        .map_err(|error| format!("spanning mesh failed: {error:?}"))?;
    let mut owned = InteriorGeometryOwnership::default();
    let (retained_base, removed_base) =
        retain_unowned_triangles(base, &mut owned)
            .map_err(|error| error.to_string())?;
    if retained_base.is_none() || removed_base != 0 {
        return Err(String::from("separated base was not retained"));
    }
    let (retained_spanning_option, removed_spanning) =
        retain_unowned_triangles(spanning, &mut owned)
            .map_err(|error| error.to_string())?;
    let retained_spanning = retained_spanning_option
        .ok_or_else(|| String::from("uncovered planar span was removed"))?;
    let retained_group = retained_spanning.groups.first().ok_or_else(|| {
        String::from("uncovered planar span group is missing")
    })?;
    if removed_spanning != 0
        || retained_spanning.groups.len() != 1
        || retained_group.triangles.len() != 1
    {
        return Err(format!(
            "uncovered planar span changed: groups={}, \
                 removed={removed_spanning}",
            retained_spanning.groups.len(),
        ));
    }
    Ok(())
}
