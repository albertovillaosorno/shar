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

use super::LmlmError;

#[test]
fn public_diagnostics_are_single_line() {
    let errors = [
        LmlmError::NonZeroReservedContainerBlock { offset: 0x200, value: 1 },
        LmlmError::NonZeroMetadataPadding { offset: 0x800, value: 2 },
        LmlmError::EntryPayloadOverlapsTable {
            path: "entry.bin".to_owned(),
            offset: 0x600,
            table_end: 0xa00,
        },
        LmlmError::InvalidEntryRange {
            path: "entry.bin".to_owned(),
            offset: 0x1000,
            size: 7,
        },
        LmlmError::PathCollision {
            first_path: "Entry.bin".to_owned(),
            second_path: "entry.bin".to_owned(),
        },
    ];

    for error in errors {
        let diagnostic = error.to_string();
        assert!(
            !diagnostic.contains(['\n', '\r']),
            "public diagnostic must remain single-line: {diagnostic:?}"
        );
    }
}

#[test]
fn public_diagnostics_escape_untrusted_control_characters() {
    let errors = [
        LmlmError::UnsafePath("bad\u{1b}[2J.bin".to_owned()),
        LmlmError::InvalidNameEncoding {
            offset: 0x600,
            message: "bad\nencoding".to_owned(),
        },
        LmlmError::PathCollision {
            first_path: "first\rpath".to_owned(),
            second_path: "second\u{7}path".to_owned(),
        },
    ];

    for error in errors {
        let diagnostic = error.to_string();
        assert!(
            diagnostic.chars().all(|character| !character.is_control()),
            "public diagnostic exposed a control character: {diagnostic:?}"
        );
    }
}
