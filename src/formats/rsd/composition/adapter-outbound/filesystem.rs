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
//   - Filesystem outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem outbound adapter.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ffi::{OsStr, OsString};
use std::path::{Component, Path, PathBuf};

use same_file::Handle;
use schoenwald_filesystem::adapters::driving::local;
use schoenwald_filesystem::{PathKind, validate_portable_path};

use super::transaction::{resolve_target, write_pending_outputs};
use crate::domain::{
    ExportReport, RsdAudio, RsdError, RsdHeader, SourceRootReport,
};
use crate::ports::Exporter;

/// Filesystem exporter owns recursive discovery so source-relative output stays
/// deterministic.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemExporter;

/// One fully converted output waiting for the batch write phase.
pub(super) struct PendingOutput {
    /// Final WAV destination after all conversion succeeds.
    pub(super) destination: PathBuf,
    /// Validated RIFF bytes ready for filesystem materialization.
    pub(super) bytes: Vec<u8>,
}

/// Converts one source while preserving its path on codec failures.
fn convert_source(
    path: &Path,
    bytes: &[u8],
) -> Result<(RsdHeader, Vec<u8>), RsdError> {
    let wrap = |source| RsdError::SourceAudio {
        path: path.to_path_buf(),
        source: Box::new(source),
    };
    let audio = RsdAudio::parse(bytes).map_err(&wrap)?;
    let header = audio.header();
    let wav = audio.to_wav().map_err(&wrap)?;
    let wav_bytes = wav.to_bytes().map_err(wrap)?;
    Ok((header, wav_bytes))
}

/// Rejects intersecting trees before discovery can consume generated output.
fn validate_tree_separation(
    roots: &[PathBuf],
    output_root: &Path,
) -> Result<(), RsdError> {
    validate_portable_path(output_root)
        .map_err(|_error| RsdError::InvalidPath(output_root.to_path_buf()))?;
    match local::path_kind(output_root) {
        Ok(PathKind::Directory | PathKind::Missing) => {},
        Ok(PathKind::File | PathKind::Other) => {
            return Err(RsdError::InvalidOutputRoot(output_root.to_path_buf()));
        },
        Err(source) => {
            return Err(RsdError::io(
                output_root.to_path_buf(),
                source.to_string(),
            ));
        },
    }
    let output = resolve_target(output_root)?;
    for root in roots {
        if local::path_kind(root)
            .map_err(|error| RsdError::io(root.clone(), error.to_string()))?
            != PathKind::Directory
        {
            return Err(RsdError::InvalidSourceRoot(root.clone()));
        }
        let source = local::canonicalize(root)
            .map_err(|error| RsdError::io(root.clone(), error.to_string()))?;
        if output.starts_with(&source) || source.starts_with(&output) {
            return Err(RsdError::OverlappingOutputRoot { source, output });
        }
    }
    Ok(())
}

/// Discovers every RSD file and returns one stable path order.
fn discover_rsd_paths(root: &Path) -> Result<Vec<PathBuf>, RsdError> {
    local::regular_files(root)
        .map_err(|source| RsdError::io(root.to_path_buf(), source.to_string()))
        .map(|paths| {
            paths
                .into_iter()
                .filter(|path| has_rsd_extension(path))
                .collect()
        })
}

/// Claims one physical source so path aliases and hard links remain idempotent.
fn claim_source(
    path: &Path,
    identities: &mut HashSet<Handle>,
) -> Result<bool, RsdError> {
    let identity = Handle::from_path(path).map_err(|source| {
        RsdError::io(path.to_path_buf(), source.to_string())
    })?;
    Ok(identities.insert(identity))
}

/// Resolves one root identity without stealing later typed validation errors.
fn root_identity(root: &Path) -> PathBuf {
    match local::canonicalize(root) {
        Ok(identity) => identity,
        Err(_error) => root.to_path_buf(),
    }
}

/// Collapses physical root aliases while preserving the first caller spelling.
fn unique_roots(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut identities = BTreeSet::new();
    roots
        .iter()
        .filter(|root| identities.insert(root_identity(root)))
        .cloned()
        .collect()
}

/// Produces the filesystem identity used for one output folder name.
#[cfg(windows)]
pub(super) fn name_identity(name: &OsStr) -> Vec<u32> {
    use std::os::windows::ffi::OsStrExt as _;

    let mut identity = Vec::new();
    for decoded in char::decode_utf16(name.encode_wide()) {
        match decoded {
            Ok(character) => {
                identity.extend(character.to_lowercase().map(u32::from));
            },
            Err(error) => {
                identity.push(u32::MAX);
                identity.push(u32::from(error.unpaired_surrogate()));
            },
        }
    }
    identity
}

/// Produces the byte-exact Unix identity for one output folder name.
#[cfg(unix)]
pub(super) fn name_identity(name: &OsStr) -> Vec<u32> {
    use std::os::unix::ffi::OsStrExt as _;

    name.as_bytes().iter().copied().map(u32::from).collect()
}

/// Produces a stable fallback identity on other target platforms.
#[cfg(not(any(unix, windows)))]
pub(super) fn name_identity(name: &OsStr) -> Vec<u32> {
    name.to_string_lossy().chars().map(u32::from).collect()
}

impl FilesystemExporter {
    /// Resolves unique output folder identities before any file is written.
    fn validate_root_names(
        roots: &[PathBuf],
    ) -> Result<Vec<OsString>, RsdError> {
        let mut claimed = BTreeMap::<Vec<u32>, PathBuf>::new();
        let mut names = Vec::with_capacity(roots.len());
        for root in roots {
            let name = root
                .file_name()
                .ok_or_else(|| RsdError::InvalidRootName(root.clone()))?
                .to_os_string();
            let identity = name_identity(&name);
            if let Some(first) = claimed.insert(identity, root.clone()) {
                return Err(RsdError::CollidingRootName {
                    first,
                    second: root.clone(),
                });
            }
            names.push(name);
        }
        Ok(names)
    }

    /// Traverses one source root so per-root evidence cannot be mixed together.
    fn export_root(
        root: &Path,
        root_name: &OsStr,
        output_root: &Path,
        report: &mut ExportReport,
        source_identities: &mut HashSet<Handle>,
        pending_outputs: &mut Vec<PendingOutput>,
    ) -> Result<SourceRootReport, RsdError> {
        let mut root_report = SourceRootReport {
            root: root.to_path_buf(),
            files: 0,
            source_bytes: 0,
            wav_bytes: 0,
        };
        for path in discover_rsd_paths(root)? {
            if !claim_source(&path, source_identities)? {
                continue;
            }
            let relative = path
                .strip_prefix(root)
                .map_err(|_strip_error| RsdError::InvalidPath(path.clone()))?;
            let destination =
                destination_path(output_root, root_name, relative)?;
            let bytes = local::read_bytes(&path).map_err(|source| {
                RsdError::io(path.clone(), source.to_string())
            })?;
            let (header, wav_bytes) = convert_source(&path, &bytes)?;
            let source_len =
                u64::try_from(bytes.len()).map_err(|_conversion_error| {
                    RsdError::ReportOverflow(
                        "RSD source byte length exceeds report capacity",
                    )
                })?;
            let wav_len = u64::try_from(wav_bytes.len()).map_err(
                |_conversion_error| {
                    RsdError::ReportOverflow(
                        "RSD WAV byte length exceeds report capacity",
                    )
                },
            )?;
            pending_outputs.push(PendingOutput {
                destination,
                bytes: wav_bytes,
            });
            root_report.add_file(source_len, wav_len)?;
            report.add_file(header, source_len, wav_len)?;
        }
        Ok(root_report)
    }
}

impl Exporter for FilesystemExporter {
    type Error = RsdError;

    fn export_roots(
        &self,
        roots: &[PathBuf],
        output_root: &Path,
    ) -> Result<ExportReport, Self::Error> {
        if roots.is_empty() {
            return Err(RsdError::NoInputRoots);
        }
        let unique_roots = unique_roots(roots);
        let root_names = Self::validate_root_names(&unique_roots)?;
        validate_tree_separation(&unique_roots, output_root)?;
        let mut report = ExportReport::default();
        let mut source_identities = HashSet::new();
        let mut pending_outputs = Vec::new();
        for (root, root_name) in unique_roots.iter().zip(&root_names) {
            let root_report = Self::export_root(
                root,
                root_name,
                output_root,
                &mut report,
                &mut source_identities,
                &mut pending_outputs,
            )?;
            report.source_roots.push(root_report);
        }
        if report.total_files == 0_usize {
            return Err(RsdError::NoAudioInputs);
        }
        report.validate()?;
        write_pending_outputs(pending_outputs, output_root)?;
        Ok(report)
    }
}

/// Matches extensions case-insensitively because extracted asset trees vary by
/// source tooling.
fn has_rsd_extension(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rsd"))
}

/// Mirrors only normal relative components so archive-derived paths cannot
/// escape output.
/// Normalizes each discovered RSD path before changing the output extension.
fn destination_path(
    output_root: &Path,
    root_name: &OsStr,
    relative: &Path,
) -> Result<PathBuf, RsdError> {
    let mut out = output_root.join(Path::new(root_name));
    for component in relative.components() {
        match component {
            Component::Normal(part) => out.push(part),
            _ => return Err(RsdError::InvalidPath(relative.to_path_buf())),
        }
    }
    let _extension_changed = out.set_extension("wav");
    validate_portable_path(&out)
        .map_err(|_error| RsdError::InvalidPath(relative.to_path_buf()))?;
    Ok(out)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/rsd/unit/adapter-outbound/filesystem/tests.rs"]
mod tests;
