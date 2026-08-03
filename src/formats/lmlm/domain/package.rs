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
//   - Package domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Package domain module.
// - Description:
//   - Implements the declared domain module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Package domain module.

#![expect(
    clippy::redundant_pub_crate,
    reason = "sibling parser modules require crate-visible contracts while \
              the private module prevents external API exposure"
)]

use super::layout::{JEBANO_TITLE_LF, PACKAGE_METADATA_PATH};
use super::payload::entry_bytes;
use super::{FileEntry, LmlmError};

/// Returns whether metadata declares exactly one supported package title.
fn metadata_title_matches(bytes: &[u8]) -> bool {
    let mut expected_lines = JEBANO_TITLE_LF.splitn(2, |byte| *byte == b'\n');
    let Some(expected_section) = expected_lines.next() else {
        return false;
    };
    let Some(expected_title) = expected_lines.next() else {
        return false;
    };
    let mut in_expected_section = false;
    let mut title_matched = false;
    for raw_line in bytes.split(|byte| *byte == b'\n') {
        let line = raw_line.strip_suffix(b"\r").unwrap_or(raw_line);
        if line.starts_with(b"[") && line.ends_with(b"]") {
            in_expected_section = line == expected_section;
            continue;
        }
        if in_expected_section && line.starts_with(b"Title=") {
            if title_matched || line != expected_title {
                return false;
            }
            title_matched = true;
        }
    }
    title_matched
}

/// Fails closed so the extractor cannot become a generic third-party package
/// copier.
pub(crate) fn require_jebano_latino_package(
    data: &[u8],
    entries: &[FileEntry],
) -> Result<(), LmlmError> {
    let metadata = entries
        .iter()
        .find(|entry| entry.path == PACKAGE_METADATA_PATH)
        .ok_or(LmlmError::UnsupportedPackage)?;
    let bytes = entry_bytes(data, metadata).ok_or_else(|| {
        LmlmError::InvalidEntryRange {
            path: metadata.path.clone(),
            offset: metadata.offset,
            size: metadata.size,
        }
    })?;
    if metadata_title_matches(bytes) {
        Ok(())
    } else {
        Err(LmlmError::UnsupportedPackage)
    }
}
