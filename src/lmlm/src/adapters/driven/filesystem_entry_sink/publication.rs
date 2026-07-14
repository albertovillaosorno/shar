// File:
//   - publication.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/publication.rs
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
//   - Filesystem publication of preflighted LMLM payloads.
// - Must-Not:
//   - Parse archives, validate paths, or overwrite existing files.
// - Allows:
//   - Create required directories and create-new destination files.
// - Split-When:
//   - Transaction rollback or staging gains independent state.
// - Merge-When:
//   - Another module owns identical create-new publication mechanics.
// - Summary:
//   - Publishes preflighted payloads to resolved destinations.
// - Description:
//   - Performs only filesystem mutations after all validation succeeds.
// - Usage:
//   - Called by the filesystem materialization facade.
// - Defaults:
//   - Existing files are never overwritten.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Publication mechanics for preflighted LMLM entries.
//!
//! Destination creation begins only after payload and path validation finish.

use std::fs::{self, OpenOptions};
use std::io::{self, Write as _};
use std::path::PathBuf;

/// Writes every preflighted payload to its resolved destination.
pub(super) fn publish_entries(
    destinations: Vec<PathBuf>,
    payloads: Vec<&[u8]>,
) -> io::Result<()> {
    for (destination, bytes) in destinations
        .into_iter()
        .zip(payloads)
    {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination)?;
        file.write_all(bytes)?;
    }
    Ok(())
}
