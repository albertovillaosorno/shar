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

use std::collections::BTreeSet;

use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

use super::{
    LevelMeshSource, PackageCoordinates, exact_reference_match, topology_matches,
    transplant_coordinates,
};

fn mesh(shader: &str, offset: f32) -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        shader,
        vec![[offset, 0., 0.], [offset + 1., 0., 0.], [offset, 1., 0.]],
        vec![[0., 0.], [1., 0.], [0., 1.]],
        &[0, 1, 2],
    )
    .map_err(|error| format!("group failed: {error:?}"))?
    .with_normals(vec![[0., 0., 1.], [0., 0., 1.], [0., 0., 1.]])
    .map_err(|error| format!("normals failed: {error:?}"))?;
    MeshAsset::new("mesh", vec![group])
        .map_err(|error| format!("mesh failed: {error:?}"))
}

fn source(ordinal: usize, mesh_name: &str, owner_name: &str) -> LevelMeshSource {
    LevelMeshSource {
        ordinal,
        member_id: format!("member-{ordinal}"),
        mesh_name: mesh_name.to_owned(),
        owner_name: owner_name.to_owned(),
        owner_kind: "srr_entity_dsg".to_owned(),
    }
}

type TestResult = Result<(), String>;

/// Return the first primitive group or a fixture error.
fn first_group(mesh: &MeshAsset) -> Result<&PrimitiveGroup, String> {
    mesh.groups
        .first()
        .ok_or_else(|| "fixture mesh has no primitive group".to_owned())
}

/// Return the first mutable primitive group or a fixture error.
fn first_group_mut(
    mesh: &mut MeshAsset,
) -> Result<&mut PrimitiveGroup, String> {
    mesh.groups
        .first_mut()
        .ok_or_else(|| "fixture mesh has no primitive group".to_owned())
}

/// Compare exact deterministic float arrays by bit pattern.
fn position_bits(value: [f32; 3]) -> [u32; 3] {
    value.map(f32::to_bits)
}

#[test]
fn source_preserving_policy_rejects_precomputed_movement() -> TestResult {
    let coordinates = PackageCoordinates::preserve_source();
    let source = LevelMeshSource {
        ordinal: 1,
        member_id: "interior-mesh".to_owned(),
        mesh_name: "interior-mesh".to_owned(),
        owner_name: "interior-owner".to_owned(),
        owner_kind: "srr_entity_dsg".to_owned(),
    };
    let original = mesh("interior-material", 7.)?;
    let mut candidate = original.clone();

    let (placements, uses_reference_placement) =
        coordinates.placements(&source);
    if !placements.is_empty() || uses_reference_placement {
        return Err(
            "source-only interior policy exposed a placement".to_owned(),
        );
    }
    if coordinates.uses_reference {
        return Err(
            String::from(
                "source-only interior policy exposed reference authority",
            ),
        );
    }
    let transplanted = coordinates
        .apply_direct_reference(&source, &mut candidate)
        .map_err(|error| error.to_string())?;
    if transplanted || candidate != original {
        return Err(
            "source-only interior policy changed mesh coordinates".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn coordinate_transplant_keeps_canonical_presentation() -> TestResult {
    let mut canonical = mesh("canonical-material", 0.)?;
    let reference = mesh("reference-material", 100.)?;
    if !topology_matches(&canonical, &reference) {
        return Err("compatible topology was rejected".to_owned());
    }
    transplant_coordinates(&mut canonical, &reference)
        .map_err(|error| error.to_string())?;
    let group = first_group(&canonical)?;
    if group.shader != "canonical-material" {
        return Err(format!("canonical shader changed: {}", group.shader));
    }
    let expected_uvs = vec![[0f32, 0f32], [1f32, 0f32], [0f32, 1f32]];
    if group.uvs != expected_uvs {
        return Err(format!("canonical UVs changed: {:?}", group.uvs));
    }
    let position = group
        .positions
        .first()
        .copied()
        .ok_or_else(|| "canonical group has no position".to_owned())?;
    if position_bits(position) != position_bits([100f32, 0f32, 0f32]) {
        return Err(format!(
            "reference position was not transplanted: {position:?}"
        ));
    }
    Ok(())
}

#[test]
fn topology_mismatch_blocks_coordinate_transplant() -> TestResult {
    let canonical = mesh("canonical-material", 0.)?;
    let mut reference = mesh("reference-material", 10.)?;
    let triangle = first_group_mut(&mut reference)?
        .triangles
        .first_mut()
        .ok_or_else(|| "reference group has no triangle".to_owned())?;
    *triangle = [0, 2, 1];
    if topology_matches(&canonical, &reference) {
        return Err("incompatible topology was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn topology_only_reference_identity_does_not_match() -> TestResult {
    let canonical = mesh("canonical-material", 0.)?;
    let canonical_source = source(1, "canonical-mesh", "canonical-owner");
    let reference_source = source(2, "reference-mesh", "reference-owner");
    let references = vec![(reference_source, canonical.clone())];
    let matched = exact_reference_match(
        &canonical_source,
        &canonical,
        &references,
        &BTreeSet::new(),
    )
    .map_err(|error| error.to_string())?;
    if matched.is_some() {
        return Err("topology-only reference identity was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn owner_only_reference_identity_does_not_match() -> TestResult {
    let canonical = mesh("canonical-material", 0.)?;
    let canonical_source = source(1, "canonical-mesh", "shared-owner");
    let reference_source = source(2, "reference-mesh", "shared-owner");
    let references = vec![(reference_source, canonical.clone())];
    let matched = exact_reference_match(
        &canonical_source,
        &canonical,
        &references,
        &BTreeSet::new(),
    )
    .map_err(|error| error.to_string())?;
    if matched.is_some() {
        return Err("owner-only reference identity was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn exact_reference_identity_matches_once() -> TestResult {
    let canonical = mesh("canonical-material", 0.)?;
    let canonical_source = source(1, "mesh", "owner");
    let reference_source = source(2, "mesh", "owner");
    let references = vec![(reference_source, canonical.clone())];
    let matched = exact_reference_match(
        &canonical_source,
        &canonical,
        &references,
        &BTreeSet::new(),
    )
    .map_err(|error| error.to_string())?;
    if matched != Some(0) {
        return Err(format!("exact reference identity did not match: {matched:?}"));
    }
    Ok(())
}

#[test]
fn ambiguous_exact_reference_identity_fails_closed() -> TestResult {
    let canonical = mesh("canonical-material", 0.)?;
    let canonical_source = source(1, "mesh", "owner");
    let reference = source(2, "mesh", "owner");
    let references = vec![
        (reference.clone(), canonical.clone()),
        (reference, canonical.clone()),
    ];
    let result = exact_reference_match(
        &canonical_source,
        &canonical,
        &references,
        &BTreeSet::new(),
    );
    let Err(error) = result else {
        return Err("ambiguous exact coordinate references were accepted".to_owned());
    };
    if !error.to_string().contains("identity is ambiguous") {
        return Err(format!("unexpected reference ambiguity error: {error}"));
    }
    Ok(())
}
