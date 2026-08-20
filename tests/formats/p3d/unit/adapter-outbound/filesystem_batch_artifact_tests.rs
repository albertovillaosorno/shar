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
//   - Filesystem batch artifact tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem batch artifact tests test module.
// - Description:
//   - Implements the declared test module responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem batch artifact tests test module.

use std::path::PathBuf;

use super::super::filesystem_batch_cache::is_cache_complete;
use super::cache_component_exists;

/// Resolves one repository-owned cache fixture to an absolute canonical root.
#[expect(
    clippy::panic,
    // jig-ignore-next-line: exact syntax is indivisible
    reason = "A missing repository-owned test fixture is an unrecoverable test setup failure"
)]
fn fixture_root(name: &str) -> PathBuf {
    let candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tests/formats/p3d/fixtures")
        .join(name);
    match candidate.canonicalize() {
        Ok(path) => path,
        Err(error) => panic!(
            "failed to resolve P3D cache fixture {}: {error}",
            candidate.display()
        ),
    }
}

#[test]
fn rejects_invalid_cached_image_artifacts() {
    let package_root = fixture_root("cache-package-invalid-image");
    assert!(!is_cache_complete(&package_root));
}

#[test]
fn rejects_invalid_cached_json_artifacts() {
    let package_root = fixture_root("cache-package-invalid-json");
    assert!(!is_cache_complete(&package_root));
}

#[test]
fn rejects_empty_component_artifacts() {
    let package_root = fixture_root("cache-package");
    let exists = cache_component_exists(&package_root, "mesh/empty.json");
    assert!(!exists);
}

#[test]
fn resolves_manifest_paths_beneath_components_directory() {
    let package_root = fixture_root("cache-package");
    let exists = cache_component_exists(&package_root, "mesh/mesh.json");
    assert!(exists);
}
