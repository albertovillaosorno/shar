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

use fbx::adapters::driven::binary_structural_guide_writer as fbx_guide;

use super::model::GuideSourceCounts;
use super::{manifest, validate_manifest};

#[test]
fn rendered_manifest_satisfies_publication_validator() -> Result<(), String> {
    let bytes = manifest::render(
        fbx_guide::StructuralGuideFbxSummary {
            vertices: 3,
            triangles: 1,
            bounds_min_meters: [-1., 79., -2.],
            bounds_max_meters: [1., 81., 2.],
        },
        GuideSourceCounts {
            input_meshes: 2,
            input_groups: 2,
            groups_without_normals: 0,
            input_triangles: 1,
            removed_duplicate_triangles: 0,
            removed_degenerate_triangles: 0,
            repaired_normal_triangles: 0,
            wasp_meshes: 0,
            prop_like_meshes: 0,
            approximated_vertex_color_triangles: 0,
        },
        &"a".repeat(64),
        &"b".repeat(64),
        &"c".repeat(64),
    )
    .map_err(|error| error.to_string())?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    assert_eq!(
        value
            .pointer("/worldFbxScene/guideExportRootPolicy")
            .and_then(serde_json::Value::as_str),
        Some("ReflectX"),
    );
    assert_eq!(
        value
            .pointer("/worldFbxScene/guideExportRootScale/0")
            .and_then(serde_json::Value::as_f64),
        Some(-1.),
    );
    assert_eq!(
        value
            .pointer("/worldFbxScene/exteriorExportRootPolicy")
            .and_then(serde_json::Value::as_str),
        Some("ReflectX"),
    );
    assert_eq!(
        value
            .pointer("/worldFbxScene/interiorExportRootPolicy")
            .and_then(serde_json::Value::as_str),
        Some("ReflectX"),
    );
    assert_eq!(
        value
            .pointer("/worldFbxScene/worldReflectionAxis")
            .and_then(serde_json::Value::as_str),
        Some("X"),
    );
    assert_eq!(
        value
            .pointer("/worldFbxScene/sourceRootsFlattenedIntoGuideMesh")
            .and_then(serde_json::Value::as_bool),
        Some(false),
    );
    validate_manifest(&value).map_err(|error| error.to_string())
}
