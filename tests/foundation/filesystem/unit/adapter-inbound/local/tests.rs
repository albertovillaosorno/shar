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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{read_utf8, regular_files, write_text};

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn case_root(label: &str) -> std::path::PathBuf {
    let id = CASE_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "schoenwald-filesystem-{label}-{}-{id}",
        std::process::id()
    ))
}

#[test]
fn complete_text_write_creates_parents_and_round_trips() -> Result<(), String> {
    let root = case_root("text-round-trip");
    let destination = root.join("nested/report.txt");

    write_text(&destination, "complete report\n", true)
        .map_err(|error| error.to_string())?;
    let actual = read_utf8(&destination).map_err(|error| error.to_string())?;

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if actual
        != "complete report
"
    {
        return Err(format!("unexpected round-trip text: {actual:?}"));
    }
    Ok(())
}

#[test]
fn recursive_snapshot_is_sorted_and_contains_only_files() -> Result<(), String>
{
    let root = case_root("sorted-tree");
    fs::create_dir_all(root.join("z/nested"))
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(root.join("a")).map_err(|error| error.to_string())?;
    fs::write(root.join("z/nested/two.bin"), b"2")
        .map_err(|error| error.to_string())?;
    fs::write(root.join("a/one.bin"), b"1")
        .map_err(|error| error.to_string())?;

    let actual = regular_files(&root).map_err(|error| error.to_string())?;
    let expected = vec![root.join("a/one.bin"), root.join("z/nested/two.bin")];

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if actual != expected {
        return Err(format!("unexpected recursive snapshot: {actual:?}"));
    }
    Ok(())
}

#[test]
fn invalid_utf8_is_reported_as_invalid_data() -> Result<(), String> {
    let root = case_root("invalid-utf8");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let source = root.join("invalid.txt");
    fs::write(&source, [0xff]).map_err(|error| error.to_string())?;

    let read_error = match read_utf8(&source) {
        Ok(text) => {
            fs::remove_dir_all(&root)
                .map_err(|cleanup_error| cleanup_error.to_string())?;
            return Err(format!(
                "invalid UTF-8 unexpectedly decoded as {text:?}"
            ));
        },
        Err(read_error) => read_error,
    };

    fs::remove_dir_all(&root)
        .map_err(|cleanup_error| cleanup_error.to_string())?;
    if read_error.kind() != std::io::ErrorKind::InvalidData {
        return Err(format!(
            "unexpected invalid UTF-8 error kind: {:?}",
            read_error.kind()
        ));
    }
    Ok(())
}
