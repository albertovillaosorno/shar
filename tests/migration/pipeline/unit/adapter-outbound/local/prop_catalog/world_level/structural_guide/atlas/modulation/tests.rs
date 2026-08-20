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

#[test]
fn material_average_tint_is_baked_into_opaque_rgb() {
    let result = bake(&[[255, 128, 64, 128]], 1, 1, [255, 128, 255, 255], [
        128, 255, 64, 255,
    ]);
    assert_eq!(result, Ok(vec![[64, 32, 8]]));
}

#[test]
fn source_pixel_count_must_match_dimensions() {
    let result = bake(&[], 1, 1, [255; 4], [255; 4]);
    assert!(result.is_err());
}
