// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - File rtf source outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - File rtf source outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! File rtf source outbound adapter.

use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{fs, io};

use schoenwald_filesystem::adapters::driving::local;

use crate::ports::{RtfSnapshot, RtfSource};

/// Loads RTF snapshots from local files.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileRtfSource;

impl RtfSource for FileRtfSource {
    fn load(&self, path: &Path) -> io::Result<RtfSnapshot> {
        let bytes = local::read_bytes(path)?;
        let modified_unix_seconds = fs::metadata(path)
            .ok()
            .and_then(|metadata| metadata.modified().ok())
            .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
            .and_then(|duration| i64::try_from(duration.as_secs()).ok());
        Ok(RtfSnapshot {
            bytes,
            modified_unix_seconds,
        })
    }
}
