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

use super::apply_overlay;
use crate::adapters::driven::local::two::localization::{
    CustomTextEntry, LanguageDocument, LanguageEntry,
};

#[test]
fn rejects_overlay_targeting_duplicate_base_hash() -> Result<(), String> {
    let base = LanguageDocument {
        id: 'S',
        language: "spanish_spain",
        source_name: "base".to_owned(),
        modulo: 1009,
        entries: vec![
            LanguageEntry {
                hash: 16,
                offset: 0,
                value: "one".to_owned(),
            },
            LanguageEntry {
                hash: 16,
                offset: 2,
                value: "two".to_owned(),
            },
        ],
    };
    let custom = [CustomTextEntry {
        key: "0x10".to_owned(),
        value: "replacement".to_owned(),
        line: 1,
    }];
    if apply_overlay(&base, &custom).is_err() {
        Ok(())
    } else {
        Err("overlay silently replaced duplicate base hashes".to_owned())
    }
}

#[test]
fn preserves_untargeted_duplicate_base_hashes() -> Result<(), String> {
    let base = LanguageDocument {
        id: 'S',
        language: "spanish_spain",
        source_name: "base".to_owned(),
        modulo: 1009,
        entries: vec![
            LanguageEntry {
                hash: 16,
                offset: 0,
                value: "one".to_owned(),
            },
            LanguageEntry {
                hash: 16,
                offset: 2,
                value: "two".to_owned(),
            },
        ],
    };
    let merge = apply_overlay(&base, &[]).map_err(|error| error.to_string())?;
    if merge
        .entries
        .iter()
        .map(|entry| entry.value.as_str())
        .eq(["one", "two"])
    {
        Ok(())
    } else {
        Err(format!(
            "duplicate base values changed: {:?}",
            merge.entries
        ))
    }
}

#[test]
fn rejects_colliding_custom_hashes() -> Result<(), String> {
    let base = LanguageDocument {
        id: 'S',
        language: "spanish_spain",
        source_name: "base".to_owned(),
        modulo: 1,
        entries: vec![LanguageEntry {
            hash: 0,
            offset: 0,
            value: "base".to_owned(),
        }],
    };
    let custom = vec![
        CustomTextEntry {
            key: "A".to_owned(),
            value: "first".to_owned(),
            line: 1,
        },
        CustomTextEntry {
            key: "B".to_owned(),
            value: "second".to_owned(),
            line: 2,
        },
    ];
    if apply_overlay(&base, &custom).is_err() {
        Ok(())
    } else {
        Err("colliding overlay identities were accepted".to_owned())
    }
}
