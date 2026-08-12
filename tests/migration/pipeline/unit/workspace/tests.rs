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
//   - Generated workspace compatibility migration unit tests.
// - Must-Not:
//   - Touch repository workspaces or depend on generated corpus state.
// - Allows:
//   - Isolated temporary roots and transaction-lock assertions.
// - Split-When:
//   - Another workspace migration protocol gains independent fixtures.
// - Merge-When:
//   - The owning workspace module no longer has migration behavior.
// - Summary:
//   - Generated workspace compatibility migration tests.
// - Description:
//   - Proves legacy extraction defaults migrate without merging or hiding
//     interrupted transaction state.
// - Usage:
//   - Included only by the owning workspace module under cfg(test).
// - Defaults:
//   - Every fixture is removed before returning success.
//

//! Generated workspace compatibility migration tests.

use std::fs::{self, OpenOptions};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    EXTRACTED_LOCK_NAME, EXTRACTED_TRANSACTION_BLOCKERS,
    EXTRACTED_WORKSPACE_ROOT, FBX_STAGING_NAME, FBX_WORKSPACE_ROOT,
    LEGACY_FBX_MANIFEST_NAME, LEGACY_FBX_WORKSPACE_ROOT,
    LEGACY_UNREAL_MANIFEST_NAME, LEGACY_UNREAL_WORKSPACE_ROOT,
    UNREAL_STAGING_WORKSPACE_ROOT, migrate_legacy_extracted_workspace_at,
    migrate_legacy_payload_workspace_at,
};

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

#[test]
fn absent_legacy_extraction_workspace_is_a_noop() -> TestResult {
    let root = case_root("absent");
    prepare_root(&root)?;
    let migrated = migrate_legacy_extracted_workspace_at(&root)
        .map_err(|error| error.to_string())?;
    let cache_exists = root.join(".cache").exists();
    cleanup_root(&root)?;
    if migrated || cache_exists {
        return Err("absent legacy workspace created migration state".to_owned());
    }
    Ok(())
}

#[test]
fn clean_legacy_extraction_workspace_moves_with_persistent_lock() -> TestResult {
    let root = case_root("clean-with-lock");
    prepare_root(&root)?;
    fs::create_dir_all(root.join("extracted/art"))
        .map_err(|error| error.to_string())?;
    fs::write(root.join("extracted/art/sentinel.bin"), b"accepted")
        .map_err(|error| error.to_string())?;
    fs::write(root.join(EXTRACTED_LOCK_NAME), b"")
        .map_err(|error| error.to_string())?;

    let migrated = migrate_legacy_extracted_workspace_at(&root)
        .map_err(|error| error.to_string())?;
    let canonical = root.join(EXTRACTED_WORKSPACE_ROOT);
    let bytes = fs::read(canonical.join("art/sentinel.bin"))
        .map_err(|error| error.to_string())?;
    let legacy_exists = root.join("extracted").exists();
    let legacy_lock_exists = root.join(EXTRACTED_LOCK_NAME).exists();
    let canonical_lock = canonical.parent().unwrap().join(EXTRACTED_LOCK_NAME);
    let canonical_lock_len = fs::metadata(&canonical_lock)
        .map_err(|error| error.to_string())?
        .len();
    cleanup_root(&root)?;
    if !migrated
        || bytes != b"accepted"
        || legacy_exists
        || legacy_lock_exists
        || canonical_lock_len != 0
    {
        return Err("clean extraction workspace migration drifted".to_owned());
    }
    Ok(())
}

#[test]
fn legacy_extraction_without_lock_gains_canonical_lock() -> TestResult {
    let root = case_root("clean-without-lock");
    prepare_root(&root)?;
    fs::create_dir_all(root.join("extracted"))
        .map_err(|error| error.to_string())?;
    let migrated = migrate_legacy_extracted_workspace_at(&root)
        .map_err(|error| error.to_string())?;
    let canonical = root.join(EXTRACTED_WORKSPACE_ROOT);
    let canonical_lock = canonical.parent().unwrap().join(EXTRACTED_LOCK_NAME);
    let lock_len = fs::metadata(&canonical_lock)
        .map_err(|error| error.to_string())?
        .len();
    cleanup_root(&root)?;
    if !migrated || lock_len != 0 {
        return Err("migration did not establish canonical extraction lock".to_owned());
    }
    Ok(())
}

#[test]
fn competing_extraction_workspaces_fail_without_mutation() -> TestResult {
    let root = case_root("competing");
    prepare_root(&root)?;
    fs::create_dir_all(root.join("extracted"))
        .map_err(|error| error.to_string())?;
    fs::write(root.join("extracted/legacy.txt"), b"legacy")
        .map_err(|error| error.to_string())?;
    let canonical = root.join(EXTRACTED_WORKSPACE_ROOT);
    fs::create_dir_all(&canonical).map_err(|error| error.to_string())?;
    fs::write(canonical.join("canonical.txt"), b"canonical")
        .map_err(|error| error.to_string())?;

    let error = migrate_legacy_extracted_workspace_at(&root)
        .expect_err("competing roots must fail")
        .to_string();
    let legacy = fs::read(root.join("extracted/legacy.txt"))
        .map_err(|error| error.to_string())?;
    let accepted = fs::read(canonical.join("canonical.txt"))
        .map_err(|error| error.to_string())?;
    cleanup_root(&root)?;
    if !error.contains("both exist")
        || legacy != b"legacy"
        || accepted != b"canonical"
    {
        return Err("competing extraction roots were not preserved".to_owned());
    }
    Ok(())
}

#[test]
fn interrupted_legacy_extraction_transaction_fails_closed() -> TestResult {
    let root = case_root("interrupted");
    prepare_root(&root)?;
    fs::create_dir_all(root.join("extracted"))
        .map_err(|error| error.to_string())?;
    let blocker = root.join(EXTRACTED_TRANSACTION_BLOCKERS[0]);
    fs::create_dir_all(&blocker).map_err(|error| error.to_string())?;

    let error = migrate_legacy_extracted_workspace_at(&root)
        .expect_err("interrupted transaction must fail")
        .to_string();
    let legacy_exists = root.join("extracted").is_dir();
    let blocker_exists = blocker.is_dir();
    cleanup_root(&root)?;
    if !error.contains("recover it explicitly")
        || !legacy_exists
        || !blocker_exists
    {
        return Err("interrupted transaction state was not preserved".to_owned());
    }
    Ok(())
}

#[test]
fn active_legacy_extraction_lock_blocks_migration() -> TestResult {
    let root = case_root("active-lock");
    prepare_root(&root)?;
    fs::create_dir_all(root.join("extracted"))
        .map_err(|error| error.to_string())?;
    let lock_path = root.join(EXTRACTED_LOCK_NAME);
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&lock_path)
        .map_err(|error| error.to_string())?;
    lock.try_lock().map_err(|error| error.to_string())?;

    let error = migrate_legacy_extracted_workspace_at(&root)
        .expect_err("active lock must fail")
        .to_string();
    drop(lock);
    let legacy_exists = root.join("extracted").is_dir();
    cleanup_root(&root)?;
    if !error.contains("active legacy extraction transaction") || !legacy_exists {
        return Err("active extraction transaction did not block migration".to_owned());
    }
    Ok(())
}

#[test]
fn complete_legacy_fbx_workspace_moves_payload_and_manifest() -> TestResult {
    let root = case_root("fbx-complete");
    prepare_root(&root)?;
    let legacy = root.join(LEGACY_FBX_WORKSPACE_ROOT);
    fs::create_dir_all(legacy.join("packages/sample"))
        .map_err(|error| error.to_string())?;
    fs::write(legacy.join("packages/sample/sample.fbx"), b"fbx")
        .map_err(|error| error.to_string())?;
    fs::write(legacy.join(LEGACY_FBX_MANIFEST_NAME), b"catalog
")
        .map_err(|error| error.to_string())?;
    let manifest = root.join("game/manifest/fbx.jsonl");

    let migrated = migrate_legacy_payload_workspace_at(
        &root,
        LEGACY_FBX_WORKSPACE_ROOT,
        FBX_WORKSPACE_ROOT,
        Some(LEGACY_FBX_MANIFEST_NAME),
        &manifest,
        &[FBX_STAGING_NAME],
        "FBX",
    )
    .map_err(|error| error.to_string())?;
    let canonical = root.join(FBX_WORKSPACE_ROOT);
    let payload = fs::read(canonical.join("packages/sample/sample.fbx"))
        .map_err(|error| error.to_string())?;
    let manifest_bytes =
        fs::read(&manifest).map_err(|error| error.to_string())?;
    let detached = !canonical.join(LEGACY_FBX_MANIFEST_NAME).exists();
    cleanup_root(&root)?;
    if !migrated || payload != b"fbx" || manifest_bytes != b"catalog
" || !detached {
        return Err("complete FBX workspace migration drifted".to_owned());
    }
    Ok(())
}

#[test]
fn fbx_publication_staging_blocks_legacy_migration() -> TestResult {
    let root = case_root("fbx-staging");
    prepare_root(&root)?;
    fs::create_dir_all(root.join(LEGACY_FBX_WORKSPACE_ROOT))
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(root.join(FBX_STAGING_NAME))
        .map_err(|error| error.to_string())?;
    let manifest = root.join("game/manifest/fbx.jsonl");
    let error = migrate_legacy_payload_workspace_at(
        &root,
        LEGACY_FBX_WORKSPACE_ROOT,
        FBX_WORKSPACE_ROOT,
        Some(LEGACY_FBX_MANIFEST_NAME),
        &manifest,
        &[FBX_STAGING_NAME],
        "FBX",
    )
    .expect_err("FBX staging must block migration")
    .to_string();
    let preserved = root.join(LEGACY_FBX_WORKSPACE_ROOT).is_dir();
    cleanup_root(&root)?;
    if !error.contains("publication staging exists") || !preserved {
        return Err(
            "FBX interrupted publication did not fail closed".to_owned()
        );
    }
    Ok(())
}

#[test]
fn complete_legacy_unreal_workspace_moves_payload_and_manifest() -> TestResult {
    let root = case_root("unreal-complete");
    prepare_root(&root)?;
    let legacy = root.join(LEGACY_UNREAL_WORKSPACE_ROOT);
    fs::create_dir_all(legacy.join("plans"))
        .map_err(|error| error.to_string())?;
    fs::write(legacy.join("plans/index.json"), b"{}")
        .map_err(|error| error.to_string())?;
    fs::write(legacy.join(LEGACY_UNREAL_MANIFEST_NAME), b"unreal
")
        .map_err(|error| error.to_string())?;
    let manifest = root.join("game/manifest/unreal.jsonl");

    let migrated = migrate_legacy_payload_workspace_at(
        &root,
        LEGACY_UNREAL_WORKSPACE_ROOT,
        UNREAL_STAGING_WORKSPACE_ROOT,
        Some(LEGACY_UNREAL_MANIFEST_NAME),
        &manifest,
        &[],
        "Unreal",
    )
    .map_err(|error| error.to_string())?;
    let canonical = root.join(UNREAL_STAGING_WORKSPACE_ROOT);
    let plan = fs::read(canonical.join("plans/index.json"))
        .map_err(|error| error.to_string())?;
    let manifest_bytes =
        fs::read(&manifest).map_err(|error| error.to_string())?;
    let detached = !canonical.join(LEGACY_UNREAL_MANIFEST_NAME).exists();
    cleanup_root(&root)?;
    if !migrated || plan != b"{}" || manifest_bytes != b"unreal
" || !detached {
        return Err("complete Unreal workspace migration drifted".to_owned());
    }
    Ok(())
}

#[test]
fn canonical_manifest_blocks_legacy_payload_migration() -> TestResult {
    let root = case_root("manifest-conflict");
    prepare_root(&root)?;
    let legacy = root.join(LEGACY_UNREAL_WORKSPACE_ROOT);
    fs::create_dir_all(&legacy).map_err(|error| error.to_string())?;
    fs::write(legacy.join(LEGACY_UNREAL_MANIFEST_NAME), b"legacy")
        .map_err(|error| error.to_string())?;
    let manifest = root.join("game/manifest/unreal.jsonl");
    fs::create_dir_all(manifest.parent().unwrap())
        .map_err(|error| error.to_string())?;
    fs::write(&manifest, b"canonical").map_err(|error| error.to_string())?;

    let error = migrate_legacy_payload_workspace_at(
        &root,
        LEGACY_UNREAL_WORKSPACE_ROOT,
        UNREAL_STAGING_WORKSPACE_ROOT,
        Some(LEGACY_UNREAL_MANIFEST_NAME),
        &manifest,
        &[],
        "Unreal",
    )
    .expect_err("canonical manifest must block legacy migration")
    .to_string();
    let old_manifest = fs::read(legacy.join(LEGACY_UNREAL_MANIFEST_NAME))
        .map_err(|error| error.to_string())?;
    let accepted = fs::read(&manifest).map_err(|error| error.to_string())?;
    cleanup_root(&root)?;
    if !error.contains("canonical Unreal manifest already exists")
        || old_manifest != b"legacy"
        || accepted != b"canonical"
    {
        return Err(
            "manifest conflict did not preserve both authorities".to_owned()
        );
    }
    Ok(())
}

fn case_root(label: &str) -> std::path::PathBuf {
    let ordinal = CASE_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "shar-workspace-{label}-{}-{ordinal}",
        std::process::id(),
    ))
}

fn prepare_root(root: &std::path::Path) -> TestResult {
    if root.exists() {
        fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(root).map_err(|error| error.to_string())
}

fn cleanup_root(root: &std::path::Path) -> TestResult {
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}
