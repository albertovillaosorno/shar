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
//   - Path inspection test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Path inspection test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Path inspection test module.

#[cfg(windows)]
#[path = "support/junction.rs"]
pub mod support;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::{fs, io};

#[cfg(windows)]
use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn case_root(label: &str) -> std::path::PathBuf {
    let id = CASE_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "schoenwald-filesystem-{label}-{}-{id}",
        std::process::id()
    ))
}

#[test]
fn directory_length_is_rejected() -> Result<(), String> {
    let root = case_root("directory-length");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;

    let result = local::file_len(&root);

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let error = match result {
        Ok(length) => {
            return Err(format!(
                "directory unexpectedly reported file length {length}"
            ));
        },
        Err(error) => error,
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected directory-length error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[test]
fn regular_file_traversal_root_is_rejected() -> Result<(), String> {
    let root = case_root("file-root");
    drop(fs::File::create(&root).map_err(|error| error.to_string())?);

    let result = local::regular_files(&root);

    fs::remove_file(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("regular file root was traversed".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected file-root error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}
#[cfg(windows)]
#[test]
fn linked_directory_traversal_root_is_rejected() -> Result<(), String> {
    let root = case_root("linked-root");
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    fs::write(target.join("outside.bin"), b"payload")
        .map_err(|error| error.to_string())?;
    support::create_junction(&link, &target)?;

    let result = local::regular_files(&link);

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("linked directory root was traversed".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected linked-root error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn linked_directory_kind_is_other() -> Result<(), String> {
    let root = case_root("linked-kind");
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    support::create_junction(&link, &target)?;

    let kind = local::path_kind(&link).map_err(|error| error.to_string())?;

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if kind != PathKind::Other {
        return Err(format!("linked directory reported kind {kind:?}"));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn linked_file_length_is_rejected() -> Result<(), String> {
    let root = case_root("linked-length");
    let target = root.join("target");
    let link = root.join("link");
    local::write_bytes(&target.join("private.bin"), b"private", true)
        .map_err(|error| error.to_string())?;
    support::create_junction(&link, &target)?;

    let result = local::file_len(&link.join("private.bin"));

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("file length followed linked prefix".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected linked-length error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn linked_prefix_path_kind_is_rejected() -> Result<(), String> {
    let root = case_root("linked-prefix-kind");
    let target = root.join("target");
    let link = root.join("link");
    local::write_bytes(&target.join("private.bin"), b"private", true)
        .map_err(|error| error.to_string())?;
    support::create_junction(&link, &target)?;

    let result = local::path_kind(&link.join("private.bin"));

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("path kind followed linked prefix".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected linked-kind error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn traversal_rejects_linked_root_prefix() -> Result<(), String> {
    let root = case_root("linked-prefix-root");
    let target = root.join("target");
    let link = root.join("link");
    local::write_bytes(&target.join("nested/private.bin"), b"private", true)
        .map_err(|error| error.to_string())?;
    support::create_junction(&link, &target)?;

    let result = local::regular_files(&link.join("nested"));

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("traversal followed linked root prefix".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected linked-root-prefix error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}
