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

use super::{
    GENERAL_CHARACTER_ANIMATION_SUBCATEGORY, animation_subcategory_candidates,
    deferred_material_identity, normalized_texture_png_file_name,
};

#[test]
fn deferred_material_preserves_decoded_shader_identity() {
    assert_eq!(
        deferred_material_identity(
            "char_swatches_lit_m_",
            "char_swatches_lit_m",
        ),
        "char_swatches_lit_m"
    );
}

#[test]
fn normalizes_trailing_nul_padded_texture_reference() {
    let result = normalized_texture_png_file_name(
        "char_swatches_lit.bmp\u{0}\u{0}\u{0}",
    );

    assert!(
        result.is_ok(),
        "fixed-width texture padding should normalize: {result:?}"
    );
    assert_eq!(result.ok().as_deref(), Some("char_swatches_lit.png"));
}

#[test]
fn character_animation_candidates_prefer_identity_specific_banks() {
    assert_eq!(
        animation_subcategory_candidates("characters/apu/base-model"),
        vec![
            "characters/apu/animation-set".to_owned(),
            GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
        ]
    );
    assert_eq!(
        animation_subcategory_candidates("characters/lisa/costume/cool"),
        vec![
            "characters/lisa/animation-set".to_owned(),
            GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
        ]
    );
}

#[test]
fn character_animation_candidates_use_general_bank_for_other_models() {
    assert_eq!(
        animation_subcategory_candidates("characters/krusty/base-model"),
        vec![
            "characters/krusty/animation-set".to_owned(),
            GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
        ]
    );
    assert_eq!(
        animation_subcategory_candidates("characters/boy1/crowd-model"),
        vec![GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned()]
    );
    assert_eq!(
        animation_subcategory_candidates("characters/homer/base-model"),
        vec![GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned()]
    );
}
