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
//   - Current directory creation test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Current directory creation test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Current directory creation test module.

#[cfg(windows)]
#[path = "support/junction.rs"]
pub mod support;

use std::path::Path;
use std::{fs, io};

use schoenwald_filesystem::adapters::driving::local;

#[test]
fn current_directory_creation_is_rejected() -> Result<(), String> {
    let result = local::create_dir_all(Path::new("."));
    let Err(error) = result else {
        return Err("current directory reported creation success".to_owned());
    };

    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected current-directory error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn linked_directory_creation_is_rejected() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "schoenwald-filesystem-linked-create-{}",
        std::process::id()
    ));
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    support::create_junction(&link, &target)?;

    let result = local::create_dir_all(&link);

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("linked directory reported creation success".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected linked-directory error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn linked_parent_directory_creation_is_rejected() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "schoenwald-filesystem-linked-parent-create-{}",
        std::process::id()
    ));
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    support::create_junction(&link, &target)?;

    let escaped = target.join("created");
    let result = local::create_dir_all(&link.join("created"));
    let escaped_exists = escaped.exists();

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("directory creation followed linked parent".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected linked-parent error kind: {:?}",
            error.kind()
        ));
    }
    if escaped_exists {
        return Err("linked parent received created directory".to_owned());
    }
    Ok(())
}

#[test]
fn parent_marker_destination_is_rejected_without_side_effects()
-> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "schoenwald-filesystem-parent-marker-create-{}",
        std::process::id()
    ));
    let intermediate = root.join("scratch");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;

    let result = local::create_dir_all(&intermediate.join(".."));
    let intermediate_exists = intermediate.exists();

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "parent marker reported directory creation success".to_owned()
        );
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected parent-marker error kind: {:?}",
            error.kind()
        ));
    }
    if intermediate_exists {
        return Err(
            "parent marker created an intermediate directory".to_owned()
        );
    }
    Ok(())
}

#[test]
fn current_marker_destination_is_rejected_without_side_effects()
-> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "schoenwald-filesystem-current-marker-create-{}",
        std::process::id()
    ));
    let intermediate = root.join("scratch");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;

    let result = local::create_dir_all(&intermediate.join("."));
    let intermediate_exists = intermediate.exists();

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "current marker reported directory creation success".to_owned()
        );
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected current-marker error kind: {:?}",
            error.kind()
        ));
    }
    if intermediate_exists {
        return Err(
            "current marker created an intermediate directory".to_owned()
        );
    }
    Ok(())
}
