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

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    REPO_FFMPEG_DIR, dependency_io_error, dependency_path_error,
    is_regular_media_tool, repo_ffmpeg_bin_dir,
};


static CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn case_dir(label: &str) -> Result<PathBuf, String> {
    let root = std::env::temp_dir().join(format!(
        "shar-media-dependency-{label}-{}-{}",
        std::process::id(),
        CASE_ID.fetch_add(1, Ordering::Relaxed),
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(root)
}

#[test]
fn repository_ffmpeg_uses_hidden_dependency_root() {
    assert_eq!(REPO_FFMPEG_DIR, ".dependencies/ffmpeg");
    assert_eq!(
        repo_ffmpeg_bin_dir(),
        Path::new(".dependencies").join("ffmpeg").join("bin")
    );
    assert!(!Path::new("dependencies").exists());
}

#[test]
fn dependency_diagnostics_hide_physical_paths() -> Result<(), String> {
    let private_fragment = "private-workstation/media/cache/tool.exe";
    let error = std::io::Error::other(private_fragment);
    let direct = dependency_io_error("run media tool", &error);
    let adapted = dependency_path_error("publish media tool")(error);
    for rendered in [direct, adapted] {
        if rendered.contains(private_fragment) || !rendered.contains("Other") {
            return Err(format!(
                "dependency diagnostic was not public-safe: {rendered}"
            ));
        }
    }
    Ok(())
}

#[test]
fn media_tool_candidates_require_direct_regular_files() -> Result<(), String> {
    let root = case_dir("regular-files")?;
    let regular = root.join("ffmpeg.exe");
    let directory = root.join("ffprobe.exe");
    fs::write(&regular, b"tool").map_err(|error| error.to_string())?;
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    if !is_regular_media_tool(&regular) {
        return Err("regular media tool was rejected".to_owned());
    }
    if is_regular_media_tool(&directory) {
        return Err("media tool directory was accepted".to_owned());
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::symlink_file;

        let link = root.join("linked-ffmpeg.exe");
        symlink_file(&regular, &link).map_err(|error| error.to_string())?;
        if is_regular_media_tool(&link) {
            return Err("media tool symlink was accepted".to_owned());
        }
    }
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(())
}
