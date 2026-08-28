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

use super::map_recovery_status;

#[test]
fn maps_decoded_and_image_payloads_to_fully_decoded() {
    assert_eq!(
        map_recovery_status("decoded_schema_payload"),
        "fully-decoded"
    );
    assert_eq!(
        map_recovery_status("recovered_embedded_image_payload"),
        "fully-decoded"
    );
}

#[test]
fn maps_unrecognized_status_to_error_sentinel() {
    assert_eq!(map_recovery_status("something_new"), super::UNKNOWN);
}

#[test]
fn scrooby_child_kinds_are_controlled() -> Result<(), String> {
    let Some(kinds) = super::controlled_values("kind") else {
        return Err("kind vocabulary should remain controlled".to_owned());
    };
    assert!(kinds.contains(&"p3d-scrooby-layout"));
    assert!(kinds.contains(&"p3d-scrooby-resource"));
    assert!(super::TAXONOMY_JSON.contains("p3d-scrooby-layout"));
    assert!(super::TAXONOMY_JSON.contains("p3d-scrooby-resource"));
    Ok(())
}
