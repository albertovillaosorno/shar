// File:
//   - current_directory_creation.rs
// Path:
//   - src/filesystem/tests/current_directory_creation.rs
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
//   - Regression coverage for current-directory creation requests.
// - Must-Not:
//   - Depend on machine-specific paths or caller policy.
// - Allows:
//   - Assert that explicit directory creation performs meaningful work.
// - Split-When:
//   - Split when another malformed directory path needs separate fixtures.
// - Merge-When:
//   - Another test file owns the same current-directory contract.
// - Summary:
//   - Current-directory creation regression tests.
// - Description:
//   - Rejects no-op directory creation presented as successful work.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Current-directory markers alone are invalid creation targets.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for current-directory creation requests.
//!
//! A no-op marker must not be reported as newly created directory state.

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
        return Err(
            format!(
                "unexpected current-directory error kind: {:?}",
                error.kind()
            ),
        );
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn linked_directory_creation_is_rejected() -> Result<(), String> {
    let root = std::env::temp_dir().join(
        format!(
            "schoenwald-filesystem-linked-create-{}",
            std::process::id()
        ),
    );
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    support::create_junction(
        &link, &target,
    )?;

    let result = local::create_dir_all(&link);

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("linked directory reported creation success".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(
            format!(
                "unexpected linked-directory error kind: {:?}",
                error.kind()
            ),
        );
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn linked_parent_directory_creation_is_rejected() -> Result<(), String> {
    let root = std::env::temp_dir().join(
        format!(
            "schoenwald-filesystem-linked-parent-create-{}",
            std::process::id()
        ),
    );
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    support::create_junction(
        &link, &target,
    )?;

    let escaped = target.join("created");
    let result = local::create_dir_all(&link.join("created"));
    let escaped_exists = escaped.exists();

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("directory creation followed linked parent".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(
            format!(
                "unexpected linked-parent error kind: {:?}",
                error.kind()
            ),
        );
    }
    if escaped_exists {
        return Err("linked parent received created directory".to_owned());
    }
    Ok(())
}

#[test]
fn parent_marker_destination_is_rejected_without_side_effects()
-> Result<(), String> {
    let root = std::env::temp_dir().join(
        format!(
            "schoenwald-filesystem-parent-marker-create-{}",
            std::process::id()
        ),
    );
    let intermediate = root.join("scratch");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;

    let result = local::create_dir_all(&intermediate.join(".."));
    let intermediate_exists = intermediate.exists();

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "parent marker reported directory creation success".to_owned(),
        );
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(
            format!(
                "unexpected parent-marker error kind: {:?}",
                error.kind()
            ),
        );
    }
    if intermediate_exists {
        return Err(
            "parent marker created an intermediate directory".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn current_marker_destination_is_rejected_without_side_effects()
-> Result<(), String> {
    let root = std::env::temp_dir().join(
        format!(
            "schoenwald-filesystem-current-marker-create-{}",
            std::process::id()
        ),
    );
    let intermediate = root.join("scratch");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;

    let result = local::create_dir_all(&intermediate.join("."));
    let intermediate_exists = intermediate.exists();

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "current marker reported directory creation success".to_owned(),
        );
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(
            format!(
                "unexpected current-marker error kind: {:?}",
                error.kind()
            ),
        );
    }
    if intermediate_exists {
        return Err(
            "current marker created an intermediate directory".to_owned(),
        );
    }
    Ok(())
}
