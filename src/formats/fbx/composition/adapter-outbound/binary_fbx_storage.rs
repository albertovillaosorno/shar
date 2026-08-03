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
//   - Binary fbx storage outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Binary fbx storage outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Binary fbx storage outbound adapter.

use std::fs::OpenOptions;
use std::io::{ErrorKind, Write as _};
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use super::binary_character_writer::CharacterBinaryFbxError;

/// Persist one complete binary document without overwriting existing output.
///
/// # Errors
///
/// Returns a typed writer error when parent creation, create-new opening, or
/// complete byte persistence fails.
pub(super) fn persist_binary_fbx(
    path: &Path,
    bytes: &[u8],
) -> Result<(), CharacterBinaryFbxError> {
    let Some(parent) = path.parent() else {
        return Err(CharacterBinaryFbxError::MissingParent(
            path.display().to_string(),
        ));
    };
    local::create_dir_all(parent).map_err(|source| {
        CharacterBinaryFbxError::CreateDir {
            path: parent.display().to_string(),
            source: source.to_string(),
        }
    })?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|source| {
        if source.kind() == ErrorKind::AlreadyExists {
            CharacterBinaryFbxError::OutputExists(path.display().to_string())
        } else {
            CharacterBinaryFbxError::Write {
                path: path.display().to_string(),
                source: source.to_string(),
            }
        }
    })?;
    file.write_all(bytes)
        .map_err(|source| CharacterBinaryFbxError::Write {
            path: path.display().to_string(),
            source: source.to_string(),
        })
}
