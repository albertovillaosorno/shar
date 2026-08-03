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

use super::compute_id;

#[test]
fn id_is_deterministic_and_type_prefixed() {
    let first = compute_id("ed/at/#1.json", "model", "fully-decoded");
    let second = compute_id("ed/at/#1.json", "model", "fully-decoded");
    assert_eq!(first, second);
    assert!(first.starts_with("model-"));
}

#[test]
fn id_changes_with_route_and_recovery() {
    let base = compute_id("ed/at/#1.json", "model", "fully-decoded");
    let other_route = compute_id("ed/at/#2.json", "model", "fully-decoded");
    let other_type = compute_id("ed/at/#1.json", "image", "fully-decoded");
    assert_ne!(base, other_route);
    assert_ne!(base, other_type);
}

#[test]
fn id_values_are_pinned_across_machines_and_releases() {
    for (route, unit_type, status, expected) in [
        (
            "ed/#1.md",
            "text",
            "fully-decoded",
            "text-1bb89d0a-0b6b-26bc-9356-9ba8d43d1c54",
        ),
        (
            "ed/ae/sd/ae/#1.wav",
            "audio",
            "fully-decoded",
            "audio-d43c57f7-259e-1172-0559-dd6548e4bf9e",
        ),
        (
            "ed/at/b7/cs/mh/#1.json",
            "model",
            "fully-decoded",
            "model-6558a19c-512f-46ce-1b3f-0b904343b3d6",
        ),
        (
            "ed/at/b7/cs/sg/#1.json",
            "world",
            "fully-decoded",
            "world-c9e85a29-c47c-569e-8555-f7981aeab284",
        ),
    ] {
        assert_eq!(compute_id(route, unit_type, status,), expected);
    }
}

#[test]
fn id_suffix_is_uuid_shaped() {
    let id = compute_id("ed/at/#1.json", "image", "fully-decoded");
    let suffix = id.strip_prefix("image-");
    assert_eq!(suffix.map(str::len), Some(36));
    assert_eq!(suffix.map(|value| value.matches('-').count()), Some(4));
}
