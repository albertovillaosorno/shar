// File:
//   - path_helpers.rs
// Path:
//   - src/game-manifest/tests/path_helpers.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Pure path-token and obfuscation regressions for game-manifest.
// - Must-Not:
//   - Depend on licensed inputs or mutable repository outputs.
// - Allows:
//   - Platform-independent Path fixtures and deterministic string assertions.
// - Split-When:
//   - Split when filesystem traversal fixtures become necessary.
// - Merge-When:
//   - Another game-manifest test owns the same pure path helper boundary.
// - Summary:
//   - Protects deterministic manifest path helpers.
// - Description:
//   - Verifies extension and obfuscation helpers preserve canonical tokens.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Fixtures contain no proprietary names or local asset routes.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Deterministic path-helper regression coverage.
//!
//! These tests exercise pure helpers without reading private or generated
//! repository content.

use std::path::Path;

use game_manifest::{NO_EXTENSION, extension_of, obfuscate_component};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn extension_of_treats_trailing_dot_as_missing() {
    assert_eq!(
        extension_of(Path::new("asset.")),
        NO_EXTENSION
    );
}

#[test]
fn extension_of_lowercases_unicode() {
    assert_eq!(
        // cspell:disable-next-line -- ÄBC
        extension_of(Path::new("asset.ÄBC")),
        // cspell:disable-next-line -- äbc
        "äbc"
    );
}

#[test]
fn obfuscate_component_lowercases_unicode() {
    assert_eq!(
        obfuscate_component("ÄZ"),
        "äz"
    );
}
