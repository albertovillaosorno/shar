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
//   - Exhaustive package-level FBX catalog publication.
// - Must-Not:
//   - Publish a partial catalog or invent FBX package evidence.
// - Allows:
//   - Reuse package writers, hash artifacts, verify, then rename root.
// - Split-When:
//   - Split when catalog scheduling gains an independent lifecycle.
// - Merge-When:
//   - Merge when another adapter owns the identical publication transaction.
// - Summary:
//   - Complete verified FBX catalog publisher.
// - Description:
//   - Publishes only after every direct FBX package verifies physically.
// - Usage:
//   - Invoked explicitly before prepare-unreal can promote FBX plans to ready.
// - Defaults:
//   - Existing output, empty selection, partial conversion, or stale read-back
//     fails without replacing the accepted catalog root.
//

//! Complete verified package-level FBX catalog publisher.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::json;
use shar_sha256::digest_hex;

use super::fbx_export::export_catalog_package;
use super::unreal_fbx_catalog::{
    CATALOG_SCHEMA, FBX_VERSION, verified_fbx_catalog,
    verified_fbx_catalog_at,
};
use crate::domain::package::{
    FbxTargetKind, PhaseThreePackageIndex, PhaseThreePackagePlanner,
    PhaseThreePackageRow,
};
use crate::domain::{PipelineError, StageReport};

const STAGE: &str = "fbx-export-catalog";
const CATALOG_FILE: &str = "catalog.jsonl";

/// Export every current direct FBX package and atomically publish catalog v2.
///
/// # Errors
///
/// Returns an error before publication when any package, artifact, catalog row,
/// root transaction, or read-back verification is incomplete or inconsistent.
pub(super) fn export_complete_fbx_catalog(
    index_path: &Path,
    output_root: &Path,
    manifest_path: &Path,
    base_root: &Path,
) -> Result<StageReport, PipelineError> {
    ensure_missing(output_root, "complete FBX catalog output")?;
    ensure_missing(manifest_path, "complete FBX catalog manifest")?;
    ensure_manifest_parent(manifest_path)?;
    let staging = staging_path(output_root)?;
    let manifest_staging = manifest_staging_path(manifest_path)?;
    ensure_missing(&staging, "complete FBX catalog staging")?;
    ensure_missing(&manifest_staging, "complete FBX manifest staging")?;
    fs::create_dir_all(staging.join("packages")).map_err(|error| {
        public_io_error("create complete FBX catalog staging", &error)
    })?;
    let built = build_catalog(index_path, &staging, base_root);
    let (package_count, file_count, byte_count, staged_evidence) = match built {
        Ok(value) => value,
        Err(error) => {
            let _cleanup = cleanup_directory(&staging);
            return Err(error);
        },
    };
    let staged_manifest = staging.join(CATALOG_FILE);
    if let Err(error) = stage_external_manifest(
        &staged_manifest,
        &manifest_staging,
    ) {
        let _cleanup = cleanup_directory(&staging);
        let _manifest_cleanup = cleanup_file(&manifest_staging);
        return Err(error);
    }
    if let Err(error) = fs::remove_file(&staged_manifest) {
        let _cleanup = cleanup_directory(&staging);
        let _manifest_cleanup = cleanup_file(&manifest_staging);
        return Err(public_io_error("detach complete FBX manifest", &error));
    }
    if let Err(error) = fs::rename(&staging, output_root) {
        let _cleanup = cleanup_directory(&staging);
        let _manifest_cleanup = cleanup_file(&manifest_staging);
        return Err(public_io_error("publish complete FBX catalog", &error));
    }
    if let Err(error) = fs::rename(&manifest_staging, manifest_path) {
        let cleanup = cleanup_directory(output_root);
        let _manifest_cleanup = cleanup_file(&manifest_staging);
        if let Err(cleanup_error) = cleanup {
            return Err(PipelineError::new(format!(
                "{}; failed to roll back FBX artifacts: {cleanup_error}",
                public_io_error("publish complete FBX manifest", &error)
            )));
        }
        return Err(public_io_error("publish complete FBX manifest", &error));
    }
    let readback = verify_published_catalog(
        output_root,
        manifest_path,
        &staged_evidence,
    );
    if let Err(error) = readback {
        let artifact_cleanup = cleanup_directory(output_root);
        let manifest_cleanup = cleanup_file(manifest_path);
        if let Err(cleanup_error) = artifact_cleanup {
            let cleanup_message = format!(
                "failed to remove rejected FBX artifacts: {cleanup_error}"
            );
            return Err(PipelineError::new(format!(
                "{error}; {cleanup_message}"
            )));
        }
        if let Err(cleanup_error) = manifest_cleanup {
            let cleanup_message = format!(
                "failed to remove rejected FBX manifest: {cleanup_error}"
            );
            return Err(PipelineError::new(format!(
                "{error}; {cleanup_message}"
            )));
        }
        return Err(error);
    }
    Ok(StageReport {
        name: STAGE,
        files: file_count,
        bytes: byte_count,
        note: format!(
            "published {package_count} verified package-level FBX assets"
        ),
    })
}

fn verify_published_catalog(
    output_root: &Path,
    manifest_path: &Path,
    staged_evidence: &[crate::domain::UnrealFbxArtifactEvidence],
) -> Result<(), PipelineError> {
    let published = verified_fbx_catalog_at(output_root, manifest_path)?
        .ok_or_else(|| {
            PipelineError::new(
                "published complete FBX catalog disappeared during read-back",
            )
        })?;
    if published != staged_evidence {
        return Err(PipelineError::new(
            "published complete FBX catalog changed during read-back",
        ));
    }
    Ok(())
}

fn build_catalog(
    index_path: &Path,
    staging: &Path,
    base_root: &Path,
) -> Result<(
    usize,
    usize,
    u64,
    Vec<crate::domain::UnrealFbxArtifactEvidence>,
), PipelineError> {
    let index = PhaseThreePackageIndex::read_for_unreal(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let mut packages = index
        .packages()
        .iter()
        .filter(|package| is_direct_fbx_package(package))
        .collect::<Vec<_>>();
    packages.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    if packages.is_empty() {
        return Err(PipelineError::new(
            "complete FBX catalog selection is empty",
        ));
    }
    let packages_root = staging.join("packages");
    for package in &packages {
        export_catalog_package(&index, package, &packages_root, base_root)
            .map_err(|error| {
                PipelineError::new(format!(
                    "complete FBX catalog package {} failed: {error}",
                    package.package_id
                ))
            })?;
    }
    let rows = catalog_rows(&packages, staging)?;
    write_catalog(staging, packages.len(), &rows)?;
    let evidence = verified_fbx_catalog(staging)?.ok_or_else(|| {
        PipelineError::new("complete FBX catalog staging verification vanished")
    })?;
    if evidence.len() != packages.len() {
        return Err(PipelineError::new(
            "complete FBX catalog verification returned a partial package set",
        ));
    }
    let (files, bytes) = tree_totals(staging)?;
    Ok((packages.len(), files, bytes, evidence))
}

fn is_direct_fbx_package(package: &PhaseThreePackageRow) -> bool {
    PhaseThreePackagePlanner::plan(package)
        .fbx
        .is_some_and(|fbx| {
            matches!(
                fbx.target_kind,
                FbxTargetKind::StaticMesh | FbxTargetKind::SkeletalMesh
            )
        })
}

fn catalog_rows(
    packages: &[&PhaseThreePackageRow],
    staging: &Path,
) -> Result<Vec<serde_json::Value>, PipelineError> {
    let mut rows = Vec::new();
    for package in packages {
        let package_name = package.package_id.replace('-', "_");
        let package_relative = format!("packages/{package_name}");
        let package_dir = staging.join(&package_relative);
        let fbx_relative = format!("{package_relative}/{package_name}.fbx");
        rows.push(fbx_row(
            &package.package_id,
            &fbx_relative,
            &staging.join(&fbx_relative),
        )?);
        let textures = package_dir.join("textures");
        match fs::read_dir(&textures) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
            Err(error) => {
                return Err(public_io_error("read catalog textures", &error));
            },
            Ok(entries) => {
                let mut files = entries
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| {
                        public_io_error("read catalog texture entry", &error)
                    })?;
                files.sort_by_key(fs::DirEntry::file_name);
                for entry in files {
                    let metadata = entry.metadata().map_err(|error| {
                        public_io_error("stat catalog texture", &error)
                    })?;
                    if !metadata.is_file() {
                        return Err(PipelineError::new(
                            "catalog texture inventory contains a non-file",
                        ));
                    }
                                        // jig-ignore-next-line: expression
                                        let file_name = entry.file_name().into_string().map_err(|_name| {
                        PipelineError::new(
                            "catalog texture name is not portable Unicode",
                        )
                    })?;
                    if Path::new(&file_name)
                        .extension()
                        .is_none_or(|extension| extension != "png")
                    {
                        return Err(PipelineError::new(
                            "catalog texture inventory contains a non-PNG file",
                        ));
                    }
                    let relative =
                        format!("{package_relative}/textures/{file_name}");
                    rows.push(texture_row(
                        &package.package_id,
                        &relative,
                        &entry.path(),
                    )?);
                }
            },
        }
    }
    Ok(rows)
}

fn fbx_row(
    package_id: &str,
    relative_path: &str,
    path: &Path,
) -> Result<serde_json::Value, PipelineError> {
    let bytes = read_artifact(path, "read catalog FBX")?;
    Ok(json!({
        "schema": CATALOG_SCHEMA,
        "record_type": "fbx",
        "package_id": package_id,
        "path": relative_path,
        "size_bytes": bytes.len(),
        "sha256": digest_hex(&bytes),
        "fbx_version": FBX_VERSION,
    }))
}

fn texture_row(
    package_id: &str,
    relative_path: &str,
    path: &Path,
) -> Result<serde_json::Value, PipelineError> {
    let bytes = read_artifact(path, "read catalog texture")?;
    Ok(json!({
        "schema": CATALOG_SCHEMA,
        "record_type": "texture",
        "package_id": package_id,
        "path": relative_path,
        "size_bytes": bytes.len(),
        "sha256": digest_hex(&bytes),
    }))
}

fn write_catalog(
    staging: &Path,
    package_count: usize,
    rows: &[serde_json::Value],
) -> Result<(), PipelineError> {
    let mut lines = Vec::with_capacity(rows.len().saturating_add(1));
    lines.push(json!({
        "schema": CATALOG_SCHEMA,
        "record_type": "header",
        "status": "complete",
        "package_count": package_count,
        "file_count": rows.len(),
    }));
    lines.extend(rows.iter().cloned());
    let mut rendered = String::new();
    for row in lines {
        rendered.push_str(&serde_json::to_string(&row).map_err(|error| {
            PipelineError::new(format!(
                "catalog JSON serialization failed: {error}"
            ))
        })?);
        rendered.push('\n');
    }
    fs::write(staging.join(CATALOG_FILE), rendered.as_bytes())
        .map_err(|error| public_io_error("write complete FBX catalog", &error))
}

fn read_artifact(path: &Path, action: &str) -> Result<Vec<u8>, PipelineError> {
    fs::read(path).map_err(|error| public_io_error(action, &error))
}

fn ensure_manifest_parent(manifest_path: &Path) -> Result<(), PipelineError> {
    let parent = manifest_path.parent().ok_or_else(|| {
        PipelineError::new("complete FBX manifest has no parent directory")
    })?;
    let metadata = fs::symlink_metadata(parent).map_err(|error| {
        public_io_error("inspect complete FBX manifest directory", &error)
    })?;
    if !metadata.is_dir() {
        return Err(PipelineError::new(
            "complete FBX manifest parent is not a directory",
        ));
    }
    Ok(())
}

fn manifest_staging_path(
    manifest_path: &Path,
) -> Result<PathBuf, PipelineError> {
    let name = manifest_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            PipelineError::new("complete FBX manifest has no portable name")
        })?;
    let parent = manifest_path.parent().ok_or_else(|| {
        PipelineError::new("complete FBX manifest has no parent directory")
    })?;
    Ok(parent.join(format!(".{name}.complete-staging")))
}

fn stage_external_manifest(
    source: &Path,
    destination: &Path,
) -> Result<(), PipelineError> {
    let bytes = fs::read(source)
        .map_err(|error| public_io_error("read staged FBX manifest", &error))?;
    fs::write(destination, bytes)
        .map_err(|error| public_io_error("stage complete FBX manifest", &error))
}

fn staging_path(output_root: &Path) -> Result<PathBuf, PipelineError> {
        // jig-ignore-next-line: expression
        let name = output_root.file_name().and_then(|name| name.to_str()).ok_or_else(
        // jig-ignore-next-line: literal
        || PipelineError::new("complete FBX catalog output has no portable name"),
    )?;
    let parent = output_root.parent().unwrap_or_else(|| Path::new("."));
    Ok(parent.join(format!(".{name}.complete-staging")))
}

fn ensure_missing(path: &Path, label: &str) -> Result<(), PipelineError> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        // jig-ignore-next-line: literal
        Err(error) => Err(public_io_error("inspect catalog transaction", &error)),
        // jig-ignore-next-line: literal
        Ok(_metadata) => Err(PipelineError::new(format!("{label} already exists"))),
    }
}

fn cleanup_directory(path: &Path) -> Result<(), PipelineError> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(public_io_error("inspect catalog cleanup", &error)),
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(path)
            .map_err(|error| public_io_error("clean catalog staging", &error)),
        Ok(_metadata) => Err(PipelineError::new(
            "complete FBX catalog staging changed file kind",
        )),
    }
}

fn cleanup_file(path: &Path) -> Result<(), PipelineError> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(public_io_error("inspect manifest cleanup", &error)),
        Ok(metadata) if metadata.is_file() => fs::remove_file(path)
            .map_err(|error| public_io_error("clean FBX manifest", &error)),
        Ok(_metadata) => Err(PipelineError::new(
            "complete FBX manifest staging changed file kind",
        )),
    }
}

fn tree_totals(root: &Path) -> Result<(usize, u64), PipelineError> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = 0_usize;
    let mut bytes = 0_u64;
    while let Some(directory) = pending.pop() {
        let mut entries = fs::read_dir(&directory)
            .map_err(|error| {
                public_io_error("traverse complete FBX catalog", &error)
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                public_io_error("read complete FBX catalog entry", &error)
            })?;
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            let metadata = entry.metadata().map_err(|error| {
                public_io_error("stat complete FBX catalog entry", &error)
            })?;
            if metadata.is_dir() {
                pending.push(entry.path());
            } else if metadata.is_file() {
                files = files.checked_add(1).ok_or_else(|| {
                                        // jig-ignore-next-line: literal
                    PipelineError::new("complete FBX catalog file count overflowed")
                })?;
                bytes = bytes.checked_add(metadata.len()).ok_or_else(|| {
                                        // jig-ignore-next-line: literal
                    PipelineError::new("complete FBX catalog byte count overflowed")
                })?;
            } else {
                return Err(PipelineError::new(
                    "complete FBX catalog contains a special file",
                ));
            }
        }
    }
    Ok((files, bytes))
}

fn public_io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed ({:?})", error.kind()))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/fbx_catalog_publish/tests.rs"]
mod tests;
