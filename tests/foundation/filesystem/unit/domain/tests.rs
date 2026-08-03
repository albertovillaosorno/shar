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

use std::path::{Path, PathBuf};

use super::{RootedPathError, resolve_under};

#[test]
fn nested_relative_path_stays_beneath_root() {
    let resolved =
        resolve_under(Path::new("output"), Path::new("audio/voice.wav"));
    assert_eq!(resolved, Ok(PathBuf::from("output/audio/voice.wav")));
}

#[test]
fn parent_component_is_rejected() {
    let result = resolve_under(Path::new("output"), Path::new("../escape.bin"));
    assert_eq!(result, Err(RootedPathError::ParentTraversal));
}

#[test]
fn absolute_path_is_rejected() {
    let absolute = if cfg!(windows) {
        PathBuf::from(r"C:\escape.bin")
    } else {
        PathBuf::from("/escape.bin")
    };
    let result = resolve_under(Path::new("output"), &absolute);
    assert_eq!(result, Err(RootedPathError::Absolute));
}
