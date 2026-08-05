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
//   - Supported optional-mod discovery and remaster overlay policy.
// - Must-Not:
//   - Identify packages by release title or publish package payloads.
// - Allows:
//   - Stable local aliases and generated extraction output.
// - Split-When:
//   - Another optional package role gains an independent lifecycle.
// - Merge-When:
//   - Optional package roles no longer require distinct policies.
// - Summary:
//   - Applies supported local LMLM packages without weakening base fidelity.
// - Description:
//   - Selects behavior by m.lmlm and j.lmlm and limits remaster writes to files
//     present in the unmodified source or extracted snapshot.
// - Usage:
//   - Called by the owning LMLM pipeline stage.
// - Defaults:
//   - Unknown package aliases fail closed.
//

//! Supported optional-mod discovery and remaster overlay policy.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use lmlm::{FileEntry, entry_bytes};
use rmv::Sha256;
use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all as local_create_dir_all, read_bytes as local_read_bytes,
};

use super::{PipelineOutcome, io_error, write_bytes};
use crate::adapters::driven::check_cancellation;
use crate::adapters::driven::local::filesystem::collect_files;
use crate::adapters::driven::local::progress::StageProgress;
use crate::domain::{PipelineError, escape_json as json_escape};

const REMASTER_ALIAS: &str = "m.lmlm";
const LATINO_ALIAS: &str = "j.lmlm";
/// Per-process nonce for collision-free optional-package work roots.
static NEXT_WORK_ROOT: AtomicU64 = AtomicU64::new(0);
/// Maximum existing work roots skipped before failing closed.
const MAX_WORK_ROOT_ATTEMPTS: usize = 1_024;

/// Behavior selected by one stable local filename.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum OptionalModRole {
    /// Replaces only files present in the base source or extracted snapshot.
    Remaster,
    /// Adds Latin-American dialogue and cinematic audio in isolation.
    Latino,
}

impl OptionalModRole {
    /// Deterministic application order.
    const fn order(self) -> u8 {
        match self {
            Self::Remaster => 0,
            Self::Latino => 1,
        }
    }

    /// Stable generated-manifest label.
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Remaster => "remaster",
            Self::Latino => "latino",
        }
    }

    /// Canonical filename that selects this behavior.
    pub(super) const fn alias(self) -> &'static str {
        match self {
            Self::Remaster => REMASTER_ALIAS,
            Self::Latino => LATINO_ALIAS,
        }
    }
}

/// One locally supplied optional package.
#[derive(Debug, Clone)]
pub(super) struct OptionalModArchive {
    /// Selected behavior.
    pub(super) role: OptionalModRole,
    /// Package path under the local mods directory.
    pub(super) path: PathBuf,
}

/// Builds one public-safe temporary-workspace diagnostic.
pub(super) fn optional_workspace_error(
    action: &str,
    error: &std::io::Error,
) -> PipelineError {
    PipelineError::new(format!(
        "optional-package workspace {action} failed ({:?})",
        error.kind()
    ))
}

/// One uniquely owned temporary optional-package workspace.
#[derive(Debug)]
pub(super) struct OptionalModWorkRoot {
    path: PathBuf,
    cleaned: bool,
}

impl OptionalModWorkRoot {
    /// Returns the uniquely claimed workspace path.
    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    /// Removes the workspace and reports cleanup failures.
    pub(super) fn cleanup(mut self) -> PipelineOutcome<()> {
        fs::remove_dir_all(&self.path)
            .map_err(|error| optional_workspace_error("cleanup", &error))?;
        self.cleaned = true;
        Ok(())
    }
}

impl Drop for OptionalModWorkRoot {
    fn drop(&mut self) {
        if !self.cleaned {
            drop(fs::remove_dir_all(&self.path));
        }
    }
}

/// Atomically claims one unique temporary optional-package workspace.
#[expect(
    clippy::create_dir,
    reason = "Unique workspace ownership needs atomic directory creation."
)]
pub(super) fn create_optional_mod_work_root(
    label: &str,
) -> PipelineOutcome<OptionalModWorkRoot> {
    let parent = std::env::temp_dir().join("shar-schoenwald");
    local_create_dir_all(&parent)
        .map_err(|error| optional_workspace_error("parent creation", &error))?;
    let metadata = fs::symlink_metadata(&parent).map_err(|error| {
        optional_workspace_error("parent inspection", &error)
    })?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(PipelineError::new(
            "optional-package temporary parent must be a real directory",
        ));
    }
    for _attempt in 0..MAX_WORK_ROOT_ATTEMPTS {
        let sequence = NEXT_WORK_ROOT.fetch_add(1, Ordering::Relaxed);
        let candidate = parent
            .join(format!("lmlm-{label}-{}-{sequence}", std::process::id()));
        match fs::create_dir(&candidate) {
            Ok(()) => {
                return Ok(OptionalModWorkRoot {
                    path: candidate,
                    cleaned: false,
                });
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            }
            Err(error) => {
                return Err(optional_workspace_error("allocation", &error));
            }
        }
    }
    Err(PipelineError::new(
        "failed to allocate a unique optional-package workspace",
    ))
}

/// Reads one discovered package without exposing its local location.
pub(super) fn read_optional_mod_bytes(
    archive: &OptionalModArchive,
) -> PipelineOutcome<Vec<u8>> {
    local_read_bytes(&archive.path).map_err(|error| {
        PipelineError::new(format!(
            "{}: optional package read failed ({:?})",
            archive.role.alias(),
            error.kind()
        ))
    })
}

/// Computes a public-safe identity for one ordered package byte set.
pub(super) fn optional_mod_approval_token<'a>(
    packages: impl IntoIterator<Item = (OptionalModRole, &'a [u8])>,
) -> Option<String> {
    let mut evidence = Vec::new();
    evidence.extend_from_slice(b"shar-schoenwald.optional-mod-approval.v1\n");
    let mut count = 0_usize;
    for (role, bytes) in packages {
        count = count.saturating_add(1);
        evidence.extend_from_slice(role.alias().as_bytes());
        evidence.push(0);
        evidence.extend_from_slice(bytes.len().to_string().as_bytes());
        evidence.push(0);
        evidence.extend_from_slice(Sha256::digest(bytes).hex().as_bytes());
        evidence.push(b'\n');
    }
    (count != 0).then(|| Sha256::digest(&evidence).hex())
}

/// Verifies approval against one exact ordered package byte set.
pub(super) fn require_package_byte_approval<'a>(
    packages: impl IntoIterator<Item = (OptionalModRole, &'a [u8])>,
    approved: Option<&str>,
) -> PipelineOutcome<()> {
    match (optional_mod_approval_token(packages), approved) {
        (None, None) => Ok(()),
        (Some(_actual), None) => Err(PipelineError::new(concat!(
            "optional packages require an approval token from the current ",
            "preview"
        ))),
        (Some(actual), Some(expected)) if actual == expected => Ok(()),
        (_, Some(_)) => Err(PipelineError::new(concat!(
            "optional package approval token does not match the current ",
            "package set; rerun the preview"
        ))),
    }
}

/// Discovers packages and verifies their exact bytes before mutation.
pub(super) fn require_optional_mod_approval(
    game_root: &Path,
    approved: Option<&str>,
) -> PipelineOutcome<()> {
    let archives = discover_optional_mods(game_root)?;
    let packages = archives
        .iter()
        .map(|archive| {
            check_cancellation()?;
            let bytes = read_optional_mod_bytes(archive)?;
            Ok((archive.role, bytes))
        })
        .collect::<PipelineOutcome<Vec<_>>>()?;
    require_package_byte_approval(
        packages
            .iter()
            .map(|(role, bytes)| (*role, bytes.as_slice())),
        approved,
    )
}

/// Counters produced by one package application.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct OptionalModCounts {
    /// Files written.
    pub(super) written: usize,
    /// Safe but inapplicable members ignored.
    pub(super) skipped: usize,
    /// Bytes written.
    pub(super) bytes: u64,
}

/// Discovers only direct canonical aliases under `game/mods`.
pub(super) fn discover_optional_mods(
    game_root: &Path,
) -> PipelineOutcome<Vec<OptionalModArchive>> {
    let mods_root = game_root.join("mods");
    if !mods_root.exists() {
        return Ok(Vec::new());
    }
    let metadata =
        fs::symlink_metadata(&mods_root).map_err(io_error(&mods_root))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(PipelineError::new("game/mods must be a real directory"));
    }

    let mut archives = Vec::new();
    let mut roles = BTreeSet::new();
    for entry in fs::read_dir(&mods_root).map_err(io_error(&mods_root))? {
        let path = entry.map_err(io_error(&mods_root))?.path();
        if !has_extension(&path, "lmlm") {
            continue;
        }
        let metadata = fs::symlink_metadata(&path).map_err(io_error(&path))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(PipelineError::new(
                "optional LMLM packages must be regular files",
            ));
        }
        let file_name = path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                PipelineError::new("optional package name is not UTF-8")
            })?
            .to_ascii_lowercase();
        let role = match file_name.as_str() {
            REMASTER_ALIAS => OptionalModRole::Remaster,
            LATINO_ALIAS => OptionalModRole::Latino,
            _ => {
                return Err(PipelineError::new(format!(
                    "unsupported optional LMLM filename: {file_name}; \
                     use m.lmlm or j.lmlm"
                )));
            }
        };
        if !roles.insert(role.order()) {
            return Err(PipelineError::new(format!(
                "duplicate optional {} package",
                role.label()
            )));
        }
        archives.push(OptionalModArchive { role, path });
    }
    archives.sort_by_key(|archive| archive.role.order());
    Ok(archives)
}

/// Captures the exact pre-mod file identities the remaster may replace.
pub(super) fn existing_file_index(
    game_root: &Path,
    extracted_root: &Path,
    generated_mod_root: &Path,
) -> PipelineOutcome<BTreeMap<String, PathBuf>> {
    let mut files = BTreeMap::new();
    let mut source_keys = BTreeSet::new();
    for path in collect_files(game_root)? {
        let relative = path.strip_prefix(game_root).map_err(|_error| {
            PipelineError::new("failed to relativize base source file")
        })?;
        if is_excluded_game_path(relative) {
            continue;
        }
        let key = portable_identity(relative);
        if !source_keys.insert(key.clone()) {
            return Err(PipelineError::new(
                "case-insensitive collision in base source files",
            ));
        }
        let _previous = files.insert(key, extracted_root.join(relative));
    }

    let mut extracted_keys = BTreeSet::new();
    for path in collect_files(extracted_root)? {
        if path.starts_with(generated_mod_root) {
            continue;
        }
        let relative = path.strip_prefix(extracted_root).map_err(|_error| {
            PipelineError::new("failed to relativize extracted base file")
        })?;
        let key = portable_identity(relative);
        if !extracted_keys.insert(key.clone()) {
            return Err(PipelineError::new(
                "case-insensitive collision in extracted base files",
            ));
        }
        if source_keys.contains(&key) {
            let _previous = files.insert(key, path);
        }
    }
    Ok(files)
}

/// Excludes generated ledgers and optional package inputs from base identity.
fn is_excluded_game_path(relative: &Path) -> bool {
    let first = relative
        .components()
        .next()
        .and_then(|component| component.as_os_str().to_str());
    if first.is_some_and(|value| {
        value.eq_ignore_ascii_case("mods")
            || value.eq_ignore_ascii_case("extracted")
    }) {
        return true;
    }
    relative
        .file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|value| {
            value.eq_ignore_ascii_case("manifest.jsonl")
                || value.eq_ignore_ascii_case("manifest-expanded.jsonl")
        })
}

/// Applies only remaster members that target a pre-existing base file.
pub(super) fn apply_remaster(
    data: &[u8],
    entries: &[FileEntry],
    extracted_root: &Path,
    base_files: &BTreeMap<String, PathBuf>,
    records: &mut Vec<String>,
) -> PipelineOutcome<OptionalModCounts> {
    let mut counts = OptionalModCounts::default();
    let mut claimed_outputs = BTreeSet::new();
    let mut progress = StageProgress::begin("remaster members", entries.len());
    for (index, entry) in entries.iter().enumerate() {
        check_cancellation()?;
        progress.advance(&format!("member {}", index.saturating_add(1)));
        let Some(relative) = remaster_relative_path(&entry.path) else {
            counts.skipped = counts.skipped.saturating_add(1);
            continue;
        };
        let key = portable_identity(Path::new(&relative));
        let Some(destination) = base_files.get(&key) else {
            counts.skipped = counts.skipped.saturating_add(1);
            continue;
        };
        if !claimed_outputs.insert(key) {
            return Err(PipelineError::new(
                "remaster maps multiple members to one base file",
            ));
        }
        let bytes = entry_bytes(data, entry).ok_or_else(|| {
            PipelineError::new(format!(
                "{}: LMLM entry out of bounds",
                entry.path
            ))
        })?;
        write_bytes(destination, bytes)?;
        counts.written = counts.written.saturating_add(1);
        counts.bytes = counts
            .bytes
            .saturating_add(u64::try_from(bytes.len()).unwrap_or(u64::MAX));
        records.push(format!(
            concat!(
                "{{\"kind\":\"remaster_replacement\",",
                "\"source\":\"{}\",",
                "\"output\":\"{}\",",
                "\"bytes\":{},",
                "\"sha256\":\"{}\"}}"
            ),
            json_escape(&entry.path),
            json_escape(&relative_output(extracted_root, destination)?),
            bytes.len(),
            Sha256::digest(bytes).hex()
        ));
    }
    progress.finish();
    Ok(counts)
}

/// Removes the loader wrapper without depending on release title or version.
pub(super) fn remaster_relative_path(entry_path: &str) -> Option<String> {
    let (first, remainder) = entry_path.split_once('/')?;
    if first.eq_ignore_ascii_case("customfiles") && !remainder.is_empty() {
        return Some(remainder.to_owned());
    }

    let mut segments = entry_path.splitn(4, '/');
    let root = segments.next()?;
    let _wrapper = segments.next()?;
    let content_root = segments.next()?;
    let relative = segments.next()?;
    if root.eq_ignore_ascii_case("mods")
        && content_root.eq_ignore_ascii_case("customfiles")
        && !relative.is_empty()
    {
        return Some(relative.to_owned());
    }
    None
}

/// Returns whether a member is Latin-American voice audio.
pub(super) fn is_latino_audio_path(entry_path: &str) -> bool {
    entry_path.split_once('/').is_some_and(|(root, relative)| {
        root.eq_ignore_ascii_case("customfiles")
            && !relative.is_empty()
            && string_has_extension(entry_path, "rsd")
    })
}

/// Returns whether a member is Latin-American cinematic media.
pub(super) fn is_latino_movie_path(entry_path: &str) -> bool {
    let mut segments = entry_path.split('/');
    let Some(root) = segments.next() else {
        return false;
    };
    let Some(directory) = segments.next() else {
        return false;
    };
    let Some(file) = segments.next() else {
        return false;
    };
    root.eq_ignore_ascii_case("customfiles")
        && directory.eq_ignore_ascii_case("movies")
        && !file.is_empty()
        && segments.next().is_none()
        && (string_has_extension(file, "bik")
            || string_has_extension(file, "rmv"))
}

/// Stable case-insensitive relative identity.
pub(super) fn portable_identity(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase()
}

/// Public-safe generated path relative to the extraction root.
pub(super) fn relative_output(
    root: &Path,
    path: &Path,
) -> PipelineOutcome<String> {
    path.strip_prefix(root)
        .map(portable_identity)
        .map_err(|_error| PipelineError::new("output escaped extraction root"))
}

/// Filesystem path extension predicate.
fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
}

/// Archive path extension predicate.
fn string_has_extension(path: &str, expected: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
}
