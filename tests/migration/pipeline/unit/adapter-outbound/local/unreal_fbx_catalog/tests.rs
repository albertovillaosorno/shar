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
//   - Generated FBX catalog adapter tests.
// - Must-Not:
//   - Read proprietary assets or contact Unreal Editor.
// - Allows:
//   - Synthetic binary FBX files and isolated filesystem fixtures.
// - Split-When:
//   - Split when catalog schemas gain independent test lifecycles.
// - Merge-When:
//   - Merge when another adapter test owns identical catalog evidence.
// - Summary:
//   - Generated FBX catalog adapter tests.
// - Description:
//   - Proves complete inventory, digest, version, and link validation.
// - Usage:
//   - Included only by the owning local adapter under cfg(test).
// - Defaults:
//   - Partial, stale, linked, or malformed catalogs fail closed.
//

//! Generated FBX catalog adapter tests.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use shar_sha256::digest_hex;

use super::{
    FBX_VERSION, verified_fbx_catalog, verified_fbx_catalog_at,
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);
const PACKAGE_ID: &str = "extracted-art-cars-model";
const RELATIVE_FBX: &str = concat!(
    "packages/extracted_art_cars_model/",
    "extracted_art_cars_model.fbx"
);
const RELATIVE_TEXTURE: &str =
    concat!("packages/extracted_art_cars_model/textures/", "paint.png");

struct TempRoot(PathBuf);

impl TempRoot {
    fn new(label: &str) -> Result<Self, String> {
        let sequence = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = PathBuf::from(".temp").join(format!(
            "unreal-fbx-catalog-{label}-{}-{sequence}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).map_err(|error| error.to_string())?;
        }
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _result = fs::remove_dir_all(&self.0);
    }
}

fn fbx_bytes(version: u32) -> Vec<u8> {
    let mut bytes = b"Kaydara FBX Binary  \0\x1a\0".to_vec();
    bytes.extend_from_slice(&version.to_le_bytes());
    bytes.extend_from_slice(b"verified-fixture");
    bytes
}

fn write_catalog(
    root: &Path,
    header_count: u64,
    row_path: &str,
    row_size: u64,
    row_digest: &str,
    row_version: u32,
    bytes: &[u8],
) -> Result<(), String> {
    let artifact = root.join(row_path);
    let parent = artifact
        .parent()
        .ok_or_else(|| "fixture has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(&artifact, bytes).map_err(|error| error.to_string())?;
    let catalog = format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.fbx-catalog.v2\",",
            "\"record_type\":\"header\",",
            "\"status\":\"complete\",",
            "\"package_count\":{},\"file_count\":1}}\n",
            "{{\"schema\":\"shar-schoenwald.fbx-catalog.v2\",",
            "\"record_type\":\"fbx\",",
            "\"package_id\":\"{}\",",
            "\"path\":\"{}\",",
            "\"size_bytes\":{},",
            "\"sha256\":\"{}\",",
            "\"fbx_version\":{}}}\n"
        ),
        header_count, PACKAGE_ID, row_path, row_size, row_digest, row_version,
    );
    fs::write(root.join("catalog.jsonl"), catalog)
        .map_err(|error| error.to_string())
}

fn png_bytes() -> Vec<u8> {
    let mut bytes = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
    bytes.extend_from_slice(b"verified-texture-fixture");
    bytes
}

fn append_texture_record(
    root: &Path,
    package_id: &str,
    row_path: &str,
    row_size: u64,
    row_digest: &str,
    bytes: &[u8],
) -> Result<(), String> {
    let artifact = root.join(row_path);
    let parent = artifact
        .parent()
        .ok_or_else(|| "texture fixture has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(&artifact, bytes).map_err(|error| error.to_string())?;

    let catalog_path = root.join("catalog.jsonl");
    let mut catalog =
        fs::read_to_string(&catalog_path).map_err(|error| error.to_string())?;
    catalog = catalog.replacen("\"file_count\":1", "\"file_count\":2", 1);
    write!(
        &mut catalog,
        concat!(
            "{{\"schema\":\"shar-schoenwald.fbx-catalog.v2\",",
            "\"record_type\":\"texture\",",
            "\"package_id\":\"{}\",",
            "\"path\":\"{}\",",
            "\"size_bytes\":{},",
            "\"sha256\":\"{}\"}}\n"
        ),
        package_id, row_path, row_size, row_digest,
    )
    .map_err(|error| error.to_string())?;
    fs::write(catalog_path, catalog).map_err(|error| error.to_string())
}

fn write_valid_catalog(root: &Path) -> Result<Vec<u8>, String> {
    let bytes = fbx_bytes(FBX_VERSION);
    write_catalog(
        root,
        1,
        RELATIVE_FBX,
        u64::try_from(bytes.len()).unwrap_or(u64::MAX),
        &digest_hex(&bytes),
        FBX_VERSION,
        &bytes,
    )?;
    Ok(bytes)
}

#[test]
fn absent_catalog_keeps_fbx_conversion_pending() -> Result<(), String> {
    let root = TempRoot::new("absent")?;
    if verified_fbx_catalog(root.path())
        .map_err(|error| error.to_string())?
        .is_some()
    {
        return Err("absent generated FBX catalog produced evidence".to_owned());
    }
    Ok(())
}

#[test]
fn verifies_catalog_stored_outside_artifact_root() -> Result<(), String> {
    let root = TempRoot::new("external-manifest")?;
    let artifacts = root.path().join("artifacts");
    let _bytes = write_valid_catalog(&artifacts)?;
    let manifest_dir = root.path().join("manifest");
    fs::create_dir_all(&manifest_dir).map_err(|error| error.to_string())?;
    let manifest = manifest_dir.join("fbx.jsonl");
    fs::rename(artifacts.join("catalog.jsonl"), &manifest)
        .map_err(|error| error.to_string())?;
    let evidence = verified_fbx_catalog_at(&artifacts, &manifest)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "external FBX manifest was not returned".to_owned())?;
    if evidence.len() != 1 {
        return Err(
            "external FBX manifest returned partial evidence".to_owned()
        );
    }
    Ok(())
}

#[test]
fn verifies_complete_binary_fbx_catalog() -> Result<(), String> {
    let root = TempRoot::new("complete")?;
    let bytes = write_valid_catalog(root.path())?;
    let evidence = verified_fbx_catalog(root.path())
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "complete catalog was not returned".to_owned())?;
    let artifact = evidence
        .first()
        .ok_or_else(|| "complete catalog returned no evidence".to_owned())?;
    if evidence.len() != 1
        || artifact.package_id != PACKAGE_ID
        || artifact.path != format!("fbx-assets/{RELATIVE_FBX}")
        || artifact.size_bytes != u64::try_from(bytes.len()).unwrap_or(u64::MAX)
        || artifact.sha256 != digest_hex(&bytes)
        || artifact.fbx_version != FBX_VERSION
    {
        return Err("verified FBX evidence does not match the bytes".to_owned());
    }
    Ok(())
}

#[test]
fn verifies_external_texture_provenance_without_promoting_it()
-> Result<(), String> {
    let root = TempRoot::new("external-texture")?;
    let fbx = write_valid_catalog(root.path())?;
    let texture = png_bytes();
    append_texture_record(
        root.path(),
        PACKAGE_ID,
        RELATIVE_TEXTURE,
        u64::try_from(texture.len()).unwrap_or(u64::MAX),
        &digest_hex(&texture),
        &texture,
    )?;

    let evidence = verified_fbx_catalog(root.path())
        .map_err(|error| error.to_string())?
        .ok_or_else(|| {
            "catalog with texture provenance was not returned".to_owned()
        })?;
    let artifact = evidence
        .first()
        .ok_or_else(|| "texture catalog returned no FBX evidence".to_owned())?;
    if evidence.len() != 1
        || artifact.path != format!("fbx-assets/{RELATIVE_FBX}")
        || artifact.sha256 != digest_hex(&fbx)
    {
        return Err(
            "texture provenance changed promoted FBX evidence".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_stale_or_non_png_texture_evidence() -> Result<(), String> {
    for mutation in ["size", "digest", "magic"] {
        let root = TempRoot::new(mutation)?;
        let _fbx = write_valid_catalog(root.path())?;
        let texture = if mutation == "magic" {
            b"not-a-png-texture".to_vec()
        } else {
            png_bytes()
        };
        let size = if mutation == "size" {
            u64::try_from(texture.len())
                .unwrap_or(u64::MAX)
                .saturating_add(1)
        } else {
            u64::try_from(texture.len()).unwrap_or(u64::MAX)
        };
        let digest = if mutation == "digest" {
            "0".repeat(64)
        } else {
            digest_hex(&texture)
        };
        append_texture_record(
            root.path(),
            PACKAGE_ID,
            RELATIVE_TEXTURE,
            size,
            &digest,
            &texture,
        )?;
        if verified_fbx_catalog(root.path()).is_ok() {
            return Err(format!(
                "invalid FBX texture evidence was accepted: {mutation}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_texture_path_or_owner_drift() -> Result<(), String> {
    let wrong_path = TempRoot::new("texture-path")?;
    let _fbx = write_valid_catalog(wrong_path.path())?;
    let texture = png_bytes();
    append_texture_record(
        wrong_path.path(),
        PACKAGE_ID,
        "packages/other/textures/paint.png",
        u64::try_from(texture.len()).unwrap_or(u64::MAX),
        &digest_hex(&texture),
        &texture,
    )?;
    if verified_fbx_catalog(wrong_path.path()).is_ok() {
        return Err("texture outside its package was accepted".to_owned());
    }

    let orphan = TempRoot::new("texture-owner")?;
    let _fbx = write_valid_catalog(orphan.path())?;
    append_texture_record(
        orphan.path(),
        "orphan-package",
        "packages/orphan_package/textures/paint.png",
        u64::try_from(texture.len()).unwrap_or(u64::MAX),
        &digest_hex(&texture),
        &texture,
    )?;
    if verified_fbx_catalog(orphan.path()).is_ok() {
        return Err(
            "texture without an FBX package owner was accepted".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_partial_and_inexact_catalog_inventory() -> Result<(), String> {
    let partial = TempRoot::new("partial")?;
    let bytes = fbx_bytes(FBX_VERSION);
    write_catalog(
        partial.path(),
        2,
        RELATIVE_FBX,
        u64::try_from(bytes.len()).unwrap_or(u64::MAX),
        &digest_hex(&bytes),
        FBX_VERSION,
        &bytes,
    )?;
    if verified_fbx_catalog(partial.path()).is_ok() {
        return Err("partial FBX catalog was accepted".to_owned());
    }

    let extra = TempRoot::new("extra")?;
    let _bytes = write_valid_catalog(extra.path())?;
    fs::write(extra.path().join("unexpected.txt"), b"unexpected")
        .map_err(|error| error.to_string())?;
    if verified_fbx_catalog(extra.path()).is_ok() {
        return Err("unexpected FBX catalog file was accepted".to_owned());
    }

    let stale_files = TempRoot::new("stale-file-count")?;
    let _bytes = write_valid_catalog(stale_files.path())?;
    let catalog_path = stale_files.path().join("catalog.jsonl");
    let catalog = fs::read_to_string(&catalog_path)
        .map_err(|error| error.to_string())?
        .replacen("\"file_count\":1", "\"file_count\":2", 1);
    fs::write(&catalog_path, catalog).map_err(|error| error.to_string())?;
    if verified_fbx_catalog(stale_files.path()).is_ok() {
        return Err("stale FBX catalog file count was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_stale_fbx_size_digest_and_version() -> Result<(), String> {
    for mutation in ["size", "digest", "catalog-version", "binary-version"] {
        let root = TempRoot::new(mutation)?;
        let binary_version = if mutation == "binary-version" {
            7400
        } else {
            FBX_VERSION
        };
        let bytes = fbx_bytes(binary_version);
        let size = if mutation == "size" {
            u64::try_from(bytes.len())
                .unwrap_or(u64::MAX)
                .saturating_add(1)
        } else {
            u64::try_from(bytes.len()).unwrap_or(u64::MAX)
        };
        let digest = if mutation == "digest" {
            "0".repeat(64)
        } else {
            digest_hex(&bytes)
        };
        let catalog_version = if mutation == "catalog-version" {
            7400
        } else {
            FBX_VERSION
        };
        write_catalog(
            root.path(),
            1,
            RELATIVE_FBX,
            size,
            &digest,
            catalog_version,
            &bytes,
        )?;
        if verified_fbx_catalog(root.path()).is_ok() {
            return Err(format!("stale FBX {mutation} was accepted"));
        }
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_catalog_path_and_fields() -> Result<(), String> {
    let unsafe_path = TempRoot::new("unsafe-path")?;
    let bytes = fbx_bytes(FBX_VERSION);
    write_catalog(
        unsafe_path.path(),
        1,
        "packages/other/other.fbx",
        u64::try_from(bytes.len()).unwrap_or(u64::MAX),
        &digest_hex(&bytes),
        FBX_VERSION,
        &bytes,
    )?;
    if verified_fbx_catalog(unsafe_path.path()).is_ok() {
        return Err("noncanonical FBX catalog path was accepted".to_owned());
    }

    let unknown = TempRoot::new("unknown-field")?;
    let _bytes = write_valid_catalog(unknown.path())?;
    let catalog_path = unknown.path().join("catalog.jsonl");
    let catalog = fs::read_to_string(&catalog_path)
        .map_err(|error| error.to_string())?
        .replacen(
            "\"status\":\"complete\"",
            "\"status\":\"complete\",\"unexpected\":true",
            1,
        );
    fs::write(&catalog_path, catalog).map_err(|error| error.to_string())?;
    if verified_fbx_catalog(unknown.path()).is_ok() {
        return Err("unknown FBX catalog field was accepted".to_owned());
    }

    let old_schema = TempRoot::new("old-schema")?;
    let _bytes = write_valid_catalog(old_schema.path())?;
    let catalog_path = old_schema.path().join("catalog.jsonl");
    let catalog = fs::read_to_string(&catalog_path)
        .map_err(|error| error.to_string())?
        .replace(
            "shar-schoenwald.fbx-catalog.v2",
            "shar-schoenwald.fbx-catalog.v1",
        );
    fs::write(&catalog_path, catalog).map_err(|error| error.to_string())?;
    if verified_fbx_catalog(old_schema.path()).is_ok() {
        return Err("stale FBX catalog schema was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_jsonl_line_endings() -> Result<(), String> {
    for mutation in ["missing-final-lf", "crlf"] {
        let root = TempRoot::new(mutation)?;
        let _bytes = write_valid_catalog(root.path())?;
        let catalog_path = root.path().join("catalog.jsonl");
        let mut catalog = fs::read_to_string(&catalog_path)
            .map_err(|error| error.to_string())?;
        if mutation == "missing-final-lf" {
            let _removed = catalog.pop();
        } else {
            catalog = catalog.replace('\n', "\r\n");
        }
        fs::write(&catalog_path, catalog).map_err(|error| error.to_string())?;
        if verified_fbx_catalog(root.path()).is_ok() {
            return Err(format!(
                "noncanonical JSONL mutation was accepted: {mutation}"
            ));
        }
    }
    Ok(())
}

#[test]
fn diagnostics_do_not_expose_catalog_root() -> Result<(), String> {
    let root = TempRoot::new("private-diagnostic")?;
    let _bytes = write_valid_catalog(root.path())?;
    fs::write(root.path().join("unexpected.txt"), b"unexpected")
        .map_err(|error| error.to_string())?;
    let Err(error) = verified_fbx_catalog(root.path()) else {
        return Err("unexpected inventory should fail".to_owned());
    };
    let error = error.to_string();
    let root_text = root.path().to_string_lossy();
    if error.contains(root_text.as_ref()) {
        return Err("FBX diagnostic exposed its physical root".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_symbolic_linked_fbx_artifacts() -> Result<(), String> {
    let root = TempRoot::new("symlink")?;
    let bytes = fbx_bytes(FBX_VERSION);
    let target = root.path().join("target.fbx");
    fs::create_dir_all(root.path()).map_err(|error| error.to_string())?;
    fs::write(&target, &bytes).map_err(|error| error.to_string())?;
    let artifact = root.path().join(RELATIVE_FBX);
    let parent = artifact
        .parent()
        .ok_or_else(|| "symlink fixture has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    create_file_link(&target, &artifact)?;
    let catalog = format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.fbx-catalog.v2\",",
            "\"record_type\":\"header\",",
            "\"status\":\"complete\",\"package_count\":1,\"file_count\":1}}\n",
            "{{\"schema\":\"shar-schoenwald.fbx-catalog.v2\",",
            "\"record_type\":\"fbx\",\"package_id\":\"{}\",",
            "\"path\":\"{}\",\"size_bytes\":{},",
            "\"sha256\":\"{}\",\"fbx_version\":{}}}\n"
        ),
        PACKAGE_ID,
        RELATIVE_FBX,
        bytes.len(),
        digest_hex(&bytes),
        FBX_VERSION,
    );
    fs::write(root.path().join("catalog.jsonl"), catalog)
        .map_err(|error| error.to_string())?;
    let Err(error) = verified_fbx_catalog(root.path()) else {
        return Err("symbolic-linked FBX artifact was accepted".to_owned());
    };
    let diagnostic = error.to_string();
    if !diagnostic.contains("symbolic link")
        && !diagnostic.contains("reparse point")
    {
        return Err("symlink fixture failed for the wrong reason".to_owned());
    }
    Ok(())
}

#[cfg(unix)]
fn create_file_link(target: &Path, link: &Path) -> Result<(), String> {
    std::os::unix::fs::symlink(target, link).map_err(|error| error.to_string())
}

#[cfg(windows)]
fn create_file_link(target: &Path, link: &Path) -> Result<(), String> {
    std::os::windows::fs::symlink_file(target, link)
        .map_err(|error| error.to_string())
}
