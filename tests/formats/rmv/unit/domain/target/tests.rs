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

use super::{CinematicTarget, TargetDecision};

#[test]
fn unreal_hap_movie_is_default_without_private_encoder() {
    let decision = TargetDecision::without_official_bink2_encoder();
    assert_eq!(decision.primary_target, CinematicTarget::UnrealHapMovie);
    assert_eq!(
        decision.optional_target,
        Some(CinematicTarget::OfficialBink2)
    );
}

#[test]
fn official_bink2_is_marked_as_private_encoder_dependent() {
    assert!(CinematicTarget::OfficialBink2.requires_private_encoder());
    assert!(!CinematicTarget::UnrealHapMovie.requires_private_encoder());
}
