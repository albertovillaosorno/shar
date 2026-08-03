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

use super::{movie_scope, music_bank, music_role, sound_effect_scope};

#[test]
fn classifies_music_banks_and_roles() {
    assert_eq!(music_bank(&["extracted", "music02"]), "bank-02");
    assert_eq!(music_role(&["music", "generic"]), "runtime-base");
    assert_eq!(music_role(&["music", "halloween"]), "holiday-halloween");
}

#[test]
fn classifies_sound_effect_scopes() {
    assert_eq!(
        sound_effect_scope(&[
            "extracted",
            "soundfx",
            "interactive",
            "props",
            "spanish"
        ]),
        "effects/interactive-props/spanish".to_owned()
    );
    assert_eq!(
        sound_effect_scope(&["extracted", "carsound", "common"]),
        "vehicle-audio/runtime-base".to_owned()
    );
}

#[test]
fn classifies_movie_scopes() {
    assert_eq!(
        movie_scope(&["extracted", "movies", "fmv4"]),
        Some("story/fmv4".to_owned())
    );
    assert_eq!(
        movie_scope(&["extracted", "movies", "radlogo"]),
        Some("logos/radlogo".to_owned())
    );
    assert_eq!(
        movie_scope(&["extracted", "lmlm", "movies", "intro"]),
        Some("mod-audio/intro".to_owned())
    );
}
