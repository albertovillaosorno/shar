// File:
//   - filesystem_batch_artifact_tests.rs
// Path:
//   - src/p3d/src/adapters/driven/filesystem_batch_artifact_tests.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for physical P3D batch cache artifacts.
// - Must-Not:
//   - Mutate fixtures, execute extraction, or validate manifest semantics.
// - Allows:
//   - Inspect committed cache-package fixtures through the driven adapter.
// - Split-When:
//   - Another artifact family requires independently maintained fixtures.
// - Merge-When:
//   - Physical artifact checks no longer differ from manifest validation.
// - Summary:
//   - Physical P3D batch cache artifact regressions.
// - Description:
//   - Verifies component-root resolution and regular-file evidence.
// - Usage:
//   - Included by filesystem_batch_cache.rs under cfg(test).
// - Defaults:
//   - Fixtures are read-only and repository-relative through
//   - CARGO_MANIFEST_DIR.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Regression tests for physical P3D batch cache artifacts.
//!
//! Manifest paths are relative to the package `components` directory rather
//! than to the package output root itself.

use std::path::PathBuf;

use super::super::filesystem_batch_cache::is_cache_complete;
use super::cache_component_exists;

#[test]
fn rejects_invalid_cached_image_artifacts() {
    let mut package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    package_root.push("tests/fixtures/cache-package-invalid-image");
    assert!(!is_cache_complete(&package_root));
}

#[test]
fn rejects_invalid_cached_json_artifacts() {
    let mut package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    package_root.push("tests/fixtures/cache-package-invalid-json");
    assert!(!is_cache_complete(&package_root));
}

#[test]
fn rejects_empty_component_artifacts() {
    let mut package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    package_root.push("tests/fixtures/cache-package");
    let exists = cache_component_exists(
        &package_root,
        "mesh/empty.json",
    );
    assert!(!exists);
}

#[test]
fn resolves_manifest_paths_beneath_components_directory() {
    let mut package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    package_root.push("tests/fixtures/cache-package");
    let exists = cache_component_exists(
        &package_root,
        "mesh/mesh.json",
    );
    assert!(exists);
}
