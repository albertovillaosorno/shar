// File:
//   - payload.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/payload.rs
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
//   - Preflight resolution of bounded LMLM entry payloads.
// - Must-Not:
//   - Resolve destinations or mutate filesystem state.
// - Allows:
//   - Convert entry ranges into borrowed archive slices.
// - Split-When:
//   - Streaming payload sources need an independent contract.
// - Merge-When:
//   - Payload range validation moves entirely into the domain value.
// - Summary:
//   - Resolves every payload before publication begins.
// - Description:
//   - Fails direct callers closed when an entry range is not bounded.
// - Usage:
//   - Called by the filesystem materialization facade.
// - Defaults:
//   - No payload is copied.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Payload preflight for LMLM filesystem materialization.
//!
//! Every entry becomes a borrowed archive slice before any filesystem write.

use std::io;

use crate::diagnostic::EscapedText;
use crate::domain::{FileEntry, entry_bytes};

/// Resolves every payload range before any filesystem mutation begins.
pub(super) fn preflight_payloads<'a>(
    data: &'a [u8],
    entries: &[FileEntry],
) -> io::Result<Vec<&'a [u8]>> {
    entries
        .iter()
        .map(
            |entry| {
                entry_bytes(
                    data, entry,
                )
                .ok_or_else(
                    || {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "entry out of bounds: {} at offset {} with \
                                 size {}",
                                EscapedText::new(&entry.path),
                                entry.offset,
                                entry.size
                            ),
                        )
                    },
                )
            },
        )
        .collect()
}
