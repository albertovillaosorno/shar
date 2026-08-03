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

use super::{scene_resource_role, scene_resource_subcategory};

#[test]
fn classifies_frontend_scene_roles() {
    assert_eq!(scene_resource_role(&["camset"]), "camera-sets");
    assert_eq!(scene_resource_role(&["l4hudmap"]), "hud-maps");
    assert_eq!(scene_resource_role(&["gaghomer"]), "gag-scenes");
    assert_eq!(scene_resource_role(&["glowtv"]), "interactive-glows");
    assert_eq!(scene_resource_role(&["curtainl"]), "screen-transitions");
    assert_eq!(scene_resource_role(&["rewardbg"]), "reward-presentation");
}

#[test]
fn classifies_scene_resources_as_frontend_scenes() {
    assert_eq!(
        scene_resource_subcategory(
            &["extracted", "art", "frontend", "scrooby", "resource"],
            &["pure3d", "camset"]
        ),
        Some(
            "ui-resources/frontend-scenes/camera-sets/sprite-layouts/\
             camset"
                .to_owned()
        )
    );
}

#[test]
fn appends_exact_details_for_tokenized_resource_roles() {
    assert_eq!(
        super::ui_resource_detail(&[], &["frontend", "card12"], "cards",),
        "/card12".to_owned()
    );
    assert_eq!(
        super::ui_resource_detail(&[], &["ingame", "qapu"], "speaker-icons",),
        "/qapu".to_owned()
    );
    assert_eq!(
        super::ui_resource_detail(&[], &["backend", "loading0"], "loading",),
        "/loading0".to_owned()
    );
    assert_eq!(
        super::ui_resource_detail(
            &["scrooby2"],
            &["txtbible", "srr2"],
            "art-assets",
        ),
        "/scene-layouts/txtbible-srr2".to_owned()
    );
}
