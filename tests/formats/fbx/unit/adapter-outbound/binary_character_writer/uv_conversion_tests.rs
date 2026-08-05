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
//   - Uv conversion tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Uv conversion tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Uv conversion tests unit tests.

use super::source_uv_to_fbx;

/// Assert exact deterministic UV components without float comparison.
fn assert_uv_bits(actual: [f64; 2], expected: [f64; 2]) {
    assert_eq!(actual.map(f64::to_bits), expected.map(f64::to_bits));
}

#[test]
fn preserves_authored_uv_coordinates_exactly() {
    assert_uv_bits(source_uv_to_fbx([0.25_f32, 0.75_f32]), [
        0.25_f64, 0.75_f64,
    ]);
    assert_uv_bits(source_uv_to_fbx([2f32, -1f32]), [2f64, -1f64]);
}
