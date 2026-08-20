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
//   - Regression evidence for strict filesystem traversal entry validation.
// - Must-Not:
//   - Follow redirects or accept special entries during strict traversal.
// - Allows:
//   - Build disposable redirect and socket fixtures for traversal checks.
// - Split-When:
//   - Redirect and special-entry policies gain independent test lifecycles.
// - Merge-When:
//   - Another test module owns the identical strict traversal evidence.
// - Summary:
//   - Strict filesystem traversal entry-validation tests.
// - Description:
//   - Proves strict traversal rejects redirected and non-regular entries.
// - Usage:
//   - Run through the schoenwald_filesystem integration test target.
// - Defaults:
//   - Fixtures contain only synthetic local entries.
//

//! Strict filesystem traversal entry-validation tests.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{fs, io};

use schoenwald_filesystem::adapters::driving::local;

#[cfg(windows)]
#[path = "support/junction.rs"]
pub mod support;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn case_root(label: &str) -> PathBuf {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "schoenwald-filesystem-strict-{label}-{}-{sequence}",
        std::process::id()
    ))
}

#[cfg(windows)]
#[test]
fn strict_traversal_rejects_nested_directory_redirect() -> Result<(), String> {
    let root = case_root("nested-redirect");
    let source = root.join("source");
    let outside = root.join("outside");
    let link = source.join("linked");
    fs::create_dir_all(&source).map_err(|error| error.to_string())?;
    fs::create_dir_all(&outside).map_err(|error| error.to_string())?;
    fs::write(outside.join("private.bin"), b"private")
        .map_err(|error| error.to_string())?;
    support::create_junction(&link, &outside)?;

    let result = local::strict_regular_files(&source);
    let cleanup = fs::remove_dir_all(&root).map_err(|error| error.to_string());
    cleanup?;

    let Err(error) = result else {
        return Err("strict traversal accepted a nested redirect".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected strict redirect error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn ordinary_traversal_still_ignores_nested_directory_redirect()
-> Result<(), String> {
    let root = case_root("ordinary-redirect");
    let source = root.join("source");
    let outside = root.join("outside");
    let link = source.join("linked");
    fs::create_dir_all(&source).map_err(|error| error.to_string())?;
    fs::create_dir_all(&outside).map_err(|error| error.to_string())?;
    fs::write(source.join("inside.bin"), b"inside")
        .map_err(|error| error.to_string())?;
    fs::write(outside.join("private.bin"), b"private")
        .map_err(|error| error.to_string())?;
    support::create_junction(&link, &outside)?;

    let files =
        local::regular_files(&source).map_err(|error| error.to_string())?;
    let cleanup = fs::remove_dir_all(&root).map_err(|error| error.to_string());
    cleanup?;

    if files.len() != 1 || files.first() != Some(&source.join("inside.bin")) {
        return Err(format!("ordinary traversal followed redirect: {files:?}"));
    }
    Ok(())
}

#[cfg(unix)]
#[test]
fn strict_traversal_rejects_special_socket_entry() -> Result<(), String> {
    use std::os::unix::net::UnixListener;

    let root = case_root("special-socket");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let socket = root.join("entry.socket");
    let listener =
        UnixListener::bind(&socket).map_err(|error| error.to_string())?;

    let result = local::strict_regular_files(&root);
    drop(listener);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;

    let Err(error) = result else {
        let message = "strict traversal accepted a special socket entry";
        return Err(message.to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected strict special-entry error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}
