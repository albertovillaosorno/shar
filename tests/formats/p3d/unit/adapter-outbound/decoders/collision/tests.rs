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

use super::*;

/// Builds a synthetic chunk for count-checked decoder tests.
fn chunk(id: u32, fields: Vec<u8>, children: Vec<Vec<u8>>) -> Option<Vec<u8>> {
    let header_size = 12_usize.checked_add(fields.len())?;
    let child_size = children
        .iter()
        .map(Vec::len)
        .try_fold(0_usize, usize::checked_add)?;
    let total_size = header_size.checked_add(child_size)?;
    let mut out = Vec::new();
    out.extend_from_slice(&id.to_le_bytes());
    out.extend_from_slice(&u32::try_from(header_size).ok()?.to_le_bytes());
    out.extend_from_slice(&u32::try_from(total_size).ok()?.to_le_bytes());
    out.extend(fields);
    for child in children {
        out.extend(child);
    }
    Some(out)
}

/// Builds a Pure3D-style test string payload.
fn pstring(value: &str) -> Option<Vec<u8>> {
    let length = u8::try_from(value.len()).ok()?;
    let mut out = Vec::new();
    out.push(length);
    out.extend_from_slice(value.as_bytes());
    Some(out)
}

/// Builds a little-endian integer field.
fn u32_field(value: u32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a little-endian short field.
fn u16_field(value: u16) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a little-endian float field.
fn f32_field(value: f32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a vector payload.
fn vec3(x: f32, y: f32, z: f32) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&f32_field(x));
    out.extend_from_slice(&f32_field(y));
    out.extend_from_slice(&f32_field(z));
    out
}

/// Joins field fragments in schema order.
fn fields(parts: Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = Vec::new();
    for part in parts {
        out.extend(part);
    }
    out
}

/// Converts optional fixtures into descriptive test errors.
fn require<T>(value: Option<T>, context: &str) -> Result<T, String> {
    value.ok_or_else(|| String::from(context))
}

/// Checks useful JSON output without panicking in tests.
fn require_json(json: &str, needle: &str, context: &str) -> Result<(), String> {
    if json.contains(needle) {
        Ok(())
    } else {
        Err(String::from(context))
    }
}

/// Builds a collision object with owner, volume, and attributes.
fn collision_fixture() -> Option<Vec<u8>> {
    collision_fixture_with_contract(1, 1, 1, 1)
}

/// Builds a collision object with an explicit subvolume count.
fn collision_fixture_with_subvolume_count(
    subvolume_count: u32,
) -> Option<Vec<u8>> {
    collision_fixture_with_contract(subvolume_count, 1, 1, 1)
}

/// Builds a collision object with explicit source-contract fields.
fn collision_fixture_with_contract(
    subvolume_count: u32,
    version: u32,
    volume_copies: usize,
    attribute_copies: usize,
) -> Option<Vec<u8>> {
    let owner_name =
        chunk(COLLISION_OWNER_NAME, pstring("joint_a")?, Vec::new())?;
    let owner = chunk(COLLISION_OWNER, u32_field(1), vec![owner_name])?;
    let self_collision = chunk(
        SELF_COLLISION,
        fields(vec![u32_field(1), u32_field(2), u16_field(1), u16_field(0)]),
        Vec::new(),
    )?;
    let vector = chunk(COLLISION_VECTOR, vec3(1., 2., 3.), Vec::new())?;
    let sphere = chunk(COLLISION_SPHERE, f32_field(5.), vec![vector])?;
    let leaf_volume = chunk(
        COLLISION_VOLUME,
        fields(vec![u32_field(9), u32_field(u32::MAX), u32_field(0)]),
        vec![sphere],
    )?;
    let bounds = chunk(COLLISION_BBOX, u32_field(0), Vec::new())?;
    let volume = chunk(
        COLLISION_VOLUME,
        fields(vec![u32_field(9), u32_field(0), u32_field(subvolume_count)]),
        vec![bounds, leaf_volume],
    )?;
    let attribute = chunk(
        COLLISION_ATTRIBUTE,
        fields(vec![
            u16_field(1),
            u32_field(7),
            u16_field(1),
            u16_field(0),
            u16_field(1),
            u16_field(0),
            u32_field(10),
            u32_field(11),
            u32_field(12),
        ]),
        Vec::new(),
    )?;
    let mut children = vec![owner, self_collision];
    for _ in 0..volume_copies {
        children.push(volume.clone());
    }
    for _ in 0..attribute_copies {
        children.push(attribute.clone());
    }
    chunk(
        COLLISION_OBJECT,
        fields(vec![
            pstring("collider")?,
            u32_field(version),
            pstring("material")?,
            u32_field(1),
            u32_field(1),
        ]),
        children,
    )
}

/// Builds a physics object with sparse joint records.
fn physics_fixture() -> Option<Vec<u8>> {
    physics_fixture_with_joint_indices(7, &[0, 2, 5])
}

/// Builds one explicit physics-joint record.
fn physics_joint_fixture(index: u32) -> Option<Vec<u8>> {
    let joint_vector = chunk(PHYSICS_VECTOR, vec3(4., 5., 6.), Vec::new())?;
    chunk(
        PHYSICS_JOINT,
        fields(vec![
            u32_field(index),
            f32_field(8.),
            f32_field(0.25),
            f32_field(45.),
            f32_field(-45.),
            u32_field(2),
        ]),
        vec![joint_vector],
    )
}

/// Builds a physics object with an explicit joint-index upper bound.
fn physics_fixture_with_joint_indices(
    joint_count: u32,
    joint_indices: &[u32],
) -> Option<Vec<u8>> {
    let center = chunk(PHYSICS_VECTOR, vec3(0.5, 1.5, 2.5), Vec::new())?;
    let inertia = chunk(
        PHYSICS_INERTIA,
        fields(vec![
            f32_field(1.),
            f32_field(0.1),
            f32_field(0.2),
            f32_field(2.),
            f32_field(0.3),
            f32_field(3.),
        ]),
        Vec::new(),
    )?;
    let mut children = vec![center, inertia];
    for index in joint_indices {
        children.push(physics_joint_fixture(*index)?);
    }
    chunk(
        PHYSICS_OBJECT,
        fields(vec![
            pstring("physics")?,
            u32_field(1),
            pstring("heavy")?,
            u32_field(joint_count),
            f32_field(12.),
            f32_field(0.75),
        ]),
        children,
    )
}

/// Builds a dynamic physics DSG wrapper fixture.
fn dsg_fixture() -> Option<Vec<u8>> {
    let mesh = chunk(
        MESH,
        fields(vec![pstring("mesh_ref")?, u32_field(0)]),
        Vec::new(),
    )?;
    chunk(
        DYNA_PHYS_DSG,
        fields(vec![pstring("crate_phys")?, u32_field(0), u32_field(1)]),
        vec![mesh, collision_fixture()?, physics_fixture()?],
    )
}

/// Builds an instanced animated dynamic-physics wrapper.
fn insta_anim_dyna_fixture() -> Option<Vec<u8>> {
    let wrapper = chunk(
        ANIM_OBJ_DSG_WRAPPER,
        fields(vec![pstring("animated_wrapper")?, vec![0, 1]]),
        vec![physics_fixture()?],
    )?;
    chunk(
        INSTA_ANIM_DYNA_PHYS_DSG,
        fields(vec![pstring("animated_root")?, u32_field(0), u32_field(1)]),
        vec![wrapper],
    )
}

/// Builds a chunk set with one child member.
fn chunk_set_fixture() -> Option<Vec<u8>> {
    let child_name = pstring("member_texture")?;
    let child = chunk(
        TEXTURE,
        fields(vec![
            child_name,
            u32_field(0),
            u32_field(64),
            u32_field(32),
            u32_field(32),
            u32_field(8),
            u32_field(1),
            u32_field(0),
            u32_field(0),
            u32_field(0),
        ]),
        Vec::new(),
    )?;
    chunk(
        CHUNK_SET,
        fields(vec![pstring("set_a")?, u32_field(0), vec![1_u8]]),
        vec![child],
    )
}

#[test]
fn collision_object_decodes_volume_owner_and_attributes() -> Result<(), String>
{
    let fixture =
        require(collision_fixture(), "collision fixture should build")?;
    let json =
        require(object_json(&fixture), "collision fixture should decode")?;
    require_json(
        &json,
        "\"schema\":\"simulation_collision_object\"",
        "schema should be emitted",
    )?;
    require_json(
        &json,
        "\"names\":[\"joint_a\"]",
        "owner name should be emitted",
    )?;
    require_json(
        &json,
        "\"kind\":\"sphere\"",
        "sphere primitive should be emitted",
    )?;
    require_json(
        &json,
        "\"num_subvolumes\":1",
        "nested volume count should be emitted",
    )?;
    require_json(
        &json,
        "\"bounds\":[{\"kind\":\"bbox\"",
        "bounding metadata should be emitted separately",
    )?;
    require_json(&json, "\"radius\":5.0", "sphere radius should be emitted")?;
    require_json(&json, "\"can_spin\":1", "attribute flags should be emitted")?;
    Ok(())
}

#[test]
fn collision_object_rejects_unobserved_version() -> Result<(), String> {
    let fixture = require(
        collision_fixture_with_contract(1, 0, 1, 1),
        "unobserved-version collision fixture should build",
    )?;
    if object_json(&fixture).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "unobserved collision version should fail closed",
        ))
    }
}

#[test]
fn collision_object_rejects_missing_top_volume() -> Result<(), String> {
    let fixture = require(
        collision_fixture_with_contract(1, 1, 0, 1),
        "missing-volume collision fixture should build",
    )?;
    if object_json(&fixture).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "missing top collision volume should fail closed",
        ))
    }
}

#[test]
fn collision_object_rejects_duplicate_attributes() -> Result<(), String> {
    let fixture = require(
        collision_fixture_with_contract(1, 1, 1, 2),
        "duplicate-attribute collision fixture should build",
    )?;
    if object_json(&fixture).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "duplicate collision attributes should fail closed",
        ))
    }
}

#[test]
fn collision_volume_rejects_declared_subvolume_count_mismatch()
-> Result<(), String> {
    let fixture = require(
        collision_fixture_with_subvolume_count(2),
        "collision fixture should build",
    )?;
    if object_json(&fixture).is_some() {
        return Err(String::from(
            "collision volume decoder must reject a declared \
                 subvolume count that does not match child chunks",
        ));
    }
    Ok(())
}

#[test]
fn physics_object_decodes_mass_and_joint_parameters() -> Result<(), String> {
    let fixture = require(physics_fixture(), "physics fixture should build")?;
    let json =
        require(physics_json(&fixture), "physics fixture should decode")?;
    require_json(
        &json,
        "\"schema\":\"simulation_physics_object\"",
        "schema should be emitted",
    )?;
    require_json(&json, "\"volume\":12.0", "object volume should be emitted")?;
    require_json(
        &json,
        "\"resting_sensitivity\":0.75",
        "resting sensitivity should be emitted",
    )?;
    require_json(
        &json,
        "\"stiffness\":0.25",
        "joint stiffness should be emitted",
    )?;
    require_json(
        &json,
        "\"dof\":2",
        "joint degree of freedom should be emitted",
    )?;
    Ok(())
}

#[test]
fn physics_object_rejects_unobserved_version() -> Result<(), String> {
    let mut fixture = require(physics_fixture(), "physics fixture should build")?;
    let name_length = usize::from(*fixture.get(12).ok_or_else(|| {
        String::from("physics fixture name length should exist")
    })?);
    let version_offset = 13_usize
        .checked_add(name_length)
        .ok_or_else(|| String::from("physics version offset overflowed"))?;
    require(
        fixture.get_mut(version_offset..version_offset + 4),
        "physics version field should exist",
    )?
    .copy_from_slice(&0_u32.to_le_bytes());
    if physics_json(&fixture).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "unobserved physics version should fail closed",
        ))
    }
}

#[test]
fn physics_object_accepts_sparse_joint_records() -> Result<(), String> {
    let fixture =
        require(physics_fixture(), "sparse physics fixture should build")?;
    let json = require(
        physics_json(&fixture),
        "sparse physics fixture should decode",
    )?;
    require_json(
        &json,
        "\"num_joints\":7",
        "joint-index upper bound should be emitted",
    )?;
    for index in [0_u32, 2, 5] {
        require_json(
            &json,
            &format!("\"index\":{index}"),
            "sparse joint record should be emitted",
        )?;
    }
    Ok(())
}

#[test]
fn physics_object_rejects_joint_index_outside_declared_bound()
-> Result<(), String> {
    let fixture = require(
        physics_fixture_with_joint_indices(3, &[3]),
        "out-of-range physics fixture should build",
    )?;
    if physics_json(&fixture).is_some() {
        return Err(String::from(
            "physics decoder must reject a joint index outside the \
                 declared upper bound",
        ));
    }
    Ok(())
}

#[test]
fn physics_object_rejects_duplicate_joint_indices() -> Result<(), String> {
    let fixture = require(
        physics_fixture_with_joint_indices(4, &[2, 2]),
        "duplicate-joint physics fixture should build",
    )?;
    if physics_json(&fixture).is_some() {
        return Err(String::from(
            "physics decoder must reject duplicate sparse joint \
                 indices",
        ));
    }
    Ok(())
}

#[test]
fn instanced_animated_dynamic_wrapper_decodes_sparse_physics()
-> Result<(), String> {
    let fixture = require(
        insta_anim_dyna_fixture(),
        "instanced animated dynamic fixture should build",
    )?;
    let json = require(
        dsg_json(&fixture, "insta_anim_dyna_phys_dsg"),
        "instanced animated dynamic fixture should decode",
    )?;
    require_json(
        &json,
        "\"schema\":\"insta_anim_dyna_phys_dsg\"",
        "instanced animated dynamic schema should be emitted",
    )?;
    require_json(
        &json,
        "\"num_joints\":7",
        "nested sparse physics should be emitted",
    )?;
    Ok(())
}

#[test]
fn dsg_wrapper_decodes_render_collision_and_physics_children()
-> Result<(), String> {
    let fixture = require(dsg_fixture(), "dsg fixture should build")?;
    let json = require(
        dsg_json(&fixture, "dyna_phys_dsg"),
        "dsg fixture should decode",
    )?;
    require_json(
        &json,
        "\"schema\":\"dyna_phys_dsg\"",
        "schema should be emitted",
    )?;
    require_json(&json, "\"has_alpha\":1", "alpha flag should be emitted")?;
    require_json(
        &json,
        "\"name\":\"mesh_ref\"",
        "render ref should be emitted",
    )?;
    require_json(
        &json,
        "\"collision_objects\":[{",
        "collision child should be embedded",
    )?;
    require_json(
        &json,
        "\"physics_objects\":[{",
        "physics child should be embedded",
    )?;
    Ok(())
}

#[test]
fn chunk_set_decodes_texture_membership() -> Result<(), String> {
    let fixture =
        require(chunk_set_fixture(), "chunk-set fixture should build")?;
    let json =
        require(chunk_set_json(&fixture), "chunk-set fixture should decode")?;
    require_json(
        &json,
        "\"kind\":\"texture\"",
        "texture child should be typed",
    )?;
    require_json(
        &json,
        "member_texture",
        "texture child name should be emitted",
    )?;
    Ok(())
}

#[test]
fn empty_chunk_set_decodes_header() -> Result<(), String> {
    let name =
        require(pstring("set_empty"), "empty chunk-set name should encode")?;
    let fixture = require(
        chunk(
            CHUNK_SET,
            fields(vec![name, u32_field(0), vec![0_u8]]),
            Vec::new(),
        ),
        "empty chunk-set fixture should build",
    )?;
    let json = require(
        chunk_set_json(&fixture),
        "empty chunk-set fixture should decode",
    )?;
    require_json(
        &json,
        "\"schema\":\"chunk_set\"",
        "schema should be emitted",
    )?;
    require_json(
        &json,
        "\"child_count\":0",
        "empty child count should be emitted",
    )?;
    Ok(())
}
