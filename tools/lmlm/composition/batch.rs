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
//   - Folder-driven LMLM WIP and export publication coordination.
// - Must-Not:
//   - Modify import packages or silently trust stale export metadata.
// - Allows:
//   - Coordinate local WIP reuse and review-only package publication.
// - Split-When:
//   - WIP and export lifecycles require independent services.
// - Merge-When:
//   - Another composition module owns the same batch workflow.
// - Summary:
//   - LMLM batch conversion composition.
// - Description:
//   - Coordinates read-only imports, editable WIP, and verified exports.
// - Usage:
//   - Used by the default compatibility-tool batch command.
// - Defaults:
//   - Redirects, collisions, and stale exports fail closed.
//

//! Folder-driven legacy conversion outside the SHAR product pipeline.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;
use serde::Serialize;

use crate::convert::{ConvertError, convert, inspect, package_manifest};

/// Stable report schema for one import-folder run.
pub const BATCH_SCHEMA: &str = "shar.lmlm-folder-conversion.v1";

/// One converted import record.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BatchItem {
    /// Original import filename only, never an absolute machine path.
    pub input: String,
    /// SHA-256 identity of the complete legacy package.
    pub source_sha256: String,
    /// Repository-relative persistent WIP directory.
    pub wip: String,
    /// Whether the WIP directory already existed and was preserved.
    pub wip_reused: bool,
    /// Repository-relative user-facing export directory.
    pub export: String,
    /// Whether the export already existed and was preserved.
    pub export_reused: bool,
}

/// Complete deterministic folder-run report.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BatchReport {
    /// Stable schema identifier.
    pub schema: &'static str,
    /// Sorted converted package records.
    pub packages: Vec<BatchItem>,
}

/// Failure from folder-driven conversion.
#[derive(Debug)]
pub enum BatchError {
    /// Caller/runtime folder contract was invalid.
    Contract(String),
    /// Local filesystem operation failed.
    Io(io::Error),
    /// One legacy package failed validation or conversion.
    Convert(ConvertError),
}

impl fmt::Display for BatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract(message) => formatter.write_str(message),
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Convert(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for BatchError {}

impl From<io::Error> for BatchError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ConvertError> for BatchError {
    fn from(error: ConvertError) -> Self {
        Self::Convert(error)
    }
}

fn direct_lmlm_files(import_root: &Path) -> Result<Vec<PathBuf>, BatchError> {
    if local::path_kind(import_root)? != PathKind::Directory {
        return Err(BatchError::Contract(
            "tools/lmlm/import must be a real directory".to_owned(),
        ));
    }
    let mut files = local::strict_regular_files(import_root)?
        .into_iter()
        .filter(|path| {
            path.parent() == Some(import_root)
                && path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| {
                        extension.eq_ignore_ascii_case("lmlm")
                    })
        })
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}

fn safe_export_stem(input: &Path) -> String {
    let stem = input
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("mod");
    let mut output = String::new();
    for character in stem.chars().take(64) {
        if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
            output.push(character);
        } else {
            output.push('_');
        }
    }
    if output.is_empty() {
        "mod".to_owned()
    } else {
        output
    }
}

fn relative_text(root: &Path, path: &Path) -> Result<String, BatchError> {
    let relative = path.strip_prefix(root).map_err(|_error| {
        BatchError::Contract(
            "conversion path escaped repository root".to_owned(),
        )
    })?;
    Ok(relative
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/"))
}

fn remove_staging(path: &Path) {
    if let Ok(metadata) = fs::symlink_metadata(path)
        && metadata.is_dir()
        && !metadata.file_type().is_symlink()
    {
        let _removed = fs::remove_dir_all(path);
    }
}

fn copy_workspace(
    source: &Path,
    destination: &Path,
    source_sha256: &str,
) -> Result<(), BatchError> {
    if local::path_kind(destination)? != PathKind::Missing {
        return Err(BatchError::Contract(
            "export destination unexpectedly exists".to_owned(),
        ));
    }
    let name = destination
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            BatchError::Contract("export has no portable name".to_owned())
        })?;
    let staging = destination.with_file_name(format!(
        ".{name}.lmlm-export-{}.tmp",
        std::process::id(),
    ));
    if local::path_kind(&staging)? != PathKind::Missing {
        return Err(BatchError::Contract(
            "export staging directory already exists".to_owned(),
        ));
    }
    if let Some(parent) = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
    {
        local::create_dir_all(parent)?;
    }
    local::create_dir_all(&staging)?;
    let copied = (|| -> Result<(), BatchError> {
        for source_file in local::strict_regular_files(source)? {
            let relative = source_file.strip_prefix(source).map_err(|_error| {
                BatchError::Contract("WIP member escaped its root".to_owned())
            })?;
            if relative == Path::new("mod.json") {
                continue;
            }
            let target = schoenwald_filesystem::resolve_under(
                &staging,
                relative,
            )
            .map_err(|error| BatchError::Contract(error.to_string()))?;
            if let Some(parent) = target
                .parent()
                .filter(|path| !path.as_os_str().is_empty())
            {
                local::create_dir_all(parent)?;
            }
            let _copied = fs::copy(&source_file, &target)?;
        }
        let package = package_manifest(&staging, source_sha256)?;
        local::write_text(
            &staging.join("mod.json"),
            &package.to_pretty_json().map_err(ConvertError::from)?,
            false,
        )?;
        fs::rename(&staging, destination)?;
        Ok(())
    })();
    if copied.is_err() {
        remove_staging(&staging);
    }
    copied
}

fn verify_workspace_report(
    workspace: &Path,
    report: &crate::report::ConversionReport,
    label: &str,
) -> Result<(), BatchError> {
    let path = workspace.join("conversion-report.json");
    let stored = local::read_utf8(&path)?;
    let mut expected = serde_json::to_string_pretty(report).map_err(|error| {
        BatchError::Contract(format!(
            "render {label} report failed: {error}"
        ))
    })?;
    expected.push(char::from(10));
    if stored != expected {
        return Err(BatchError::Contract(format!(
            "{label} report does not match the current import"
        )));
    }
    Ok(())
}

fn verify_workspace_package(
    workspace: &Path,
    report: &crate::report::ConversionReport,
    label: &str,
) -> Result<(), BatchError> {
    let stored = local::read_utf8(&workspace.join("mod.json"))?;
    let observed = shar_mod_package::PackageManifest::from_json(&stored)
        .map_err(ConvertError::from)?;
    let expected = package_manifest(workspace, &report.source_sha256)?;
    if observed != expected {
        return Err(BatchError::Contract(format!(
            "{label} mod.json does not match the current workspace"
        )));
    }
    Ok(())
}

fn ensure_directory_or_missing(
    path: &Path,
    label: &str,
) -> Result<bool, BatchError> {
    match local::path_kind(path)? {
        PathKind::Missing => Ok(false),
        PathKind::Directory => Ok(true),
        PathKind::File | PathKind::Other => Err(BatchError::Contract(format!(
            "{label} must be a real directory"
        ))),
    }
}

/// Converts explicit import/export/WIP roots without touching the product
/// pipeline.
///
/// Existing WIP and export directories are preserved. Delete an export manually
/// if you intentionally want to republish an edited persistent WIP workspace.
///
/// # Errors
///
/// Returns a deterministic failure for invalid folders, unsafe filesystem
/// state,
/// unsupported archives, or failed conversion/publication.
pub fn convert_folders(
    repository_root: &Path,
    import_root: &Path,
    export_root: &Path,
    wip_root: &Path,
) -> Result<BatchReport, BatchError> {
    if local::path_kind(export_root)? == PathKind::Missing {
        local::create_dir_all(export_root)?;
    }
    if local::path_kind(wip_root)? == PathKind::Missing {
        local::create_dir_all(wip_root)?;
    }
    let mut packages = Vec::new();
    for input in direct_lmlm_files(import_root)? {
        let report = inspect(&input)?;
        let wip = wip_root.join(&report.source_sha256);
        let wip_reused = ensure_directory_or_missing(&wip, "LMLM WIP")?;
        if wip_reused {
            verify_workspace_report(&wip, &report, "LMLM WIP")?;
        } else {
            let converted = convert(&input, &wip)?;
            if converted != report {
                let message = concat!(
                    "conversion report changed between inspection ",
                    "and publication",
                );
                return Err(BatchError::Contract(message.to_owned()));
            }
        }
        let short_hash = report.source_sha256.get(..12).ok_or_else(|| {
            BatchError::Contract(
                "source SHA-256 is unexpectedly short".to_owned(),
            )
        })?;
        let export_name = format!("{}-{short_hash}", safe_export_stem(&input));
        let export = export_root.join(export_name);
        let export_reused =
            ensure_directory_or_missing(&export, "LMLM export")?;
        if export_reused {
            verify_workspace_report(&export, &report, "LMLM export")?;
            verify_workspace_package(&export, &report, "LMLM export")?;
        } else {
            copy_workspace(&wip, &export, &report.source_sha256)?;
        }
        let input_name = input
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                BatchError::Contract(
                    "import filename is not Unicode".to_owned(),
                )
            })?
            .to_owned();
        packages.push(BatchItem {
            input: input_name,
            source_sha256: report.source_sha256,
            wip: relative_text(repository_root, &wip)?,
            wip_reused,
            export: relative_text(repository_root, &export)?,
            export_reused,
        });
    }
    Ok(BatchReport {
        schema: BATCH_SCHEMA,
        packages,
    })
}

/// Runs the fixed repository folder workflow documented by this tool.
///
/// # Errors
///
/// Returns a deterministic failure when the repository-relative import, export,
/// WIP, validation, or publication contract fails.
pub fn run_default() -> Result<BatchReport, BatchError> {
    let tool_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository_root = tool_root
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| {
            BatchError::Contract("cannot resolve repository root".to_owned())
        })?;
    convert_folders(
        repository_root,
        &tool_root.join("import"),
        &tool_root.join("export"),
        &repository_root.join(".cache").join("lmlm").join("wip"),
    )
}
