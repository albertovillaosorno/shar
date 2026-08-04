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
//   - Media dependency path unit tests.
// - Must-Not:
//   - Own production behavior or perform network access.
// - Allows:
//   - Pure assertions over repository-local dependency paths.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Media dependency path unit tests.
// - Description:
//   - Prevents portable media tools from escaping the hidden dependency root.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Path regressions fail explicitly.
//

//! Media dependency path unit tests.

use std::path::Path;

use super::{REPO_FFMPEG_DIR, repo_ffmpeg_bin_dir};

#[test]
fn repository_ffmpeg_uses_hidden_dependency_root() {
    assert_eq!(REPO_FFMPEG_DIR, ".dependencies/ffmpeg");
    assert_eq!(
        repo_ffmpeg_bin_dir(),
        Path::new(".dependencies").join("ffmpeg").join("bin")
    );
    assert!(!Path::new("dependencies").exists());
}
