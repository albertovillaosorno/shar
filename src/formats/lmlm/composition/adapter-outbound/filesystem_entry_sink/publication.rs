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
//   - Publication outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Publication outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Publication outbound adapter.

use std::fs::{self, OpenOptions};
use std::io::{self, Write as _};
use std::path::PathBuf;

/// Writes every preflighted payload to its resolved destination.
pub(super) fn publish_entries(
    destinations: Vec<PathBuf>,
    payloads: Vec<&[u8]>,
) -> io::Result<()> {
    for (destination, bytes) in destinations.into_iter().zip(payloads) {
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
