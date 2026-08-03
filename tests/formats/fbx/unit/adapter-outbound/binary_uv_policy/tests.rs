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

use super::mirrors_u;

#[test]
fn mirrors_vehicle_liveries_but_not_plain_paint_wheels_or_effects() {
    assert!(mirrors_u("ambulshape__body", "ambul_m", Some("ambul.png"),));
    assert!(mirrors_u(
        "cColaDoorDShape__driver-door",
        "cColaDoorDNorm_m",
        Some("cColaDoorDNorm.png"),
    ));
    assert!(!mirrors_u(
        "snake-vshape__body",
        "snake_vPaint_m",
        Some("snake_vPaint.png"),
    ));
    assert!(!mirrors_u(
        "wshape__wheel",
        "cCola_Wheel_m",
        Some("cCola_Wheel.png"),
    ));
    assert!(!mirrors_u(
        "honor-vshape__body",
        "honor_vWheel_m",
        Some("honor_vWheel.png"),
    ));
    assert!(!mirrors_u(
        "backfireflashgroupshape__vfx",
        "brakeFlareA_m",
        Some("brakeFlareA.png"),
    ));
}

#[test]
fn mirrors_named_world_and_prop_graphics_only() {
    assert!(mirrors_u(
        "kwik-e-mart-sign",
        "store_sign_m",
        Some("store_sign.png"),
    ));
    assert!(mirrors_u(
        "phone-screen",
        "phone_icon_m",
        Some("phone_icon.png"),
    ));
    assert!(!mirrors_u("terrain-patch", "grass_m", Some("grass.png"),));
    assert!(!mirrors_u(
        "frink_h_merged_3__glass",
        "eyeglass3_m",
        Some("eyeglass3.png"),
    ));
}
