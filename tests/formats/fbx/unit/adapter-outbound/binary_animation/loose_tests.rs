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
//   - Binary animation loose unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Binary animation loose unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Assertions fail explicitly.
//

//! Binary animation loose unit tests.

use super::*;

#[test]
fn build_animation_plan_rejects_duplicate_clip_names() {
    let clips = [
        AnimationClip {
            name: "walk".to_owned(),
            source_identity: None,
            frame_rate: 30f64,
            cyclic: false,
            frame_count: 1,
            tracks: Vec::new(),
            ignored_group_ids: Vec::new(),
        },
        AnimationClip {
            name: "walk".to_owned(),
            source_identity: None,
            frame_rate: 30f64,
            cyclic: false,
            frame_count: 1,
            tracks: Vec::new(),
            ignored_group_ids: Vec::new(),
        },
    ];

    assert!(
        matches!(
            build_animation_plan(
                &clips,
                &BTreeMap::new(),
            ),
            Err(BinaryAnimationError::DuplicateClipName(name))
                if name == "walk"
        ),
        "duplicate logical clip names must not emit duplicate stacks or takes"
    );
}

#[test]
fn shared_frame_rate_rejects_distinct_exact_values() {
    let clips = [
        AnimationClip {
            name: "first".to_owned(),
            source_identity: None,
            frame_rate: 30f64,
            cyclic: false,
            frame_count: 1,
            tracks: Vec::new(),
            ignored_group_ids: Vec::new(),
        },
        AnimationClip {
            name: "second".to_owned(),
            source_identity: None,
            frame_rate: 30.000_000_000_5_f64,
            cyclic: false,
            frame_count: 1,
            tracks: Vec::new(),
            ignored_group_ids: Vec::new(),
        },
    ];

    assert_eq!(
        shared_frame_rate(&clips),
        Err(BinaryAnimationError::MixedFrameRate)
    );
}
