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
//   - Transactional publication of joined Scrooby sprite rasters.
// - Must-Not:
//   - Re-infer authored image identities or feed Unreal planning implicitly.
// - Allows:
//   - Compile exact package/entity targets supplied by Scrooby preflight.
// - Split-When:
//   - Joined raster evidence becomes an Unreal construction/import contract.
// - Merge-When:
//   - Another publisher owns the identical package/entity raster lifecycle.
// - Summary:
//   - Publish deterministic rasters for joined Scrooby sprite entities.
// - Description:
//   - Keys every raster by normalized package id plus sprite ordinal.
// - Usage:
//   - Used by prepare-unreal after Scrooby semantic preflight.
// - Defaults:
//   - Missing, changed, extra, or interrupted output fails explicitly.
//

//! Joined Scrooby sprite raster catalog publisher.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::semantic_texture_png::decode_png_bytes;
use serde_json::json;
use shar_sha256::digest_hex;

use crate::domain::{PhaseThreePackageIndex, PipelineError, PipelineOutcome};

use super::ui_scrooby_project::ScroobyUiPreflight;
use super::ui_sprite_raster::{
    CompiledScroobyJoinedRaster, compile_scrooby_joined_sprite_raster,
};

const SCHEMA: &str = "shar-schoenwald.scrooby-joined-raster-catalog.v1";
const CATALOG_FILE: &str = "catalog.jsonl";
const RASTER_DIR: &str = "rasters";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ScroobyJoinedRasterSummary {
    pub(super) package_count: usize,
    pub(super) raster_count: usize,
    pub(super) tile_count: usize,
    pub(super) total_bytes: u64,
}

/// Compile and atomically publish every joined Scrooby sprite target.
///
/// # Errors
///
/// Returns an error when one target package/entity is absent, compilation
/// fails, transaction debris exists, or staged/published read-back changes.
pub(super) fn publish_scrooby_joined_raster_catalog(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
    preflight: &ScroobyUiPreflight,
    output_root: &Path,
) -> PipelineOutcome<ScroobyJoinedRasterSummary> {
    let packages = index
        .packages()
        .iter()
        .map(|package| (package.package_id.as_str(), package))
        .collect::<BTreeMap<_, _>>();
    let mut compiled = Vec::new();
    for target in preflight.joined_sprite_targets() {
        let package = packages.get(target.package_id.as_str()).ok_or_else(|| {
            PipelineError::new(
                "joined Scrooby raster target package is absent from index",
            )
        })?;
        compiled.push(compile_scrooby_joined_sprite_raster(
            package,
            extracted_root,
            target.sprite_ordinal,
        )?);
    }
    compiled.sort_by(|left, right| {
        (left.package_id.as_str(), left.sprite_ordinal)
            .cmp(&(right.package_id.as_str(), right.sprite_ordinal))
    });
    let summary = summarize(&compiled)?;
    let rendered = render_catalog(&compiled, summary)?;
    publish_catalog(output_root, &compiled, &rendered)?;
    Ok(summary)
}

fn summarize(
    compiled: &[CompiledScroobyJoinedRaster],
) -> PipelineOutcome<ScroobyJoinedRasterSummary> {
    let package_count = compiled
        .iter()
        .map(|artifact| artifact.package_id.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let tile_count = compiled.iter().try_fold(0usize, |total, artifact| {
        total.checked_add(artifact.tile_count).ok_or_else(|| {
            PipelineError::new("joined Scrooby raster tile count overflowed")
        })
    })?;
    let total_bytes = compiled.iter().try_fold(0u64, |total, artifact| {
        let size = u64::try_from(artifact.png_bytes.len()).map_err(|error| {
            PipelineError::new(format!(
                "joined Scrooby raster size conversion failed: {error}"
            ))
        })?;
        total.checked_add(size).ok_or_else(|| {
            PipelineError::new("joined Scrooby raster byte count overflowed")
        })
    })?;
    Ok(ScroobyJoinedRasterSummary {
        package_count,
        raster_count: compiled.len(),
        tile_count,
        total_bytes,
    })
}

fn render_catalog(
    compiled: &[CompiledScroobyJoinedRaster],
    summary: ScroobyJoinedRasterSummary,
) -> PipelineOutcome<String> {
    let mut output = String::new();
    push_json_line(
        &mut output,
        &json!({
            "schema": SCHEMA,
            "record_type": "header",
            "status": "complete",
            "package_count": summary.package_count,
            "raster_count": summary.raster_count,
            "tile_count": summary.tile_count,
            "total_bytes": summary.total_bytes,
        }),
    )?;
    for artifact in compiled {
        push_json_line(
            &mut output,
            &json!({
                "schema": SCHEMA,
                "record_type": "raster",
                "package_id": artifact.package_id,
                "sprite_ordinal": artifact.sprite_ordinal,
                "path": format!("{RASTER_DIR}/{}", artifact.filename),
                "size_bytes": artifact.png_bytes.len(),
                "sha256": artifact.png_sha256,
                "source_revision": artifact.source_revision,
                "width": artifact.width,
                "height": artifact.height,
                "tile_count": artifact.tile_count,
            }),
        )?;
    }
    Ok(output)
}

fn push_json_line(
    output: &mut String,
    value: &serde_json::Value,
) -> PipelineOutcome<()> {
    writeln!(
        output,
        "{}",
        serde_json::to_string(&value).map_err(|error| {
            PipelineError::new(format!(
                "joined Scrooby raster JSON failed: {error}"
            ))
        })?,
    )
    .map_err(|_error| PipelineError::new("joined raster formatting failed"))
}

fn publish_catalog(
    output_root: &Path,
    compiled: &[CompiledScroobyJoinedRaster],
    rendered: &str,
) -> PipelineOutcome<()> {
    let (staging, backup) = transaction_paths(output_root)?;
    ensure_absent(&staging, "joined Scrooby raster staging")?;
    ensure_absent(&backup, "joined Scrooby raster backup")?;
    let had_output = match fs::symlink_metadata(output_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            return Err(io_error("inspect joined raster output", &error));
        },
        Ok(metadata) => {
            validate_directory(&metadata, "joined Scrooby raster output")?;
            true
        },
    };
    if had_output && verify_catalog(output_root, compiled, rendered).is_ok() {
        return Ok(());
    }
    let parent = output_root.parent().ok_or_else(|| {
        PipelineError::new("joined Scrooby raster output has no parent")
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        io_error("create joined raster output parent", &error)
    })?;
    let parent_metadata = fs::symlink_metadata(parent).map_err(|error| {
        io_error("inspect joined raster output parent", &error)
    })?;
    validate_directory(
        &parent_metadata,
        "joined Scrooby raster output parent",
    )?;
    fs::create_dir_all(staging.join(RASTER_DIR))
        .map_err(|error| io_error("create joined raster staging", &error))?;
    if let Err(error) = stage_catalog(&staging, compiled, rendered) {
        let _cleanup = remove_directory(&staging);
        return Err(error);
    }
    if let Err(error) = verify_catalog(&staging, compiled, rendered) {
        let _cleanup = remove_directory(&staging);
        return Err(error);
    }
    if had_output
        && let Err(error) = fs::rename(output_root, &backup)
    {
        let primary = io_error("back up joined raster output", &error);
        return Err(with_staging_cleanup(primary, &staging));
    }
    if let Err(error) = fs::rename(&staging, output_root) {
        let publish = io_error("publish joined Scrooby raster catalog", &error);
        let cleanup = remove_directory(&staging);
        if had_output
            && let Err(rollback) = fs::rename(&backup, output_root)
        {
            return match cleanup {
                Ok(()) => Err(PipelineError::new(format!(
                    "{publish}; restore joined raster backup failed ({:?})",
                    rollback.kind(),
                ))),
                Err(cleanup_error) => Err(PipelineError::new(format!(
                    concat!(
                        "{}; restore joined raster backup failed ({:?}); ",
                        "joined raster staging cleanup failed: {}",
                    ),
                    publish,
                    rollback.kind(),
                    cleanup_error,
                ))),
            };
        }
        return match cleanup {
            Ok(()) => Err(publish),
            Err(cleanup_error) => Err(PipelineError::new(format!(
                concat!(
                    "{}; joined raster staging cleanup failed: {}",
                ),
                publish,
                cleanup_error,
            ))),
        };
    }
    if let Err(error) = verify_catalog(output_root, compiled, rendered) {
        remove_directory(output_root)?;
        if had_output {
            fs::rename(&backup, output_root).map_err(|rollback| {
                PipelineError::new(format!(
                    "{error}; restore joined raster backup failed ({:?})",
                    rollback.kind(),
                ))
            })?;
        }
        return Err(error);
    }
    if had_output {
        remove_directory(&backup)?;
    }
    Ok(())
}

fn stage_catalog(
    staging: &Path,
    compiled: &[CompiledScroobyJoinedRaster],
    rendered: &str,
) -> PipelineOutcome<()> {
    for artifact in compiled {
        fs::write(
            staging.join(RASTER_DIR).join(&artifact.filename),
            &artifact.png_bytes,
        )
        .map_err(|error| io_error("write joined Scrooby raster", &error))?;
    }
    fs::write(staging.join(CATALOG_FILE), rendered).map_err(|error| {
        io_error("write joined Scrooby raster catalog", &error)
    })
}

fn verify_catalog(
    root: &Path,
    compiled: &[CompiledScroobyJoinedRaster],
    rendered: &str,
) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(root)
        .map_err(|error| io_error("inspect joined raster root", &error))?;
    validate_directory(&metadata, "joined Scrooby raster root")?;
    let mut root_names = fs::read_dir(root)
        .map_err(|error| io_error("read joined raster root", &error))?
        .map(|entry| entry.map(|value| value.file_name()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| io_error("read joined raster root entry", &error))?;
    root_names.sort();
    let expected_root = [
        std::ffi::OsString::from(CATALOG_FILE),
        std::ffi::OsString::from(RASTER_DIR),
    ];
    if root_names.as_slice() != expected_root.as_slice() {
        return Err(PipelineError::new(
            "joined Scrooby raster root inventory is not exact",
        ));
    }
    let catalog_path = root.join(CATALOG_FILE);
    validate_regular_file(&catalog_path, "joined Scrooby raster catalog")?;
    let raster_root = root.join(RASTER_DIR);
    let raster_metadata = fs::symlink_metadata(&raster_root)
        .map_err(|error| io_error("inspect joined raster directory", &error))?;
    validate_directory(&raster_metadata, "joined Scrooby raster directory")?;
    let text = fs::read_to_string(&catalog_path)
        .map_err(|error| io_error("read joined raster catalog", &error))?;
    if text != rendered {
        return Err(PipelineError::new(
            "joined Scrooby raster catalog disagrees with compiled evidence",
        ));
    }
    let expected = compiled
        .iter()
        .map(|artifact| artifact.filename.as_str())
        .collect::<BTreeSet<_>>();
    let mut actual = BTreeSet::new();
    for entry in fs::read_dir(&raster_root)
        .map_err(|error| io_error("read joined raster directory", &error))?
    {
        let entry = entry.map_err(|error| {
            io_error("read joined raster entry", &error)
        })?;
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|error| io_error("inspect joined raster entry", &error))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(PipelineError::new(
                "joined Scrooby raster inventory contains a non-file",
            ));
        }
        let name = entry.file_name().into_string().map_err(|_name| {
            PipelineError::new("joined Scrooby raster filename is not Unicode")
        })?;
        let _inserted = actual.insert(name);
    }
    let expected_owned = expected
        .iter()
        .map(|name| (*name).to_owned())
        .collect::<BTreeSet<_>>();
    if actual != expected_owned {
        return Err(PipelineError::new(
            "joined Scrooby raster inventory is not exact",
        ));
    }
    for artifact in compiled {
        let bytes = fs::read(root.join(RASTER_DIR).join(&artifact.filename))
            .map_err(|error| io_error("read joined Scrooby raster", &error))?;
        if digest_hex(&bytes) != artifact.png_sha256
            || bytes != artifact.png_bytes
        {
            return Err(PipelineError::new(
                "joined Scrooby raster bytes disagree with compiled evidence",
            ));
        }
        let decoded = decode_png_bytes(&bytes).map_err(|error| {
            PipelineError::new(format!(
                "joined Scrooby raster PNG verification failed: {error:?}"
            ))
        })?;
        if decoded.width() != artifact.width
            || decoded.height() != artifact.height
        {
            return Err(PipelineError::new(
                "joined Scrooby raster dimensions disagree with catalog",
            ));
        }
    }
    Ok(())
}

fn transaction_paths(
    output_root: &Path,
) -> PipelineOutcome<(PathBuf, PathBuf)> {
    let name = output_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new("joined Scrooby raster output has no name")
        })?;
    let parent = output_root.parent().unwrap_or_else(|| Path::new("."));
    Ok((
        parent.join(format!(".{name}.complete-staging")),
        parent.join(format!(".{name}.complete-backup")),
    ))
}

fn ensure_absent(path: &Path, label: &str) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => {
            Err(io_error("inspect joined raster transaction", &error))
        },
        Ok(_metadata) => {
            Err(PipelineError::new(format!("{label} already exists")))
        },
    }
}

fn validate_directory(
    metadata: &fs::Metadata,
    label: &str,
) -> PipelineOutcome<()> {
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(PipelineError::new(format!("{label} must be a real directory")))
    }
}

fn with_staging_cleanup(
    primary: PipelineError,
    staging: &Path,
) -> PipelineError {
    match remove_directory(staging) {
        Ok(()) => primary,
        Err(cleanup) => PipelineError::new(format!(
            "{primary}; joined raster staging cleanup failed: {cleanup}"
        )),
    }
}

fn validate_regular_file(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_error("inspect joined raster file", &error))?;
    if metadata.is_file() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(PipelineError::new(format!("{label} must be a real file")))
    }
}

fn remove_directory(path: &Path) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io_error("inspect joined raster cleanup", &error)),
        Ok(metadata) => {
            validate_directory(&metadata, "joined Scrooby raster cleanup")?;
            fs::remove_dir_all(path).map_err(|error| {
                io_error("remove joined raster directory", &error)
            })
        },
    }
}

fn io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed ({:?})", error.kind()))
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/ui_scrooby_joined_raster_tests.rs"]
mod tests;
