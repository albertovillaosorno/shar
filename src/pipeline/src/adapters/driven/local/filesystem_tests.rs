// File:
//   - filesystem_tests.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/filesystem_tests.rs
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
//   - Regression coverage for deterministic shared filesystem traversal.
// - Must-Not:
//   - Test unrelated phase behavior or depend on repository-local data.
// - Allows:
//   - Isolated temporary files removed before the test returns.
// - Split-When:
//   - Traversal gains a second independently testable contract.
// - Merge-When:
//   - Filesystem traversal tests move to a dedicated integration-test crate.
// - Summary:
//   - Deterministic filesystem traversal regressions.
// - Description:
//   - Verifies canonical result ordering for shared recursive traversal.
// - Usage:
//   - Included only by phase/filesystem.rs under cfg(test).
// - Defaults:
//   - Test fixtures are local, unique, and removed before return.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Regression tests for deterministic shared filesystem traversal.
//!
//! Each case uses explicit local fixtures so ordering and file-only behavior
//! remain readable without sharing production test helpers.

use std::fs;

use super::collect_files;

#[test]
fn collect_files_returns_paths_in_canonical_order() -> Result<(), String> {
    let root = std::env::temp_dir().join(
        format!(
            "pipeline-filesystem-order-{}",
            std::process::id(),
        ),
    );
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.to_string()),
    }
    for directory in [
        "zeta", "alpha", "middle",
    ] {
        let path = root
            .join(directory)
            .join("file.bin");
        fs::create_dir_all(
            path.parent()
                .ok_or_else(|| String::from("missing parent"))?,
        )
        .map_err(|error| error.to_string())?;
        fs::write(
            &path, directory,
        )
        .map_err(|error| error.to_string())?;
    }

    let actual = collect_files(&root).map_err(|error| error.to_string())?;
    let mut expected = actual.clone();
    expected.sort();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;

    if actual != expected {
        return Err(
            format!("filesystem traversal was not canonical: {actual:?}"),
        );
    }
    Ok(())
}
