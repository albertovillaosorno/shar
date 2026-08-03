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
//   - Payload outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Payload outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Payload outbound adapter.

use std::io;

use crate::domain::diagnostic::EscapedText;
use crate::domain::{FileEntry, entry_bytes};

/// Resolves every payload range before any filesystem mutation begins.
pub(super) fn preflight_payloads<'a>(
    data: &'a [u8],
    entries: &[FileEntry],
) -> io::Result<Vec<&'a [u8]>> {
    entries
        .iter()
        .map(|entry| {
            entry_bytes(data, entry).ok_or_else(|| {
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
            })
        })
        .collect()
}
