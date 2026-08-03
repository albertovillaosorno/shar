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

use super::discover;
use crate::domain::mesh::PrimitiveGroup;

#[test]
fn merges_multiple_disconnected_islands_per_eye_side() {
    let positions = vec![
        [-2.2, 0., 0.],
        [-2., 0.2, 0.],
        [-1.8, 0., 0.],
        [-1.7, 0., 0.],
        [-1.5, 0.2, 0.],
        [-1.3, 0., 0.],
        [1.3, 0., 0.],
        [1.5, 0.2, 0.],
        [1.7, 0., 0.],
        [1.8, 0., 0.],
        [2., 0.2, 0.],
        [2.2, 0., 0.],
    ];
    let indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let group_result =
        PrimitiveGroup::new(0, "eye", positions, Vec::new(), &indices);
    assert!(
        group_result.is_ok(),
        "fixture group failed: {group_result:?}"
    );
    let Ok(group) = group_result else {
        return;
    };
    let discovery_result = discover(&group);
    assert!(
        discovery_result.is_ok(),
        "eye discovery failed: {discovery_result:?}"
    );
    let Ok(components) = discovery_result else {
        return;
    };
    let [left, right] = components.as_slice() else {
        assert_eq!(components.len(), 2);
        return;
    };

    assert_eq!(left.vertex_indices.len(), 6);
    assert_eq!(right.vertex_indices.len(), 6);
    assert!(left.centroid_x < right.centroid_x);
}
