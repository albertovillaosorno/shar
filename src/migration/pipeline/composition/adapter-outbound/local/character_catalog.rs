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
//   - Character catalog outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Character catalog outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Character catalog outbound adapter.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all, path_kind, write_bytes,
};
use schoenwald_filesystem::domain::PathKind;
use serde_json::json;

use super::fbx_export::export_prepared_character_package;
use crate::domain::package::PhaseThreePackageIndex;
use crate::domain::{PipelineError, StageReport};

/// Stable stage identity for complete character catalog publication.
const STAGE: &str = "fbx-export-characters";

/// Export every skinned character presentation through one root transaction.
///
/// # Errors
///
/// Returns an error when the index is invalid, no character package exists, any
/// package fails preparation or verification, output already exists, or final
/// publication cannot complete atomically.
pub(super) fn export_character_catalog(
    index_path: &Path,
    output_dir: &Path,
    base_root: &Path,
) -> Result<StageReport, PipelineError> {
    ensure_missing(output_dir, "character catalog output")?;
    let staging = staging_path(output_dir)?;
    ensure_missing(&staging, "character catalog staging")?;
    create_dir_all(&staging).map_err(|error| {
        PipelineError::new(format!("catalog staging failed: {error}"))
    })?;
    let result = build_catalog(index_path, &staging, base_root).and_then(
        |(package_count, files, bytes)| {
            std::fs::rename(&staging, output_dir).map_err(|error| {
                PipelineError::new(format!(
                    "catalog publication failed: {error}"
                ))
            })?;
            Ok(StageReport {
                name: STAGE,
                files,
                bytes,
                note: format!(
                    "published {package_count} verified character \
                         packages as binary FBX 7.7 with external textures"
                ),
            })
        },
    );
    if result.is_err() {
        let _cleanup_result = std::fs::remove_dir_all(&staging);
    }
    result
}

/// Build every catalog entry below the already-created staging root.
fn build_catalog(
    index_path: &Path,
    staging: &Path,
    base_root: &Path,
) -> Result<(usize, usize, u64), PipelineError> {
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let mut packages = index
        .packages()
        .iter()
        .filter(|package| {
            package.category == "characters"
                && package.members().iter().any(|member| {
                    member.kind == "p3d-skin"
                        && member.source_chunk_kind == "skin"
                })
        })
        .collect::<Vec<_>>();
    packages.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    if packages.is_empty() {
        return Err(PipelineError::new("character catalog selection is empty"));
    }
    let next = AtomicUsize::new(0);
    let collected = Mutex::new(Vec::with_capacity(packages.len()));
    let workers = catalog_worker_count(packages.len());
    thread::scope(|scope| {
        for _worker in 0..workers {
            let _handle = scope.spawn(|| {
                loop {
                    let position = next.fetch_add(1, Ordering::Relaxed);
                    let Some(package) = packages.get(position) else {
                        break;
                    };
                    let result = export_prepared_character_package(
                        &index, package, staging, base_root,
                    )
                    .map_err(|error| {
                        PipelineError::new(format!(
                            "character catalog package {} failed: \
                                         {error}",
                            package.package_id
                        ))
                    });
                    collected
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .push((package.package_id.clone(), result));
                }
            });
        }
    });
    let mut package_results = collected
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if package_results.len() != packages.len() {
        return Err(PipelineError::new(format!(
            "character catalog workers returned {} of {} packages",
            package_results.len(),
            packages.len()
        )));
    }
    package_results.sort_by(|left, right| left.0.cmp(&right.0));
    let entries = package_results
        .into_iter()
        .map(|(_package_id, result)| result)
        .collect::<Result<Vec<_>, _>>()?;
    let input_root = staging.join(".texture-inputs");
    if input_root.exists() {
        std::fs::remove_dir_all(&input_root).map_err(|error| {
            PipelineError::new(format!("catalog input cleanup failed: {error}"))
        })?;
    }
    let manifest = json!({
        "schema_version": 1_i32,
        "status": "complete",
        "package_count": entries.len(),
        "ordering": "package-id-ascending",
        "fbx_version": 7_700_i32,
        "texture_storage": "external",
        "packed_images": 0_i32,
        "entries": entries,
    });
    let mut bytes = serde_json::to_vec_pretty(&manifest).map_err(|error| {
        PipelineError::new(format!("catalog JSON failed: {error}"))
    })?;
    bytes.push(b'\n');
    write_bytes(&staging.join("catalog.json"), &bytes, false).map_err(
        |error| PipelineError::new(format!("catalog write failed: {error}")),
    )?;
    let (files, total_bytes) = tree_totals(staging)?;
    Ok((packages.len(), files, total_bytes))
}

/// Bound parallel package work to two-thirds of logical processors.
fn catalog_worker_count(package_count: usize) -> usize {
    let available =
        thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    catalog_worker_count_for(available, package_count)
}

/// Calculate the stable two-thirds worker limit for one machine and catalog.
fn catalog_worker_count_for(available: usize, package_count: usize) -> usize {
    available
        .saturating_mul(2)
        .checked_div(3)
        .unwrap_or(1)
        .max(1)
        .min(package_count.max(1))
}

/// Return deterministic recursive file and byte totals for one catalog tree.
fn tree_totals(root: &Path) -> Result<(usize, u64), PipelineError> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = 0_usize;
    let mut bytes = 0_u64;
    while let Some(directory) = pending.pop() {
        let mut entries = std::fs::read_dir(&directory)
            .map_err(|error| {
                PipelineError::new(format!(
                    "catalog total traversal failed: {error}"
                ))
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                PipelineError::new(format!(
                    "catalog total entry failed: {error}"
                ))
            })?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let path = entry.path();
            let metadata = entry.metadata().map_err(|error| {
                PipelineError::new(format!("catalog metadata failed: {error}"))
            })?;
            if metadata.is_dir() {
                pending.push(path);
            } else if metadata.is_file() {
                files = files.checked_add(1).ok_or_else(|| {
                    PipelineError::new("catalog file overflow")
                })?;
                bytes = bytes.checked_add(metadata.len()).ok_or_else(|| {
                    PipelineError::new("catalog byte overflow")
                })?;
            } else {
                return Err(PipelineError::new(format!(
                    "catalog contains unsupported entry: {}",
                    path.display()
                )));
            }
        }
    }
    Ok((files, bytes))
}

/// Require one path to be missing before transactional publication.
fn ensure_missing(path: &Path, role: &str) -> Result<(), PipelineError> {
    match path_kind(path).map_err(|error| {
        PipelineError::new(format!("{role} check failed: {error}"))
    })? {
        PathKind::Missing => Ok(()),
        kind => Err(PipelineError::new(format!(
            "{role} already exists as {kind:?}"
        ))),
    }
}

/// Derive one hidden sibling staging identity from the output leaf.
fn staging_path(output: &Path) -> Result<PathBuf, PipelineError> {
    let name = output
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new("catalog output requires a UTF-8 leaf name")
        })?;
    Ok(output.with_file_name(format!(".{name}.catalog-staging")))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/character_catalog/tests.rs"]
mod tests;
