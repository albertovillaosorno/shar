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
//   - Filesystem movie auditor outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem movie auditor outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem movie auditor outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::{
    AuditReport, MovieKind, MovieRecord, ProvenanceEvidence, RmvError, Sha256,
    is_windows_safe_component,
};
use crate::ports::MovieAuditor;

#[derive(Debug, Default, Clone, Copy)]
/// Filesystemmovieauditor.
pub struct FilesystemMovieAuditor;

impl MovieAuditor for FilesystemMovieAuditor {
    fn audit_roots(
        &self,
        roots: &[PathBuf],
        output_root: &Path,
    ) -> Result<AuditReport, RmvError> {
        if roots.is_empty() {
            return Err(RmvError::NoInputRoots);
        }
        let mut report = AuditReport::default();
        let mut source_identities = BTreeSet::new();
        let excluded_output_root = local::canonicalize(output_root).ok();
        let mut ordered_roots = roots.to_vec();
        ordered_roots.sort();
        for root in &ordered_roots {
            let canonical_root = local::canonicalize(root)
                .map_err(|source| RmvError::io(root.clone(), source))?;
            if excluded_output_root
                .as_ref()
                .is_some_and(|output_identity| {
                    canonical_root.starts_with(output_identity)
                })
            {
                return Err(RmvError::InputRootInsideOutput(root.clone()));
            }
            let Some(root_name) = canonical_root.file_name() else {
                return Err(RmvError::InvalidRootName(root.clone()));
            };
            audit_root(
                root,
                root_name,
                output_root,
                excluded_output_root.as_deref(),
                &mut report,
                &mut source_identities,
            )?;
        }
        report.records.sort_by(|left, right| {
            left.source_root
                .cmp(&right.source_root)
                .then_with(|| left.relative_path.cmp(&right.relative_path))
        });
        let mut output_paths = BTreeSet::new();
        for record in &report.records {
            if !output_paths.insert(output_identity(&record.output_path)) {
                return Err(RmvError::OutputPathCollision(
                    record.output_path.clone(),
                ));
            }
        }
        if report.records.is_empty() {
            return Err(RmvError::NoMovieInputs);
        }
        let mut counts = BTreeMap::new();
        for record in &report.records {
            let count = counts.entry(record.hash).or_insert(0_usize);
            *count = (*count).saturating_add(1);
            if !is_valid_bink2_output(&record.output_path) {
                report.missing_bk2_outputs =
                    report.missing_bk2_outputs.saturating_add(1);
            }
        }
        report.duplicate_inputs =
            counts.values().map(|count| count.saturating_sub(1)).sum();
        Ok(report)
    }
}

/// Reports whether an expected output is a regular Bink2 file.
fn is_valid_bink2_output(path: &Path) -> bool {
    let Ok(bytes) = local::read_bytes(path) else {
        return false;
    };
    MovieKind::from_bytes(&bytes) == MovieKind::BinkV2
}

/// Returns a one-to-one uppercase mapping for Windows path identity.
#[cfg(windows)]
fn windows_case_character(character: char) -> char {
    let mut uppercase = character.to_uppercase();
    let first_uppercase = uppercase.next().unwrap_or(character);
    if uppercase.next().is_some() {
        character
    } else {
        first_uppercase
    }
}

/// Produces a platform-accurate output identity for collision checks.
#[cfg(windows)]
fn output_identity(path: &Path) -> Vec<u32> {
    use std::os::windows::ffi::OsStrExt as _;

    let mut identity = Vec::new();
    for decoded in char::decode_utf16(path.as_os_str().encode_wide()) {
        match decoded {
            Ok(character) => {
                let identity_character = windows_case_character(character);
                identity.push(u32::from(identity_character));
            },
            Err(error) => {
                identity
                    .push(0x11_0000 + u32::from(error.unpaired_surrogate()));
            },
        }
    }
    identity
}

/// Produces a byte-exact Unix output identity for collision checks.
#[cfg(unix)]
fn output_identity(path: &Path) -> Vec<u32> {
    use std::os::unix::ffi::OsStrExt as _;

    path.as_os_str()
        .as_bytes()
        .iter()
        .copied()
        .map(u32::from)
        .collect()
}

/// Produces a stable fallback output identity on other platforms.
#[cfg(not(any(unix, windows)))]
fn output_identity(path: &Path) -> Vec<u32> {
    path.to_string_lossy().chars().map(u32::from).collect()
}

/// Audit root.
#[expect(
    clippy::filetype_is_file,
    // jig-ignore-next-line: exact syntax is indivisible
    reason = "Movie discovery accepts regular RMV files only and ignores symlinks and special entries"
)]
fn audit_root(
    root: &Path,
    root_name: &std::ffi::OsStr,
    output_root: &Path,
    excluded_output_root: Option<&Path>,
    report: &mut AuditReport,
    source_identities: &mut BTreeSet<PathBuf>,
) -> Result<(), RmvError> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(current) = stack.pop() {
        for dir_entry in fs::read_dir(&current)
            .map_err(|source| RmvError::io(current.clone(), source))?
        {
            let entry = dir_entry
                .map_err(|source| RmvError::io(current.clone(), source))?;
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(|source| RmvError::io(path.clone(), source))?;
            if file_type.is_dir() {
                let is_output_root =
                    excluded_output_root.is_some_and(|excluded| {
                        local::canonicalize(&path)
                            .is_ok_and(|identity| identity == excluded)
                    });
                if !is_output_root {
                    stack.push(path);
                }
            } else if file_type.is_file() && has_rmv_extension(&path) {
                let source_identity = local::canonicalize(&path)
                    .map_err(|source| RmvError::io(path.clone(), source))?;
                if !source_identities.insert(source_identity) {
                    continue;
                }
                let bytes = local::read_bytes(&path)
                    .map_err(|source| RmvError::io(path.clone(), source))?;
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_error| RmvError::InvalidPath(path.clone()))?;
                let output_path =
                    destination_path(output_root, root_name, relative)?;
                report.records.push(MovieRecord {
                    source_root: root.to_path_buf(),
                    source_path: path.clone(),
                    relative_path: relative.to_path_buf(),
                    output_path,
                    bytes: u64::try_from(bytes.len()).unwrap_or(u64::MAX),
                    kind: MovieKind::from_bytes(&bytes),
                    hash: Sha256::digest(&bytes),
                    provenance: ProvenanceEvidence::from_bytes(&bytes),
                });
            }
        }
    }
    Ok(())
}

/// Has rmv extension.
fn has_rmv_extension(path: &Path) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rmv"))
}

/// Destination path.
fn destination_path(
    output_root: &Path,
    root_name: &std::ffi::OsStr,
    relative: &Path,
) -> Result<PathBuf, RmvError> {
    let Some(root_name_text) = root_name.to_str() else {
        return Err(RmvError::InvalidRootName(PathBuf::from(root_name)));
    };
    if !is_windows_safe_component(root_name_text) {
        return Err(RmvError::InvalidRootName(PathBuf::from(root_name)));
    }
    let mut out = output_root.join(Path::new(root_name));
    for component in relative.components() {
        match component {
            Component::Normal(part) => {
                let Some(part_text) = part.to_str() else {
                    return Err(RmvError::InvalidPath(relative.to_path_buf()));
                };
                if !is_windows_safe_component(part_text) {
                    return Err(RmvError::InvalidPath(relative.to_path_buf()));
                }
                out.push(part);
            },
            _ => return Err(RmvError::InvalidPath(relative.to_path_buf())),
        }
    }
    let _replaced_extension = out.set_extension("bk2");
    Ok(out)
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/formats/rmv/unit/adapter-outbound/filesystem_movie_auditor/tests.rs"]
mod tests;
