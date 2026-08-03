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

use super::is_publishable_composite;
use crate::adapters::driven::local::prop_catalog::model::PropFamily;

#[test]
fn cards_publish_phone_icon_model_evidence() {
    assert!(is_publishable_composite(PropFamily::Cards, "card_idle"));
    assert!(is_publishable_composite(PropFamily::Cards, "phone_icon"));
    assert!(is_publishable_composite(PropFamily::Missions, "phone_icon"));
}
