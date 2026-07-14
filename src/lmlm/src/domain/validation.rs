// File:
//   - validation.rs
// Path:
//   - src/lmlm/src/domain/validation.rs
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
//   - Parsed LMLM payload bounds, alignment, aliases, and trailing padding.
// - Must-Not:
//   - Parse directory records or write extracted files.
// - Allows:
//   - Checked arithmetic over parsed file-entry metadata.
// - Split-When:
//   - One validation family gains independent state or publication policy.
// - Merge-When:
//   - Another LMLM validation module proves the same payload invariants.
// - Summary:
//   - Fails malformed payload layouts before extraction.
// - Description:
//   - Validates parsed entry ranges against archive and table boundaries.
// - Usage:
//   - Called by the LMLM parser after directory traversal.
// - Defaults:
//   - Payloads are block aligned, disjoint, in bounds, and followed by zeros.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Payload-layout validation for parsed LMLM entries.
//!
//! Enforces archive bounds, block alignment, table separation, disjoint ranges,
//! and zero trailing padding before any extraction adapter receives an entry.

// Sibling parser modules share these contracts without exposing them publicly.
#![expect(
    clippy::redundant_pub_crate,
    reason = "sibling parser modules require crate-visible contracts while \
              the private module prevents external API exposure"
)]

use super::binary::{checked_slice, first_nonzero_byte};
use super::layout::BLOCK_U64;
use super::{FileEntry, LmlmError};

/// Validates one file payload before cross-entry alias checks.
fn validate_file_payload(
    data: &[u8],
    path: &str,
    offset: u64,
    size: u64,
    table_end: usize,
) -> Result<(), LmlmError> {
    let Ok(payload_start) = usize::try_from(offset) else {
        return Err(
            LmlmError::InvalidEntryRange {
                path: path.to_owned(),
                offset,
                size,
            },
        );
    };
    if payload_start < table_end {
        return Err(
            LmlmError::EntryPayloadOverlapsTable {
                path: path.to_owned(),
                offset,
                table_end,
            },
        );
    }
    if offset.checked_rem(BLOCK_U64) != Some(0) {
        return Err(
            LmlmError::UnalignedEntryOffset {
                path: path.to_owned(),
                offset,
            },
        );
    }
    let Some(payload_size) = usize::try_from(size).ok() else {
        return Err(
            LmlmError::InvalidEntryRange {
                path: path.to_owned(),
                offset,
                size,
            },
        );
    };
    if checked_slice(
        data,
        payload_start,
        payload_size,
    )
    .is_none()
    {
        return Err(
            LmlmError::InvalidEntryRange {
                path: path.to_owned(),
                offset,
                size,
            },
        );
    }
    Ok(())
}

/// Orders parsed entries by deterministic payload identity.
fn sorted_entries(entries: &[FileEntry]) -> Vec<&FileEntry> {
    let mut sorted: Vec<&FileEntry> = entries
        .iter()
        .collect();
    sorted.sort_by(
        |left, right| {
            left.offset
                .cmp(&right.offset)
                .then_with(
                    || {
                        left.size
                            .cmp(&right.size)
                    },
                )
                .then_with(
                    || {
                        left.path
                            .cmp(&right.path)
                    },
                )
        },
    );
    sorted
}

/// Rejects aliases after entries are ordered by deterministic payload identity.
fn validate_overlapping_ranges(
    entries: &[&FileEntry]
) -> Result<(), LmlmError> {
    for pair in entries.windows(2) {
        let Some(first) = pair
            .first()
            .copied()
        else {
            continue;
        };
        let Some(second) = pair
            .get(1)
            .copied()
        else {
            continue;
        };
        let first_end = first
            .offset
            .checked_add(first.size)
            .ok_or_else(
                || LmlmError::InvalidEntryRange {
                    path: first
                        .path
                        .clone(),
                    offset: first.offset,
                    size: first.size,
                },
            )?;
        if second.offset < first_end {
            return Err(
                LmlmError::OverlappingEntryRanges {
                    first_path: first
                        .path
                        .clone(),
                    first_offset: first.offset,
                    first_size: first.size,
                    second_path: second
                        .path
                        .clone(),
                    second_offset: second.offset,
                    second_size: second.size,
                },
            );
        }
    }
    Ok(())
}

/// Rejects nonzero bytes between the table and declared payload ranges.
fn validate_unclaimed_padding(
    data: &[u8],
    entries: &[&FileEntry],
    table_end: usize,
) -> Result<(), LmlmError> {
    let mut claimed_end = table_end;
    for entry in entries {
        let Ok(start) = usize::try_from(entry.offset) else {
            return Err(
                LmlmError::InvalidEntryRange {
                    path: entry
                        .path
                        .clone(),
                    offset: entry.offset,
                    size: entry.size,
                },
            );
        };
        let Ok(size) = usize::try_from(entry.size) else {
            return Err(
                LmlmError::InvalidEntryRange {
                    path: entry
                        .path
                        .clone(),
                    offset: entry.offset,
                    size: entry.size,
                },
            );
        };
        let gap_len = start
            .checked_sub(claimed_end)
            .ok_or(LmlmError::Truncated)?;
        if let Some((offset, value)) = first_nonzero_byte(
            data,
            claimed_end,
            gap_len,
        )? {
            return Err(
                LmlmError::NonZeroUnclaimedData {
                    offset,
                    value,
                },
            );
        }
        claimed_end = start
            .checked_add(size)
            .ok_or(LmlmError::Truncated)?;
    }
    Ok(())
}

/// Rejects nonzero bytes after the final declared payload.
fn validate_trailing_padding(
    data: &[u8],
    entries: &[FileEntry],
    table_end: usize,
) -> Result<(), LmlmError> {
    let mut payload_end = table_end;
    for entry in entries {
        let Some(start) = usize::try_from(entry.offset).ok() else {
            return Err(
                LmlmError::InvalidEntryRange {
                    path: entry
                        .path
                        .clone(),
                    offset: entry.offset,
                    size: entry.size,
                },
            );
        };
        let Some(size) = usize::try_from(entry.size).ok() else {
            return Err(
                LmlmError::InvalidEntryRange {
                    path: entry
                        .path
                        .clone(),
                    offset: entry.offset,
                    size: entry.size,
                },
            );
        };
        let Some(end) = start.checked_add(size) else {
            return Err(
                LmlmError::InvalidEntryRange {
                    path: entry
                        .path
                        .clone(),
                    offset: entry.offset,
                    size: entry.size,
                },
            );
        };
        payload_end = payload_end.max(end);
    }
    let trailing_len = data
        .len()
        .checked_sub(payload_end)
        .ok_or(LmlmError::Truncated)?;
    if let Some((offset, value)) = first_nonzero_byte(
        data,
        payload_end,
        trailing_len,
    )? {
        return Err(
            LmlmError::NonZeroTrailingData {
                offset,
                value,
            },
        );
    }
    Ok(())
}

/// Validates every payload range and rejects aliases before extraction.
pub(crate) fn validate_entry_ranges(
    data: &[u8],
    entries: &[FileEntry],
    table_end: usize,
) -> Result<(), LmlmError> {
    for entry in entries {
        validate_file_payload(
            data,
            &entry.path,
            entry.offset,
            entry.size,
            table_end,
        )?;
    }
    let sorted = sorted_entries(entries);
    validate_overlapping_ranges(&sorted)?;
    validate_unclaimed_padding(
        data, &sorted, table_end,
    )?;
    validate_trailing_padding(
        data, entries, table_end,
    )
}
