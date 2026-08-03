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

use super::{catalog_worker_count, catalog_worker_count_for};

#[test]
fn catalog_worker_count_is_nonzero_and_bounded() {
    assert_eq!(catalog_worker_count(0), 1);
    assert_eq!(catalog_worker_count(1), 1);
    assert!(catalog_worker_count(110) >= 1);
}

#[test]
fn catalog_worker_count_uses_two_thirds_of_logical_processors() {
    assert_eq!(catalog_worker_count_for(24, 110,), 16);
    assert_eq!(catalog_worker_count_for(8, 3,), 3);
    assert_eq!(catalog_worker_count_for(1, 110,), 1);
}
