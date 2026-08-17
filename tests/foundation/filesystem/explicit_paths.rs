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
//   - Explicit paths test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Explicit paths test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Explicit paths test module.

#[cfg(windows)]
#[path = "support/junction.rs"]
pub mod support;

use std::path::Path;
use std::io;

#[cfg(windows)]
use std::fs;

use schoenwald_filesystem::adapters::driving::local;

#[test]
fn empty_optional_read_path_is_rejected() -> Result<(), String> {
    let result = local::read_optional_utf8(Path::new(""));
    let Err(error) = result else {
        return Err(
            "empty path unexpectedly returned optional state".to_owned()
        );
    };

    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected empty-path error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn read_rejects_linked_path_prefix() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "schoenwald-filesystem-linked-read-{}",
        std::process::id()
    ));
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    fs::write(target.join("private.bin"), b"private")
        .map_err(|error| error.to_string())?;
    support::create_junction(&link, &target)?;

    let result = local::read_bytes(&link.join("private.bin"));

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("read followed linked path prefix".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected linked-read error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn reserved_device_write_path_is_rejected() -> Result<(), String> {
    let result = local::write_bytes(Path::new("NUL"), b"payload", false);
    let Err(error) = result else {
        return Err("reserved device accepted a file write".to_owned());
    };

    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected reserved-device error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}
