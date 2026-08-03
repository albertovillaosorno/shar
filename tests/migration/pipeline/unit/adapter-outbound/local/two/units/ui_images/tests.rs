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

use super::subcategory_from_tail;

#[test]
fn classifies_image_metadata_roots() {
    assert_eq!(
        super::image_metadata_subcategory(&[]),
        "ui-images/source-metadata/root".to_owned()
    );
    assert_eq!(
        super::image_metadata_subcategory(&["cars2d"]),
        "ui-images/vehicle-icons/source-metadata".to_owned()
    );
    assert_eq!(
        super::image_metadata_subcategory(&["skins2d"]),
        "ui-images/character-skins/source-metadata".to_owned()
    );
}

#[test]
fn classifies_mission_icon_families() {
    assert_eq!(
        subcategory_from_tail(&["msnicons", "object", "cola"]),
        Some("ui-images/mission-icons/objects/cola".to_owned())
    );
}

#[test]
fn classifies_exact_frontend_image_details() {
    assert_eq!(
        subcategory_from_tail(&["mis01", "08"]),
        Some("ui-images/mission-briefing/level-01/mis01-08".to_owned())
    );
    assert_eq!(
        subcategory_from_tail(&["scrapbook", "mis03", "07"]),
        Some("ui-images/scrapbook/level-03/mis03-07".to_owned())
    );
    assert_eq!(
        subcategory_from_tail(&["license", "spanish", "licensepc"]),
        Some("ui-images/licenses/spanish/licensepc".to_owned())
    );
    assert_eq!(
        subcategory_from_tail(&["skins2d", "b", "ninja"]),
        Some("ui-images/character-skins/b/ninja".to_owned())
    );
}

#[test]
fn classifies_vehicle_icon_state() {
    assert_eq!(
        subcategory_from_tail(&["cars2d", "apu", "vd"]),
        Some("ui-images/vehicle-icons/damaged/apu".to_owned())
    );
    assert_eq!(
        subcategory_from_tail(&["cars2d", "apu", "v"]),
        Some("ui-images/vehicle-icons/normal/apu".to_owned())
    );
}

#[test]
fn classifies_localized_loading_screens() {
    assert_eq!(
        subcategory_from_tail(&["loading", "german", "loading4"]),
        Some("ui-images/loading/german/level-04".to_owned())
    );
}
