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

use super::triangulate_indices;

#[test]
fn empty_index_stream_is_rejected() {
    let result = triangulate_indices(&[]);

    assert_eq!(result, Err(super::MeshError::UnsupportedIndexCount(0)));
}

#[test]
fn quad_triangles_preserve_winding() {
    let result = triangulate_indices(&[0, 1, 2, 3]);

    assert_eq!(result, Ok(vec![[0, 1, 2], [0, 2, 3]]));
}
