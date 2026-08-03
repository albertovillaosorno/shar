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

use super::contains_banned_token;

#[test]
fn standalone_scope_rejects_collection_effect_identities() {
    let banned = ["collect", "glow", "particle", "effect", "quad"];
    assert!(contains_banned_token("wrench_collect", &banned));
    assert!(contains_banned_token("circelglowShape", &banned));
    assert!(!contains_banned_token("wrench7Shape", &banned));
    assert!(!contains_banned_token("wrench46", &banned));
}
