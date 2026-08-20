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
//   - Complete FBX catalog publication transaction regression tests.
// - Must-Not:
//   - Read proprietary assets or publish outside the repository work area.
// - Allows:
//   - Synthetic filesystem identities under the repository temporary root.
// - Split-When:
//   - Split when publication transaction tests gain an independent lifecycle.
// - Merge-When:
//   - Merge when another test module owns the identical boundary.
// - Summary:
//   - Complete FBX catalog transaction tests.
// - Description:
//   - Proves fail-before-build and sibling staging invariants.
// - Usage:
//   - Included only by the owning publisher under cfg(test).
// - Defaults:
//   - Existing transaction identities fail closed.
//

//! Complete FBX catalog publication transaction regression tests.

use std::fs;
use std::path::{Path, PathBuf};

use super::{
    cleanup_directory, export_complete_fbx_catalog, manifest_staging_path,
    staging_path,
};

fn case_root(label: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".temp")
        .join(format!("fbx-catalog-{label}-{}", std::process::id()))
}

fn clean(root: &Path) -> Result<(), String> {
    if root.exists() {
        fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
fn catalog_staging_is_hidden_sibling() -> Result<(), String> {
    let output = Path::new("generated/fbx-assets");
    let staging = staging_path(output).map_err(|error| error.to_string())?;
    if staging != Path::new("generated/.fbx-assets.complete-staging") {
        return Err(format!("unexpected catalog staging path: {staging:?}"));
    }
    Ok(())
}

#[test]
fn manifest_staging_is_hidden_sibling() -> Result<(), String> {
    let manifest = Path::new("game/manifest/fbx.jsonl");
    let staging =
        manifest_staging_path(manifest).map_err(|error| error.to_string())?;
    if staging != Path::new("game/manifest/.fbx.jsonl.complete-staging") {
        return Err(format!("unexpected manifest staging path: {staging:?}"));
    }
    Ok(())
}

#[test]
fn existing_output_fails_before_index_access() -> Result<(), String> {
    let root = case_root("existing-output");
    clean(&root)?;
    let output = root.join("accepted");
    fs::create_dir_all(&output).map_err(|error| error.to_string())?;
    fs::write(output.join("sentinel"), b"accepted")
        .map_err(|error| error.to_string())?;
    let result = export_complete_fbx_catalog(
        &root.join("missing-index.jsonl"),
        &output,
        &root.join("fbx.jsonl"),
        &root,
    );
    // jig-ignore-next-line: literal
    let sentinel = fs::read(output.join("sentinel")).map_err(|error| error.to_string())?;
    clean(&root)?;
    let Err(error) = result else {
        return Err("existing accepted catalog was replaced".to_owned());
    };
    // jig-ignore-next-line: literal
    if !error.to_string().contains("output already exists") || sentinel != b"accepted" {
        return Err("existing catalog did not fail before build".to_owned());
    }
    Ok(())
}

#[test]
fn existing_manifest_fails_before_index_access() -> Result<(), String> {
    let root = case_root("existing-manifest");
    clean(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let manifest = root.join("fbx.jsonl");
    fs::write(&manifest, b"accepted").map_err(|error| error.to_string())?;
    let result = export_complete_fbx_catalog(
        &root.join("missing-index.jsonl"),
        &root.join("accepted"),
        &manifest,
        &root,
    );
    let contents = fs::read(&manifest).map_err(|error| error.to_string())?;
    clean(&root)?;
    let Err(error) = result else {
        return Err("existing accepted manifest was replaced".to_owned());
    };
    if !error.to_string().contains("manifest already exists")
        || contents != b"accepted"
    {
        return Err("existing manifest did not fail before build".to_owned());
    }
    Ok(())
}

#[test]
fn cleanup_rejects_non_directory_transaction_identity() -> Result<(), String> {
    let root = case_root("special-cleanup");
    clean(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let transaction = root.join("staging");
    // jig-ignore-next-line: literal
    fs::write(&transaction, b"not-a-directory").map_err(|error| error.to_string())?;
    let result = cleanup_directory(&transaction);
    let still_exists = transaction.is_file();
    clean(&root)?;
    let Err(error) = result else {
        return Err("non-directory transaction identity was removed".to_owned());
    };
    if !still_exists || !error.to_string().contains("changed file kind") {
        return Err("catalog cleanup did not fail closed".to_owned());
    }
    Ok(())
}
