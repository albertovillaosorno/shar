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
//   - Read-only optional-package planning and canonical preview rendering.
// - Must-Not:
//   - Write persistent extraction output or expose local package paths.
// - Allows:
//   - Temporary media decoding required to predict normalized output exactly.
// - Split-When:
//   - Another preview schema or package role gains an independent lifecycle.
// - Merge-When:
//   - Extraction and preview no longer require separate effect boundaries.
// - Summary:
//   - Shows exactly which supported package members would write or be skipped.
// - Description:
//   - Reuses package admission and media normalization rules without publishing
//     generated extraction files.
// - Usage:
//   - Called by the optional-mod dry-run application use case.
// - Defaults:
//   - Every package is structurally validated before preview evidence is built.
//

//! Read-only optional-package planning and canonical preview rendering.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use lmlm::{FileEntry, entry_bytes, parse as parse_lmlm};
use rmv::Sha256;
use serde::Serialize;

use super::optional_mods::{
    OptionalModArchive, OptionalModRole, create_optional_mod_work_root,
    discover_optional_mods, existing_file_index, is_latino_audio_path,
    is_latino_movie_path, optional_mod_approval_token, portable_identity,
    read_optional_mod_bytes, relative_output, remaster_relative_path,
};
use super::{
    PipelineOutcome, decode_lmlm_movie_audio, lmlm_entry_path, rsd_bytes_to_wav,
};
use crate::adapters::driven::check_cancellation;
use crate::domain::{
    OPTIONAL_MOD_PREVIEW_SCHEMA, OptionalModPreview, PipelineError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum PreviewAction {
    Add,
    Replace,
    Skip,
}

impl PreviewAction {
    const fn writes(self) -> bool {
        !matches!(self, Self::Skip)
    }
}

#[derive(Debug, Clone, Serialize)]
struct PreviewChange {
    alias: &'static str,
    role: &'static str,
    source: String,
    action: PreviewAction,
    output: Option<String>,
    reason: &'static str,
    normalized_bytes: u64,
    sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct PreviewPackage {
    alias: &'static str,
    role: &'static str,
    package_bytes: u64,
    package_sha256: String,
    members: usize,
    would_write: usize,
    would_skip: usize,
    normalized_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
struct PreviewAliases {
    #[serde(rename = "m.lmlm")]
    remaster: &'static str,
    #[serde(rename = "j.lmlm")]
    latino: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct PreviewSummary {
    packages: usize,
    would_write: usize,
    would_skip: usize,
    normalized_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
struct PreviewDocument {
    schema: &'static str,
    dry_run: bool,
    aliases: PreviewAliases,
    approval_token: Option<String>,
    packages: Vec<PreviewPackage>,
    changes: Vec<PreviewChange>,
    summary: PreviewSummary,
}

#[derive(Debug)]
struct ParsedArchive {
    archive: OptionalModArchive,
    data: Vec<u8>,
    entries: Vec<FileEntry>,
}

/// Builds a canonical read-only preview for zero, one, or both packages.
pub(in crate::adapters::driven::local) fn preview_optional_mods(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<OptionalModPreview> {
    let archives = discover_optional_mods(game_root)?;
    let parsed = parse_archives(archives)?;
    let generated_mod_root = extracted_root.join("lmlm");
    let base_files = if parsed
        .iter()
        .any(|parsed| parsed.archive.role == OptionalModRole::Remaster)
    {
        existing_file_index(game_root, extracted_root, &generated_mod_root)?
    } else {
        BTreeMap::new()
    };
    let work_root = create_optional_mod_work_root("preview")?;
    let result = build_preview(
        &parsed,
        extracted_root,
        &generated_mod_root,
        &base_files,
        work_root.path(),
    );
    let cleanup = work_root.cleanup();
    match (result, cleanup) {
        (Ok(preview), Ok(())) => Ok(preview),
        (Err(error), _) => Err(error),
        (Ok(_preview), Err(error)) => Err(error),
    }
}

fn parse_archives(
    archives: Vec<OptionalModArchive>,
) -> PipelineOutcome<Vec<ParsedArchive>> {
    archives
        .into_iter()
        .map(|archive| {
            check_cancellation()?;
            let data = read_optional_mod_bytes(&archive)?;
            let entries = parse_lmlm(&data).map_err(|error| {
                PipelineError::new(format!("{}: {error}", archive.role.alias()))
            })?;
            Ok(ParsedArchive {
                archive,
                data,
                entries,
            })
        })
        .collect()
}

fn build_preview(
    parsed: &[ParsedArchive],
    extracted_root: &Path,
    generated_mod_root: &Path,
    base_files: &BTreeMap<String, PathBuf>,
    work_root: &Path,
) -> PipelineOutcome<OptionalModPreview> {
    let approval_token = optional_mod_approval_token(
        parsed
            .iter()
            .map(|item| (item.archive.role, item.data.as_slice())),
    );
    let mut packages = Vec::new();
    let mut changes = Vec::new();
    for parsed_archive in parsed {
        check_cancellation()?;
        let archive = &parsed_archive.archive;
        let entries = &parsed_archive.entries;
        let data = &parsed_archive.data;
        let package_changes = match archive.role {
            OptionalModRole::Remaster => preview_remaster(
                archive.role,
                data,
                entries,
                extracted_root,
                base_files,
            )?,
            OptionalModRole::Latino => preview_latino(
                archive.role,
                data,
                entries,
                extracted_root,
                generated_mod_root,
                work_root,
            )?,
        };
        let writes = package_changes
            .iter()
            .filter(|change| change.action.writes())
            .count();
        let skips = package_changes.len().saturating_sub(writes);
        let normalized_bytes = checked_normalized_bytes(&package_changes)?;
        packages.push(PreviewPackage {
            alias: archive.role.alias(),
            role: archive.role.label(),
            package_bytes: u64::try_from(data.len()).unwrap_or(u64::MAX),
            package_sha256: Sha256::digest(data).hex(),
            members: entries.len(),
            would_write: writes,
            would_skip: skips,
            normalized_bytes,
        });
        changes.extend(package_changes);
    }
    render_preview(packages, changes, approval_token)
}

fn preview_remaster(
    role: OptionalModRole,
    data: &[u8],
    entries: &[FileEntry],
    extracted_root: &Path,
    base_files: &BTreeMap<String, PathBuf>,
) -> PipelineOutcome<Vec<PreviewChange>> {
    let mut claimed_outputs = BTreeSet::new();
    let mut changes = Vec::new();
    for entry in entries {
        check_cancellation()?;
        let Some(relative) = remaster_relative_path(&entry.path) else {
            changes.push(skipped_change(role, entry, "unsupported_wrapper"));
            continue;
        };
        let key = portable_identity(Path::new(&relative));
        let Some(destination) = base_files.get(&key) else {
            changes.push(skipped_change(role, entry, "not_base_identity"));
            continue;
        };
        if !claimed_outputs.insert(key) {
            return Err(PipelineError::new(
                "remaster maps multiple members to one base file",
            ));
        }
        let bytes = required_entry_bytes(data, entry)?;
        changes.push(writing_change(
            role,
            entry,
            PreviewAction::Replace,
            relative_output(extracted_root, destination)?,
            "existing_base_identity",
            bytes,
        ));
    }
    Ok(changes)
}

fn preview_latino(
    role: OptionalModRole,
    data: &[u8],
    entries: &[FileEntry],
    extracted_root: &Path,
    generated_mod_root: &Path,
    work_root: &Path,
) -> PipelineOutcome<Vec<PreviewChange>> {
    let output_root = generated_mod_root.join("latino");
    let mut claimed_outputs = BTreeSet::new();
    let mut changes = Vec::new();
    for entry in entries {
        check_cancellation()?;
        if is_latino_audio_path(&entry.path) {
            let source = required_entry_bytes(data, entry)?;
            let wav = rsd_bytes_to_wav(source, &entry.path)?;
            let destination = lmlm_entry_path(&output_root, &entry.path)
                .with_extension("wav");
            push_unique_output(
                &mut claimed_outputs,
                &destination,
                "Latino package maps multiple members to one voice output",
            )?;
            changes.push(writing_change(
                role,
                entry,
                PreviewAction::Add,
                relative_output(extracted_root, &destination)?,
                "supported_voice_audio",
                &wav,
            ));
        } else if is_latino_movie_path(&entry.path) {
            let source = required_entry_bytes(data, entry)?;
            let Some(wav) =
                decode_lmlm_movie_audio(work_root, &entry.path, source)?
            else {
                changes.push(skipped_change(role, entry, "no_audio_stream"));
                continue;
            };
            let stem = Path::new(&entry.path)
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("movie");
            let destination = output_root
                .join("movies")
                .join(stem)
                .join("audio_track_01.wav");
            push_unique_output(
                &mut claimed_outputs,
                &destination,
                "Latino package maps multiple members to one movie output",
            )?;
            changes.push(writing_change(
                role,
                entry,
                PreviewAction::Add,
                relative_output(extracted_root, &destination)?,
                "supported_cinematic_audio",
                &wav,
            ));
        } else {
            changes.push(skipped_change(role, entry, "unsupported_member"));
        }
    }
    Ok(changes)
}

fn required_entry_bytes<'archive>(
    data: &'archive [u8],
    entry: &FileEntry,
) -> PipelineOutcome<&'archive [u8]> {
    entry_bytes(data, entry).ok_or_else(|| {
        PipelineError::new(format!("{}: LMLM entry out of bounds", entry.path))
    })
}

fn push_unique_output(
    claimed: &mut BTreeSet<String>,
    destination: &Path,
    message: &'static str,
) -> PipelineOutcome<()> {
    if claimed.insert(portable_identity(destination)) {
        Ok(())
    } else {
        Err(PipelineError::new(message))
    }
}

fn skipped_change(
    role: OptionalModRole,
    entry: &FileEntry,
    reason: &'static str,
) -> PreviewChange {
    PreviewChange {
        alias: role.alias(),
        role: role.label(),
        source: entry.path.clone(),
        action: PreviewAction::Skip,
        output: None,
        reason,
        normalized_bytes: 0,
        sha256: None,
    }
}

fn writing_change(
    role: OptionalModRole,
    entry: &FileEntry,
    action: PreviewAction,
    output: String,
    reason: &'static str,
    normalized: &[u8],
) -> PreviewChange {
    PreviewChange {
        alias: role.alias(),
        role: role.label(),
        source: entry.path.clone(),
        action,
        output: Some(output),
        reason,
        normalized_bytes: u64::try_from(normalized.len()).unwrap_or(u64::MAX),
        sha256: Some(Sha256::digest(normalized).hex()),
    }
}

fn checked_normalized_bytes(changes: &[PreviewChange]) -> PipelineOutcome<u64> {
    changes.iter().try_fold(0_u64, |total, change| {
        total.checked_add(change.normalized_bytes).ok_or_else(|| {
            PipelineError::new("optional-mod preview bytes overflowed")
        })
    })
}

fn render_preview(
    packages: Vec<PreviewPackage>,
    changes: Vec<PreviewChange>,
    approval_token: Option<String>,
) -> PipelineOutcome<OptionalModPreview> {
    let writes = changes
        .iter()
        .filter(|change| change.action.writes())
        .count();
    let skips = changes.len().saturating_sub(writes);
    let normalized_bytes = checked_normalized_bytes(&changes)?;
    let document = PreviewDocument {
        schema: OPTIONAL_MOD_PREVIEW_SCHEMA,
        dry_run: true,
        aliases: PreviewAliases {
            remaster: "remaster",
            latino: "latino",
        },
        approval_token: approval_token.clone(),
        summary: PreviewSummary {
            packages: packages.len(),
            would_write: writes,
            would_skip: skips,
            normalized_bytes,
        },
        packages,
        changes,
    };
    let json = serde_json::to_string(&document).map_err(|error| {
        PipelineError::new(format!(
            "failed to render optional-mod preview: {error}"
        ))
    })?;
    Ok(OptionalModPreview::new(
        json,
        document.summary.packages,
        document.summary.would_write,
        document.summary.would_skip,
        document.summary.normalized_bytes,
        approval_token,
    ))
}
