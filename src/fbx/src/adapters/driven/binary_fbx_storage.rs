// File:
//   - binary_fbx_storage.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_fbx_storage.rs
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
//   - Atomic create-new persistence for complete binary FBX documents.
// - Must-Not:
//   - Build scene objects, encode FBX nodes, or infer model semantics.
// - Allows:
//   - Parent creation, create-new file opening, and complete byte writes.
// - Summary:
//   - Keeps filesystem storage outside the binary scene serializer.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Binary FBX filesystem persistence adapter.

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
        return Err(
            CharacterBinaryFbxError::MissingParent(
                path.display()
                    .to_string(),
            ),
        );
    };
    local::create_dir_all(parent).map_err(
        |source| CharacterBinaryFbxError::CreateDir {
            path: parent
                .display()
                .to_string(),
            source: source.to_string(),
        },
    )?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(
            |source| {
                if source.kind() == ErrorKind::AlreadyExists {
                    CharacterBinaryFbxError::OutputExists(
                        path.display()
                            .to_string(),
                    )
                } else {
                    CharacterBinaryFbxError::Write {
                        path: path
                            .display()
                            .to_string(),
                        source: source.to_string(),
                    }
                }
            },
        )?;
    file.write_all(bytes)
        .map_err(
            |source| CharacterBinaryFbxError::Write {
                path: path
                    .display()
                    .to_string(),
                source: source.to_string(),
            },
        )
}
