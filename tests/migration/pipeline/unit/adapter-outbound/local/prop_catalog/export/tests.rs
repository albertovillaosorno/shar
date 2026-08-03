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

use super::{asset_name, asset_relative_dir};
use crate::adapters::driven::local::prop_catalog::model::PropFamily;

#[test]
fn prop_families_publish_below_stable_directories() {
    assert_eq!(
        asset_relative_dir(PropFamily::Cards, "card-idle",),
        "cards/card-idle"
    );
    assert_eq!(
        asset_relative_dir(PropFamily::Cards, "phone-icon",),
        "phone-icon"
    );
    assert_eq!(
        asset_relative_dir(PropFamily::Missions, "bombbarrel",),
        "missions/bombbarrel"
    );
}

#[test]
fn asset_name_is_readable_without_a_hash_suffix() {
    assert_eq!(asset_name("Finish Line!"), Ok("finish-line".to_owned()));
}

#[test]
fn asset_name_rejects_empty_portable_identity() {
    assert!(asset_name("___").is_err());
}
