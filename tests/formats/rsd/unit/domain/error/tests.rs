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

use std::path::PathBuf;

use super::RsdError;

#[test]
fn io_error_escapes_source_control_characters() {
    let error = RsdError::io(
        PathBuf::from("audio.rsd"),
        "read
injected"
            .to_owned(),
    );

    let rendered = error.to_string();

    assert!(
        !rendered.chars().any(char::is_control),
        "diagnostic contains a control character: {rendered:?}"
    );
    assert!(rendered.contains(r"read\ninjected"));
}

#[test]
fn source_audio_error_keeps_one_escape_layer() {
    let inner = RsdError::io(
        PathBuf::from("inner.rsd"),
        "read
injected"
            .to_owned(),
    );
    let error = RsdError::SourceAudio {
        path: PathBuf::from("outer.rsd"),
        source: Box::new(inner),
    };

    let rendered = error.to_string();

    assert!(
        !rendered.chars().any(char::is_control),
        "diagnostic contains a control character: {rendered:?}"
    );
    assert!(rendered.contains(r"read\ninjected"));
    assert!(!rendered.contains(r"read\\ninjected"));
}
