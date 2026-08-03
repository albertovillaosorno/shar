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

use super::{identity, matrix_key, multiply, translation};

#[test]
fn row_vector_composition_preserves_translation_order() {
    let first = translation([1., 2., 3.]);
    let second = translation([4., 5., 6.]);
    let product = multiply(&first, &second);
    assert_eq!(product[12..15], [5., 7., 9.]);
    assert_eq!(
        matrix_key(&multiply(&identity(), &first)),
        matrix_key(&first)
    );
}
