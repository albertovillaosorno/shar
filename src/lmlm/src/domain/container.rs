// File:
//   - container.rs
// Path:
//   - src/lmlm/src/domain/container.rs
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
//   - Fixed LSPA header and root-block validation.
// - Must-Not:
//   - Write extracted files or accept unchecked archive structure.
// - Allows:
//   - Operations required by this single LMLM responsibility.
// - Split-When:
//   - One contained invariant gains independent state or a distinct API.
// - Merge-When:
//   - Another LMLM module proves the same invariant without distinction.
// - Summary:
//   - Owns fixed lspa header and root-block validation.
// - Description:
//   - Keeps this parser responsibility deterministic and fail closed.
// - Usage:
//   - Imported only by parser orchestration.
// - Defaults:
//   - Every structural range is bounded before interpretation.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Fixed container and root-block validation.
//!
//! Checks magic, version, flags, reservations, and the bounded root count.

// Sibling parser modules share these contracts without exposing them publicly.
#![expect(
    clippy::redundant_pub_crate,
    reason = "sibling parser modules require crate-visible contracts while \
              the private module prevents external API exposure"
)]

use super::LmlmError;
use super::binary::{checked_offset, first_nonzero_byte, read_u16, read_u32};
use super::layout::{
    BLOCK, HEADER_FLAGS, HEADER_FLAGS_OFFSET, MAGIC, ROOT_BLOCK, VERSION,
    VERSION_OFFSET,
};

/// Validates fixed LSPA header fields before reading the directory table.
pub(crate) fn validate_header(data: &[u8]) -> Result<(), LmlmError> {
    let Some(magic) = data.get(0..MAGIC.len()) else {
        return Err(LmlmError::Truncated);
    };
    let Ok(observed) = <[u8; 4]>::try_from(magic) else {
        return Err(LmlmError::Truncated);
    };
    if observed != *MAGIC {
        return Err(
            LmlmError::BadMagic {
                observed,
            },
        );
    }
    let version = read_u32(
        data,
        VERSION_OFFSET,
    )
    .ok_or(LmlmError::Truncated)?;
    if version != VERSION {
        return Err(
            LmlmError::UnsupportedVersion {
                offset: VERSION_OFFSET,
                value: version,
            },
        );
    }
    let flags = read_u32(
        data,
        HEADER_FLAGS_OFFSET,
    )
    .ok_or(LmlmError::Truncated)?;
    if flags != HEADER_FLAGS {
        return Err(
            LmlmError::UnsupportedHeaderFlags {
                offset: HEADER_FLAGS_OFFSET,
                value: flags,
            },
        );
    }
    for (start, len) in [
        (
            8, 4,
        ),
        (
            0x10,
            BLOCK.saturating_sub(0x10),
        ),
    ] {
        if let Some((offset, value)) = first_nonzero_byte(
            data, start, len,
        )? {
            return Err(
                LmlmError::NonZeroReservedHeader {
                    offset,
                    value,
                },
            );
        }
    }
    if let Some((offset, value)) = first_nonzero_byte(
        data, BLOCK, BLOCK,
    )? {
        return Err(
            LmlmError::NonZeroReservedContainerBlock {
                offset,
                value,
            },
        );
    }
    Ok(())
}

/// Validates the root block and returns its sibling-entry count.
pub(crate) fn read_root_entry_count(data: &[u8]) -> Result<usize, LmlmError> {
    for (start, len) in [
        (
            ROOT_BLOCK, 2,
        ),
        (
            ROOT_BLOCK.saturating_add(4),
            BLOCK.saturating_sub(4),
        ),
    ] {
        if let Some((offset, value)) = first_nonzero_byte(
            data, start, len,
        )? {
            return Err(
                LmlmError::NonZeroReservedRootBlock {
                    offset,
                    value,
                },
            );
        }
    }
    read_u16(
        data,
        checked_offset(
            ROOT_BLOCK, 2,
        )?,
    )
    .map(usize::from)
    .ok_or(LmlmError::Truncated)
}
