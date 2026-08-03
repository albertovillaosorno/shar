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

use super::portable_name;

#[test]
fn portable_world_name_has_no_hash_suffix() {
    assert_eq!(
        portable_name("Cypress Tree!"),
        Ok("cypress-tree".to_owned())
    );
}

#[test]
fn phone_stops_publish_as_readable_phone_booths() {
    assert_eq!(
        portable_name("l1_phonestop"),
        Ok("l1-phone-booth".to_owned())
    );
}
