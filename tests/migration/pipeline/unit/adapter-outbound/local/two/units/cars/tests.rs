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

use super::vehicle_family;

#[test]
fn classifies_character_vehicle_rigs() {
    assert_eq!(vehicle_family("apu-v"), "character-rigs");
    assert_eq!(vehicle_family("homer-v"), "character-rigs");
}

#[test]
fn classifies_vehicle_support_families() {
    assert_eq!(vehicle_family("common"), "runtime-base");
    assert_eq!(vehicle_family("ambul"), "service-vehicles");
    assert_eq!(vehicle_family("sedana"), "traffic-variants");
    assert_eq!(vehicle_family("ccola"), "commercial-vehicles");
    assert_eq!(vehicle_family("tt"), "special-vehicles");
}
