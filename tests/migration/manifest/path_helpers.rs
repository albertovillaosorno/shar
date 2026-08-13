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
//   - Path helpers test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Path helpers test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Path helpers test module.

use std::path::{Path, PathBuf};

use game_manifest::{
    NO_EXTENSION, exact_file_shortfalls, extension_of, kind_taxonomy_jsonl,
    obfuscate_component,
};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn extension_of_treats_trailing_dot_as_missing() {
    assert_eq!(extension_of(Path::new("asset.")), NO_EXTENSION);
}

#[test]
fn extension_of_lowercases_unicode() {
    assert_eq!(extension_of(Path::new("asset.ÄBC")), "äbc");
}

#[test]
fn obfuscate_component_lowercases_unicode() {
    assert_eq!(obfuscate_component("ÄZ"), "äz");
}

#[test]
fn exact_root_requirements_are_case_sensitive() {
    let root = Path::new("game");
    let files = vec![
        PathBuf::from("game/simpsons.exe"),
        PathBuf::from("game/Simpsons.ico"),
    ];

    assert_eq!(
        exact_file_shortfalls(root, &files),
        vec!["  <root> Simpsons.exe: have 0, need at least 1".to_owned()]
    );
}

#[test]
fn optional_uninstall_icon_is_not_a_shortfall() {
    let root = Path::new("game");
    let files = vec![
        PathBuf::from("game/Simpsons.exe"),
        PathBuf::from("game/Simpsons.ico"),
    ];

    assert!(exact_file_shortfalls(root, &files).is_empty());
    assert!(kind_taxonomy_jsonl().contains(
        "\"required_files\":[{\"path\":\"Simpsons.exe\",\"min\":1},"
    ));
    assert!(kind_taxonomy_jsonl().contains(
        "{\"path\":\"uninst.ico\",\"min\":0}]"
    ));
}
