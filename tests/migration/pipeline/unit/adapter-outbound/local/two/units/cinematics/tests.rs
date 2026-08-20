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

use super::{
    gag_subcategory, named_gag_scene_subcategory, numbered_gag_series,
};

#[test]
fn classifies_explicit_level_gags() {
    assert_eq!(
        gag_subcategory(&["extracted", "art", "nis", "gags", "l04", "azte"]),
        Some("cinematics/gags/level-04/named/azte".to_owned())
    );
    assert_eq!(
        gag_subcategory(&["extracted", "art", "nis", "gags", "l2", "dump"]),
        Some("cinematics/gags/level-02/dump-scenes".to_owned())
    );
}

#[test]
fn classifies_named_gag_scene_codes() {
    assert_eq!(
        named_gag_scene_subcategory(&["gag", "alm2"]),
        Some("cinematics/gags/named/alm2".to_owned())
    );
    assert_eq!(
        named_gag_scene_subcategory(&["gag", "k", "h1"]),
        Some("cinematics/gags/named/k-h1".to_owned())
    );
    assert_eq!(
        gag_subcategory(&["extracted", "art", "nis", "gags", "gag", "bbq"]),
        Some("cinematics/gags/named/bbq".to_owned())
    );
}

#[test]
fn classifies_numbered_gag_series() {
    assert_eq!(numbered_gag_series("gag0207"), Some("02".to_owned()));
    assert_eq!(
        gag_subcategory(&["extracted", "art", "nis", "gags", "gag0207"]),
        Some("cinematics/gags/series-02/numbered/gag0207".to_owned())
    );
}
