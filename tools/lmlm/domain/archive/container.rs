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
//   - Container domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Container domain module.
// - Description:
//   - Implements the declared domain module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Container domain module.

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
        return Err(LmlmError::BadMagic { observed });
    }
    let version = read_u32(data, VERSION_OFFSET).ok_or(LmlmError::Truncated)?;
    if version != VERSION {
        return Err(LmlmError::UnsupportedVersion {
            offset: VERSION_OFFSET,
            value: version,
        });
    }
    let flags =
        read_u32(data, HEADER_FLAGS_OFFSET).ok_or(LmlmError::Truncated)?;
    if flags != HEADER_FLAGS {
        return Err(LmlmError::UnsupportedHeaderFlags {
            offset: HEADER_FLAGS_OFFSET,
            value: flags,
        });
    }
    for (start, len) in [(8, 4), (0x10, BLOCK.saturating_sub(0x10))] {
        if let Some((offset, value)) = first_nonzero_byte(data, start, len)? {
            return Err(LmlmError::NonZeroReservedHeader { offset, value });
        }
    }
    if let Some((offset, value)) = first_nonzero_byte(data, BLOCK, BLOCK)? {
        return Err(LmlmError::NonZeroReservedContainerBlock { offset, value });
    }
    Ok(())
}

/// Validates the root block and returns its sibling-entry count.
pub(crate) fn read_root_entry_count(data: &[u8]) -> Result<usize, LmlmError> {
    for (start, len) in [
        (ROOT_BLOCK, 2),
        (ROOT_BLOCK.saturating_add(8), BLOCK.saturating_sub(8)),
    ] {
        if let Some((offset, value)) = first_nonzero_byte(data, start, len)? {
            return Err(LmlmError::NonZeroReservedRootBlock { offset, value });
        }
    }
    let flags_offset = checked_offset(ROOT_BLOCK, 4)?;
    let flags = read_u32(data, flags_offset).ok_or(LmlmError::Truncated)?;
    if flags > 1 {
        return Err(LmlmError::UnsupportedRootFlags {
            offset: flags_offset,
            value: flags,
        });
    }
    read_u16(data, checked_offset(ROOT_BLOCK, 2)?)
        .map(usize::from)
        .ok_or(LmlmError::Truncated)
}
