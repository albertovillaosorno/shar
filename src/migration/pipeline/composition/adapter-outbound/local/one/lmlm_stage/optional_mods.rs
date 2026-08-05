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

use lmlm::{FileEntry, entry_bytes};
use rmv::Sha256;
use schoenwald_filesystem::adapters::driving::local::read_bytes as local_read_bytes;

use super::{PipelineOutcome, io_error, write_bytes};
use crate::adapters::driven::check_cancellation;
use crate::adapters::driven::local::filesystem::collect_files;
use crate::adapters::driven::local::progress::StageProgress;
use crate::domain::{PipelineError, escape_json as json_escape};

const REMASTER_ALIAS: &str = "m.lmlm";
const LATINO_ALIAS: &str = "j.lmlm";

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
pub(super) fn discover_optional_mods(game_root: &Path) -> PipelineOutcome<Vec<OptionalModArchive>> {
    let mods_root = game_root.join("mods");
    if !mods_root.exists() {
        return Ok(Vec::new());
    }
    let metadata = fs::symlink_metadata(&mods_root).map_err(io_error(&mods_root))?;
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
            .ok_or_else(|| PipelineError::new("optional package name is not UTF-8"))?
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
        let relative = path
            .strip_prefix(game_root)
            .map_err(|_error| PipelineError::new("failed to relativize base source file"))?;
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
        let relative = path
            .strip_prefix(extracted_root)
            .map_err(|_error| PipelineError::new("failed to relativize extracted base file"))?;
        let key = portable_identity(relative);
        if !extracted_keys.insert(key.clone()) {
            return Err(PipelineError::new(
                "case-insensitive collision in extracted base files",
            ));
        }
        let _previous = files.insert(key, path);
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
        value.eq_ignore_ascii_case("mods") || value.eq_ignore_ascii_case("extracted")
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
            PipelineError::new(format!("{}: LMLM entry out of bounds", entry.path))
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
        && (string_has_extension(file, "bik") || string_has_extension(file, "rmv"))
}

/// Stable case-insensitive relative identity.
pub(super) fn portable_identity(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase()
}

/// Public-safe generated path relative to the extraction root.
pub(super) fn relative_output(root: &Path, path: &Path) -> PipelineOutcome<String> {
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
