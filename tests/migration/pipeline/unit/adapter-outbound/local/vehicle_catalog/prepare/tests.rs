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

use fbx::domain::texture::MaterialSemantics;

use super::{
    is_wheel_identity, texture_state_role, vehicle_part_role,
    vehicle_part_semantics,
};

fn role(mesh: &str, shader: &str) -> &'static str {
    let semantics =
        vehicle_part_semantics(mesh, shader, MaterialSemantics::default());
    vehicle_part_role(mesh, shader, semantics)
}

#[test]
fn semantic_roles_keep_moving_and_glass_parts_separate() {
    assert_eq!(role("TrunkRotShape", "trunk_m"), "trunk");
    assert_eq!(role("DoorDRotShape", "door_m"), "driver-door");
    assert_eq!(role("homer_vShape", "WindsheildT_m"), "glass");
    assert_eq!(role("w0Shape", "wheel_m"), "wheel");
}

#[test]
fn wheel_identity_does_not_capture_unrelated_body_names() {
    assert!(is_wheel_identity("wshape3"));
    assert!(is_wheel_identity("w2shape"));
    assert!(!is_wheel_identity("windowshape"));
}

#[test]
fn damage_textures_receive_a_distinct_sidecar_role() {
    assert_eq!(texture_state_role("homer_vDoorDDam.png"), "damage");
    assert_eq!(texture_state_role("homer_vSideFL.png"), "alternates");
}
