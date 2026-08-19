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
//   - Binary domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Binary domain module.
// - Description:
//   - Implements the declared domain module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Binary domain module.

#![expect(
    clippy::redundant_pub_crate,
    reason = "sibling parser modules require crate-visible contracts while \
              the private module prevents external API exposure"
)]

use super::LmlmError;

/// Returns a checked byte range without allowing offset arithmetic to wrap.
pub(crate) fn checked_slice(
    data: &[u8],
    start: usize,
    len: usize,
) -> Option<&[u8]> {
    data.get(start..start.checked_add(len)?)
}

/// Finds the first nonzero byte in a checked archive range.
pub(crate) fn first_nonzero_byte(
    data: &[u8],
    start: usize,
    len: usize,
) -> Result<Option<(usize, u8)>, LmlmError> {
    let bytes = checked_slice(data, start, len).ok_or(LmlmError::Truncated)?;
    let Some(relative) = bytes.iter().position(|byte| *byte != 0) else {
        return Ok(None);
    };
    let offset = start.checked_add(relative).ok_or(LmlmError::Truncated)?;
    let value = bytes.get(relative).copied().ok_or(LmlmError::Truncated)?;
    Ok(Some((offset, value)))
}

/// Adds a structural archive offset and converts overflow into malformed input.
pub(crate) fn checked_offset(
    value: usize,
    delta: usize,
) -> Result<usize, LmlmError> {
    value.checked_add(delta).ok_or(LmlmError::Truncated)
}

/// Reads little-endian directory counters without trusting the archive bounds.
pub(crate) fn read_u16(data: &[u8], pos: usize) -> Option<u16> {
    checked_slice(data, pos, 2)
        .and_then(|slice| slice.try_into().ok())
        .map(u16::from_le_bytes)
}

/// Reads a little-endian header value without trusting the archive bounds.
pub(crate) fn read_u32(data: &[u8], pos: usize) -> Option<u32> {
    checked_slice(data, pos, 4)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_le_bytes)
}

/// Reads little-endian offsets and sizes without letting malformed metadata
/// panic.
pub(crate) fn read_u64(data: &[u8], pos: usize) -> Option<u64> {
    checked_slice(data, pos, 8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
}
