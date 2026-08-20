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

use super::{IntersectDocument, collision_mesh, topology_matches};

#[test]
fn coordinate_reference_requires_exact_original_topology() {
    let canonical = IntersectDocument {
        schema: "intersect_dsg".to_owned(),
        num_indices: 3,
        indices: vec![0, 1, 2],
        num_positions: 3,
        positions: vec![[0.; 3], [1.; 3], [2.; 3]],
    };
    let mut moved = canonical.clone();
    moved.positions = vec![[10.; 3], [11.; 3], [12.; 3]];
    assert!(topology_matches(&canonical, &moved));
    moved.indices = vec![0, 2, 1];
    assert!(!topology_matches(&canonical, &moved));
}


#[test]
// jig-ignore-next-line: long identifier
fn collision_mesh_rejects_repeated_index_source_triangles() -> Result<(), String> {
    let document = IntersectDocument {
        schema: "intersect_dsg".to_owned(),
        num_indices: 6,
        indices: vec![0, 1, 2, 2, 2, 3],
        num_positions: 4,
        positions: vec![[0.; 3], [1.; 3], [2.; 3], [3.; 3]],
    };
    let result = collision_mesh("package", "surface", document, None);
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("collision mesh altered repeated-index source topology".to_owned());
    };
    if !error.to_string().contains("RepeatedTriangleVertex") {
        return Err(format!("unexpected collision topology error: {error}"));
    }
    Ok(())
}
