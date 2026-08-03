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
//   - Uv conversion tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Uv conversion tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Uv conversion tests unit tests.

use super::{ModelUvPolicy, source_uv_to_fbx};

/// Assert exact deterministic UV components without float comparison.
fn assert_uv_bits(actual: [f64; 2], expected: [f64; 2]) {
    assert_eq!(actual.map(f64::to_bits), expected.map(f64::to_bits));
}

#[test]
fn preserves_source_u_without_decal_evidence() {
    assert_uv_bits(source_uv_to_fbx([0.25_f32, 0.75_f32], false), [
        0.25_f64, 0.75_f64,
    ]);
}

#[test]
fn mirrors_only_selected_decal_u_without_changing_v() {
    assert_uv_bits(source_uv_to_fbx([0.25_f32, 0.75_f32], true), [
        0.75_f64, 0.75_f64,
    ]);
    assert_uv_bits(source_uv_to_fbx([2f32, -1f32], true), [-1f64, -1f64]);
}

#[test]
fn preserve_policy_disables_even_evidence_backed_graphic_mirroring() {
    assert!(ModelUvPolicy::Selective.mirrors_u(
        "kwik-e-mart-sign",
        "kwik-e-mart-sign_m",
        Some("kwik-e-mart-sign.png"),
    ));
    assert!(!ModelUvPolicy::Preserve.mirrors_u(
        "kwik-e-mart-sign",
        "kwik-e-mart-sign_m",
        Some("kwik-e-mart-sign.png"),
    ));
}
