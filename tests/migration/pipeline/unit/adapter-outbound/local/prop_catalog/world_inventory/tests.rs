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
//   - World inventory unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - World inventory unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! World inventory unit tests.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::decoded_mesh_names;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn fixture_root(label: &str) -> Result<PathBuf, String> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "shar-world-inventory-map-{label}-{}-{sequence}",
        std::process::id()
    ));
    fs::create_dir_all(root.join("components/mesh"))
        .map_err(|error| error.to_string())?;
    Ok(root)
}

#[test]
fn decoded_mesh_names_reject_duplicate_source_identity() -> Result<(), String> {
    let root = fixture_root("duplicate-name")?;
    fs::write(root.join("components/mesh/001.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    fs::write(root.join("components/mesh/002.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    // jig-ignore-next-line: literal
    let result = decoded_mesh_names(&root, &["001".to_owned(), "002".to_owned()]);
    drop(fs::remove_dir_all(&root));
    let Err(error) = result else {
        return Err("duplicate decoded mesh identity was accepted".to_owned());
    };
    if !error
        .to_string()
        .contains("world prop repeats mesh identity shared")
    {
        // jig-ignore-next-line: literal
        return Err(format!("unexpected duplicate mesh identity error: {error}"));
    }
    Ok(())
}
