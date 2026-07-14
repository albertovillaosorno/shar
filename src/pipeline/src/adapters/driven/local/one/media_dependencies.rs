// File:
//   - media_dependencies.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/one/media_dependencies.rs
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
//   - The media dependencies contract for pipeline phase one.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute media dependencies.
// - Split-When:
//   - Split when media dependencies contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Keeps portable media tools outside Git but inside the repository tree.
// - Description:
//   - Defines media dependencies data and behavior for pipeline phase one.
// - Usage:
//   - Used by pipeline phase one code that needs media dependencies.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Keeps portable media tools outside Git but inside the
//   - repository tree keeps tightly coupled validation, ordering, and
//   - deterministic transformation invariants together; split when a stable
//   - independently testable sub-boundary is identified.
//

//! Keeps portable media tools outside Git but inside the repository tree.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Keeps portable media tools outside Git but inside the repository tree.
const REPO_FFMPEG_DIR: &str = "dependencies/ffmpeg";
/// Downloads the full Windows build because the essentials build omits HAP.
const FFMPEG_FULL_BUILD_URL: &str =
    "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-full.7z";
/// Bootstraps 7z extraction without requiring a system-wide install.
const SEVEN_ZIP_REDUCED_URL: &str = "https://www.7-zip.org/a/7zr.exe";

/// Resolves ffmpeg tools from override, local dependencies, or PATH.
pub(super) fn media_tool_path(name: &str) -> PathBuf {
    if let Ok(dir) = std::env::var("SHAR_UNREAL_FFMPEG_DIR") {
        let local = PathBuf::from(dir).join(media_tool_file_name(name));
        if local.exists() {
            return local;
        }
    }
    let dependency_tool =
        repo_ffmpeg_bin_dir().join(media_tool_file_name(name));
    if dependency_tool.exists() {
        return dependency_tool;
    }
    PathBuf::from(name)
}

/// Ensures movie export has ffmpeg, ffprobe, and the HAP encoder.
pub(super) fn ensure_ffmpeg_dependency() -> Result<(), String> {
    if std::env::var_os("SHAR_UNREAL_FFMPEG_DIR").is_none() {
        ensure_repo_ffmpeg_dependency()?;
    }
    for executable in [
        "ffmpeg", "ffprobe",
    ] {
        let output = Command::new(media_tool_path(executable))
            .arg("-version")
            .output()
            .map_err(
                |error| {
                    format!(
                        "movie export requires {executable} in PATH or \
                         dependencies/ffmpeg/bin: {error}"
                    )
                },
            )?;
        if !output
            .status
            .success()
        {
            return Err(
                format!(
                    "movie export requires a working {executable} executable"
                ),
            );
        }
    }
    if !media_tool_has_hap_encoder(&media_tool_path("ffmpeg"))? {
        return Err(
            concat!(
                "movie export requires an ffmpeg build with the HAP encoder; ",
                "the pipeline installs one under dependencies/ffmpeg unless ",
                "SHAR_UNREAL_FFMPEG_DIR overrides the tool directory"
            )
            .to_owned(),
        );
    }
    Ok(())
}

/// Installs local ffmpeg only when the cached copy is missing or unsuitable.
fn ensure_repo_ffmpeg_dependency() -> Result<(), String> {
    let ffmpeg = repo_ffmpeg_bin_dir().join(media_tool_file_name("ffmpeg"));
    let ffprobe = repo_ffmpeg_bin_dir().join(media_tool_file_name("ffprobe"));
    if ffmpeg.is_file()
        && ffprobe.is_file()
        && media_tool_has_hap_encoder(&ffmpeg)?
    {
        return Ok(());
    }
    if !cfg!(windows) {
        return Err(
            concat!(
                "automatic ffmpeg dependency install currently supports ",
                "Windows portable tools; set SHAR_UNREAL_FFMPEG_DIR on ",
                "this platform"
            )
            .to_owned(),
        );
    }
    install_repo_ffmpeg_dependency()
}

/// Downloads and extracts portable ffmpeg into the ignored dependency cache.
fn install_repo_ffmpeg_dependency() -> Result<(), String> {
    let root = PathBuf::from(REPO_FFMPEG_DIR);
    let download_dir = root.join("download");
    let staging_dir = root.join("staging");
    let bootstrap_dir = root.join("bootstrap");
    let bin_dir = repo_ffmpeg_bin_dir();
    fs::create_dir_all(&download_dir).map_err(path_error(&download_dir))?;
    fs::create_dir_all(&bootstrap_dir).map_err(path_error(&bootstrap_dir))?;
    fs::create_dir_all(&bin_dir).map_err(path_error(&bin_dir))?;

    let seven_zip = bootstrap_dir.join("7zr.exe");
    download_file(
        SEVEN_ZIP_REDUCED_URL,
        &seven_zip,
        "7-Zip extractor",
    )?;
    let archive = download_dir.join("ffmpeg-release-full.7z");
    download_file(
        FFMPEG_FULL_BUILD_URL,
        &archive,
        "ffmpeg full build",
    )?;

    if staging_dir.exists() {
        fs::remove_dir_all(&staging_dir).map_err(path_error(&staging_dir))?;
    }
    fs::create_dir_all(&staging_dir).map_err(path_error(&staging_dir))?;
    let output_dir = format!(
        "-o{}",
        staging_dir.display()
    );
    let status = Command::new(&seven_zip)
        .arg("x")
        .arg("-y")
        .arg(output_dir)
        .arg(&archive)
        .status()
        .map_err(
            |error| {
                format!("failed to run repository-local 7zr extractor: {error}")
            },
        )?;
    if !status.success() {
        return Err(
            "failed to extract repository-local ffmpeg dependency".to_owned(),
        );
    }

    copy_dependency_tool(
        &staging_dir,
        &bin_dir,
        &media_tool_file_name("ffmpeg"),
    )?;
    copy_dependency_tool(
        &staging_dir,
        &bin_dir,
        &media_tool_file_name("ffprobe"),
    )?;
    let installed_ffmpeg = bin_dir.join(media_tool_file_name("ffmpeg"));
    if !media_tool_has_hap_encoder(&installed_ffmpeg)? {
        return Err(
            "installed ffmpeg dependency does not report the hap encoder"
                .to_owned(),
        );
    }
    Ok(())
}

/// Downloads one installer artifact through the operator machine curl binary.
fn download_file(
    url: &str,
    target: &Path,
    label: &str,
) -> Result<(), String> {
    let parent = target
        .parent()
        .ok_or_else(|| format!("{label} has no parent directory"))?;
    fs::create_dir_all(parent).map_err(path_error(parent))?;
    let partial = target.with_extension("part");
    if partial.exists() {
        fs::remove_file(&partial).map_err(path_error(&partial))?;
    }
    let status = Command::new("curl")
        .arg("-L")
        .arg("--fail")
        .arg("--retry")
        .arg("2")
        .arg("-o")
        .arg(&partial)
        .arg(url)
        .status()
        .map_err(|error| format!("curl failed for {label}: {error}"))?;
    if !status.success() {
        return Err(format!("failed to download {label} from {url}"));
    }
    if target.exists() {
        fs::remove_file(target).map_err(path_error(target))?;
    }
    fs::rename(
        &partial, target,
    )
    .map_err(path_error(target))?;
    Ok(())
}

/// Copies the extracted runtime tool into the stable dependency bin path.
fn copy_dependency_tool(
    staging_dir: &Path,
    bin_dir: &Path,
    file_name: &str,
) -> Result<(), String> {
    let source = find_file_named(
        staging_dir,
        file_name,
    )?;
    let target = bin_dir.join(file_name);
    let _bytes = fs::copy(
        &source, &target,
    )
    .map_err(path_error(&target))?;
    Ok(())
}

/// Locates tools inside versioned archive roots whose names change over time.
fn find_file_named(
    root: &Path,
    file_name: &str,
) -> Result<PathBuf, String> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(current) = stack.pop() {
        let entries = fs::read_dir(&current)
            .map_err(path_error(&current))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(path_error(&current))?;
        for entry in entries {
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(path_error(&path))?;
            if file_type.is_dir() {
                stack.push(path);
            } else if file_type.is_file()
                && path
                    .file_name()
                    .and_then(|value| value.to_str())
                    == Some(file_name)
            {
                return Ok(path);
            }
        }
    }
    Err(format!("dependency archive did not contain {file_name}"))
}

/// Checks the exact encoder needed before any movie package is written.
fn media_tool_has_hap_encoder(ffmpeg: &Path) -> Result<bool, String> {
    if !ffmpeg.exists() {
        return Ok(false);
    }
    let output = Command::new(ffmpeg)
        .arg("-hide_banner")
        .arg("-encoders")
        .output()
        .map_err(
            |error| {
                format!(
                    "ffmpeg encoder check failed for {}: {error}",
                    ffmpeg.display()
                )
            },
        )?;
    if !output
        .status
        .success()
    {
        return Ok(false);
    }
    let encoders = String::from_utf8_lossy(&output.stdout);
    Ok(
        encoders
            .lines()
            .any(
                |line| {
                    line.split_whitespace()
                        .any(|token| token == "hap")
                },
            ),
    )
}

/// Adds the Windows executable suffix while keeping override names portable.
fn media_tool_file_name(name: &str) -> String {
    if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_owned()
    }
}

/// Returns the canonical ignored bin directory used by movie export.
fn repo_ffmpeg_bin_dir() -> PathBuf {
    PathBuf::from(REPO_FFMPEG_DIR).join("bin")
}

/// Captures the path in filesystem errors so dependency failures are
/// actionable.
fn path_error(path: &Path) -> impl FnOnce(std::io::Error) -> String + '_ {
    move |error| {
        format!(
            "{}: {error}",
            path.display()
        )
    }
}
