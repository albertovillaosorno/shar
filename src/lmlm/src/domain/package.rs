// File:
//   - package.rs
// Path:
//   - src/lmlm/src/domain/package.rs
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
//   - Identity validation for the supported LMLM package.
// - Must-Not:
//   - Write extracted files or bypass checked parser boundaries.
// - Allows:
//   - Operations required by this single LMLM responsibility.
// - Split-When:
//   - One contained invariant gains independent state or a distinct API.
// - Merge-When:
//   - Another LMLM module proves the same invariant without distinction.
// - Summary:
//   - Owns identity validation for the supported lmlm package.
// - Description:
//   - Keeps this parser responsibility deterministic and fail closed.
// - Usage:
//   - Imported only by owned LMLM modules.
// - Defaults:
//   - Malformed input never becomes a portable output identity.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Supported-package identity validation.
//!
//! Reads only the declared metadata payload and matches an exact title line.

// Sibling parser modules share these contracts without exposing them publicly.
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
    let mut expected_lines = JEBANO_TITLE_LF.splitn(
        2,
        |byte| *byte == b'\n',
    );
    let Some(expected_section) = expected_lines.next() else {
        return false;
    };
    let Some(expected_title) = expected_lines.next() else {
        return false;
    };
    let mut in_expected_section = false;
    let mut title_matched = false;
    for raw_line in bytes.split(|byte| *byte == b'\n') {
        let line = raw_line
            .strip_suffix(b"\r")
            .unwrap_or(raw_line);
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
    let bytes = entry_bytes(
        data, metadata,
    )
    .ok_or_else(
        || LmlmError::InvalidEntryRange {
            path: metadata
                .path
                .clone(),
            offset: metadata.offset,
            size: metadata.size,
        },
    )?;
    if metadata_title_matches(bytes) {
        Ok(())
    } else {
        Err(LmlmError::UnsupportedPackage)
    }
}
