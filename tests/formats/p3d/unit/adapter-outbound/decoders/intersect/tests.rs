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

/// Builds a synthetic chunk for fail-closed decoder tests.
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

/// Builds a little-endian integer fixture field.
fn u32_field(value: u32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a little-endian float fixture field.
fn f32_field(value: f32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a vector fixture field.
fn vec3(x: f32, y: f32, z: f32) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&f32_field(x));
    out.extend_from_slice(&f32_field(y));
    out.extend_from_slice(&f32_field(z));
    out
}

/// Converts optional fixture construction into a test error.
fn require<T>(value: Option<T>, context: &str) -> Result<T, String> {
    value.ok_or_else(|| String::from(context))
}

/// Checks useful JSON fields without panicking inside tests.
fn require_json(json: &str, needle: &str, context: &str) -> Result<(), String> {
    if json.contains(needle) {
        Ok(())
    } else {
        Err(String::from(context))
    }
}

/// Builds an intersect fixture with arrays and child chunks.
fn intersect_fixture() -> Option<Vec<u8>> {
    let mut fields = Vec::new();
    fields.extend_from_slice(&u32_field(3));
    fields.extend_from_slice(&u32_field(0));
    fields.extend_from_slice(&u32_field(1));
    fields.extend_from_slice(&u32_field(2));
    fields.extend_from_slice(&u32_field(3));
    fields.extend_from_slice(&vec3(0f32, 1f32, 2f32));
    fields.extend_from_slice(&vec3(3f32, 4f32, 5f32));
    fields.extend_from_slice(&vec3(6f32, 7f32, 8f32));
    fields.extend_from_slice(&u32_field(1));
    fields.extend_from_slice(&vec3(0f32, 1f32, 0f32));
    let mut bbox = Vec::new();
    bbox.extend_from_slice(&vec3(-1f32, -2f32, -3f32));
    bbox.extend_from_slice(&vec3(9f32, 8f32, 7f32));
    let mut terrain = Vec::new();
    terrain.extend_from_slice(&u32_field(0));
    terrain.extend_from_slice(&u32_field(1));
    terrain.push(4_u8);
    let mut sphere = Vec::new();
    sphere.extend_from_slice(&vec3(4f32, 3f32, 2f32));
    sphere.extend_from_slice(&f32_field(9f32));
    chunk(INTERSECT_DSG, fields, vec![
        chunk(TERRAIN_TYPE, terrain, Vec::new())?,
        chunk(BSPHERE, sphere, Vec::new())?,
        chunk(BBOX, bbox, Vec::new())?,
    ])
}

#[test]
fn intersect_dsg_decodes_arrays_and_children() -> Result<(), String> {
    let fixture =
        require(intersect_fixture(), "intersect fixture should build")?;
    let json = require(dsg_json(&fixture), "intersect fixture should decode")?;
    require_json(&json, "\"indices\":[0,1,2]", "indices should be emitted")?;
    require_json(
        &json,
        "\"positions\":[[0.0,1.0,2.0]",
        "positions should be emitted",
    )?;
    require_json(
        &json,
        "\"normals\":[[0.0,1.0,0.0]]",
        "normals should be emitted",
    )?;
    require_json(&json, "\"kind\":\"bbox\"", "bbox child should be emitted")?;
    require_json(&json, "\"types\":[4]", "terrain bytes should be emitted")?;
    Ok(())
}

#[test]
fn intersect_dsg_rejects_impossible_array_counts() -> Result<(), String> {
    for count_offset in [12_usize, 28, 68] {
        let mut fixture =
            require(intersect_fixture(), "intersect fixture should build")?;
        require(
            fixture.get_mut(count_offset..count_offset + 4),
            "intersect count field should exist",
        )?
        .copy_from_slice(&u32::MAX.to_le_bytes());
        if dsg_json(&fixture).is_some() {
            return Err(String::from(
                "intersect accepted an impossible inline array count",
            ));
        }
    }
    Ok(())
}

#[test]
fn intersect_dsg_rejects_missing_required_sphere() -> Result<(), String> {
    let fixture =
        require(intersect_fixture(), "intersect fixture should build")?;
    let (_, header_size, total_size) = require(
        chunk_bounds(&fixture),
        "intersect fixture bounds should decode",
    )?;
    let children = require(
        subchunks(&fixture, header_size, total_size),
        "intersect fixture children should decode",
    )?;
    let sphere = children
        .iter()
        .find(|child| child.id == BSPHERE)
        .ok_or_else(|| String::from("fixture sphere should exist"))?;
    let mut malformed = fixture;
    let _removed = malformed.drain(sphere.offset..sphere.end()).count();
    let total =
        u32::try_from(malformed.len()).map_err(|error| error.to_string())?;
    require(
        malformed.get_mut(8..12),
        "fixture total-size field should exist",
    )?
    .copy_from_slice(&total.to_le_bytes());
    if dsg_json(&malformed).is_none() {
        Ok(())
    } else {
        Err(String::from("missing intersect sphere should fail closed"))
    }
}

#[test]
fn intersect_dsg_rejects_terrain_triangle_count_drift() -> Result<(), String> {
    let mut fixture =
        require(intersect_fixture(), "intersect fixture should build")?;
    let (_, header_size, total_size) = require(
        chunk_bounds(&fixture),
        "intersect fixture bounds should decode",
    )?;
    let children = require(
        subchunks(&fixture, header_size, total_size),
        "intersect fixture children should decode",
    )?;
    let terrain = children
        .iter()
        .find(|child| child.id == TERRAIN_TYPE)
        .ok_or_else(|| String::from("fixture terrain should exist"))?;
    let count_offset = terrain.data_offset() + 4;
    require(
        fixture.get_mut(count_offset..count_offset + 4),
        "fixture terrain count field should exist",
    )?
    .copy_from_slice(&2_u32.to_le_bytes());
    if dsg_json(&fixture).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "terrain triangle count drift should fail closed",
        ))
    }
}

#[test]
fn intersect_dsg_rejects_unobserved_terrain_version() -> Result<(), String> {
    let mut fixture =
        require(intersect_fixture(), "intersect fixture should build")?;
    let (_, header_size, total_size) = require(
        chunk_bounds(&fixture),
        "intersect fixture bounds should decode",
    )?;
    let children = require(
        subchunks(&fixture, header_size, total_size),
        "intersect fixture children should decode",
    )?;
    let terrain = children
        .iter()
        .find(|child| child.id == TERRAIN_TYPE)
        .ok_or_else(|| String::from("fixture terrain should exist"))?;
    let offset = terrain.data_offset();
    require(
        fixture.get_mut(offset..offset + 4),
        "fixture terrain version field should exist",
    )?
    .copy_from_slice(&1_u32.to_le_bytes());
    if dsg_json(&fixture).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "unobserved terrain version should fail closed",
        ))
    }
}

#[test]
fn intersect_dsg_fails_closed_on_truncated_array() -> Result<(), String> {
    let mut fixture =
        require(intersect_fixture(), "intersect fixture should build")?;
    let _removed: Option<u8> = fixture.pop();
    if dsg_json(&fixture).is_none() {
        Ok(())
    } else {
        Err(String::from("truncated array should fail closed"))
    }
}
