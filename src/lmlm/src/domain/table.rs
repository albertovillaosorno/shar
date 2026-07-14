// File:
//   - table.rs
// Path:
//   - src/lmlm/src/domain/table.rs
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
//   - Recursive entry-table traversal and metadata decoding.
// - Must-Not:
//   - Write extracted files or accept unchecked archive structure.
// - Allows:
//   - Operations required by this single LMLM responsibility.
// - Split-When:
//   - One contained invariant gains independent state or a distinct API.
// - Merge-When:
//   - Another LMLM module proves the same invariant without distinction.
// - Summary:
//   - Owns recursive entry-table traversal and metadata decoding.
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

//! Recursive LMLM entry-table traversal.
//!
//! Decodes bounded records, tracks table extent, and creates file entries.

// Sibling parser modules share these contracts without exposing them publicly.
#![expect(
    clippy::redundant_pub_crate,
    reason = "sibling parser modules require crate-visible contracts while \
              the private module prevents external API exposure"
)]

use std::collections::BTreeMap;

use super::binary::{checked_offset, first_nonzero_byte, read_u16, read_u64};
use super::layout::{BLOCK, ENTRY_KIND, MAX_DIRECTORY_DEPTH};
use super::name::{read_name, register_path};
use super::{FileEntry, LmlmError};

/// Rejects bytes outside the timestamp, size, offset, and count fields.
fn validate_metadata_reserved_bytes(
    data: &[u8],
    meta: usize,
) -> Result<(), LmlmError> {
    for (start, len) in [
        (
            meta, 2,
        ),
        (
            checked_offset(
                meta, 0x0b,
            )?,
            1,
        ),
    ] {
        if let Some((padding_offset, value)) = first_nonzero_byte(
            data, start, len,
        )? {
            return Err(
                LmlmError::NonZeroMetadataPadding {
                    offset: padding_offset,
                    value,
                },
            );
        }
    }
    Ok(())
}

/// Reads the entry offset and validates the reserved metadata bytes.
fn read_metadata_offset(
    data: &[u8],
    meta: usize,
) -> Result<u64, LmlmError> {
    validate_metadata_reserved_bytes(
        data, meta,
    )?;
    let offset = read_u64(
        data,
        checked_offset(
            meta, 0x14,
        )?,
    )
    .ok_or(LmlmError::Truncated)?;
    if let Some((padding_offset, value)) = first_nonzero_byte(
        data,
        checked_offset(
            meta, 0x1c,
        )?,
        BLOCK.saturating_sub(0x1c),
    )? {
        return Err(
            LmlmError::NonZeroMetadataPadding {
                offset: padding_offset,
                value,
            },
        );
    }
    Ok(offset)
}

/// Reads a directory child-kind control and validates its reserved tail.
fn read_directory_record_control(
    data: &[u8],
    meta: usize,
) -> Result<
    (
        usize,
        u8,
    ),
    LmlmError,
> {
    let control_offset = checked_offset(
        meta, 0x0e,
    )?;
    let control = data
        .get(control_offset)
        .copied()
        .ok_or(LmlmError::Truncated)?;
    if control > 1 {
        return Err(
            LmlmError::UnsupportedDirectoryRecordControl {
                offset: control_offset,
                value: control,
            },
        );
    }
    let padding_start = checked_offset(
        control_offset,
        1,
    )?;
    if let Some((padding_offset, value)) = first_nonzero_byte(
        data,
        padding_start,
        5,
    )? {
        return Err(
            LmlmError::NonZeroMetadataPadding {
                offset: padding_offset,
                value,
            },
        );
    }
    Ok(
        (
            control_offset,
            control,
        ),
    )
}

/// Validated fields needed before descending into one directory.
struct DirectoryRecord {
    /// Archive-relative offset of the child-kind control byte.
    control_offset: usize,
    /// Declared child-kind control value.
    declared_control: u8,
    /// Number of immediate child records.
    child_count: usize,
    /// Validated recursion depth for the child list.
    child_depth: usize,
}

/// Reads one directory record and validates its recursion depth.
fn read_directory_record(
    data: &[u8],
    meta: usize,
    path: &str,
    depth: usize,
) -> Result<DirectoryRecord, LmlmError> {
    let (control_offset, declared_control) = read_directory_record_control(
        data, meta,
    )?;
    let child_count = usize::from(
        read_u16(
            data,
            checked_offset(
                meta, 0x0c,
            )?,
        )
        .ok_or(LmlmError::Truncated)?,
    );
    let child_depth = depth
        .checked_add(1)
        .ok_or(LmlmError::Truncated)?;
    if child_depth > MAX_DIRECTORY_DEPTH {
        return Err(
            LmlmError::ExcessiveDirectoryDepth {
                path: path.to_owned(),
                depth: child_depth,
            },
        );
    }
    Ok(
        DirectoryRecord {
            control_offset,
            declared_control,
            child_count,
            child_depth,
        },
    )
}

/// Validates a directory control against its immediate child kinds.
fn validate_directory_record_control(
    path: &str,
    offset: usize,
    declared: u8,
    contains_directory: bool,
) -> Result<(), LmlmError> {
    let expected = u8::from(contains_directory);
    if declared != expected {
        return Err(
            LmlmError::DirectoryRecordControlMismatch {
                path: path.to_owned(),
                offset,
                declared,
                expected,
            },
        );
    }
    Ok(())
}

/// Validates one file transition block and returns its exclusive end.
fn validate_file_record_control(
    data: &[u8],
    control_start: usize,
) -> Result<usize, LmlmError> {
    let control = data
        .get(control_start)
        .copied()
        .ok_or(LmlmError::Truncated)?;
    if control > 1 {
        return Err(
            LmlmError::UnsupportedFileRecordControl {
                offset: control_start,
                value: control,
            },
        );
    }
    let padding_start = checked_offset(
        control_start,
        1,
    )?;
    if let Some((padding_offset, value)) = first_nonzero_byte(
        data,
        padding_start,
        BLOCK.saturating_sub(1),
    )? {
        return Err(
            LmlmError::NonZeroFileRecordPadding {
                offset: padding_offset,
                value,
            },
        );
    }
    checked_offset(
        control_start,
        BLOCK,
    )
}

/// Returns the end of one file record, including an optional final control.
fn file_record_end(
    data: &[u8],
    metadata_end: usize,
    globally_final: bool,
    entries: &[FileEntry],
) -> Result<usize, LmlmError> {
    if !globally_final {
        return validate_file_record_control(
            data,
            metadata_end,
        );
    }
    let earliest_payload = entries
        .iter()
        .map(|entry| entry.offset)
        .min()
        .and_then(|offset| usize::try_from(offset).ok());
    let control_end = checked_offset(
        metadata_end,
        BLOCK,
    )?;
    if earliest_payload
        .is_some_and(|payload_start| payload_start >= control_end)
    {
        return validate_file_record_control(
            data,
            metadata_end,
        );
    }
    Ok(metadata_end)
}

/// Result of parsing one immediate sibling list.
struct ParsedEntries {
    /// First byte after the parsed sibling list and all descendants.
    next_pos: usize,
    /// Whether at least one immediate sibling was a directory.
    contains_directory: bool,
}

/// Parses root siblings and initializes bounded recursive state.
pub(crate) fn parse_entries(
    data: &[u8],
    pos: usize,
    count: usize,
    prefix: &str,
    out: &mut Vec<FileEntry>,
    seen_paths: &mut BTreeMap<String, String>,
    table_end: &mut usize,
) -> Result<usize, LmlmError> {
    let mut state = (
        out, seen_paths, table_end,
    );
    parse_entries_at(
        data, pos, count, prefix, &mut state, 0, true,
    )
    .map(|parsed| parsed.next_pos)
}

/// Parses `count` sibling entries and their bounded descendants.
fn parse_entries_at(
    data: &[u8],
    mut pos: usize,
    count: usize,
    prefix: &str,
    state: &mut (
        &mut Vec<FileEntry>,
        &mut BTreeMap<String, String>,
        &mut usize,
    ),
    depth: usize,
    globally_final_branch: bool,
) -> Result<ParsedEntries, LmlmError> {
    let mut contains_directory = false;
    for index in 0..count {
        let globally_final =
            globally_final_branch && index.saturating_add(1) == count;
        let kind = read_u16(
            data, pos,
        )
        .ok_or(LmlmError::Truncated)?;
        if kind != ENTRY_KIND {
            return Err(
                LmlmError::InvalidEntryKind {
                    offset: pos,
                    value: kind,
                },
            );
        }
        let full_path = register_path(
            read_name(
                data, pos,
            )?,
            prefix,
            &mut *state.1,
        )?;
        let meta = checked_offset(
            pos, BLOCK,
        )?;
        let metadata_end = checked_offset(
            meta, BLOCK,
        )?;
        *state.2 = (*state.2).max(metadata_end);
        let offset = read_metadata_offset(
            data, meta,
        )?;
        if offset == 0 {
            contains_directory = true;
            let directory = read_directory_record(
                data, meta, &full_path, depth,
            )?;
            let child_prefix = format!("{full_path}/");
            let parsed_children = parse_entries_at(
                data,
                checked_offset(
                    pos,
                    BLOCK.saturating_mul(2),
                )?,
                directory.child_count,
                &child_prefix,
                state,
                directory.child_depth,
                globally_final,
            )?;
            validate_directory_record_control(
                &full_path,
                directory.control_offset,
                directory.declared_control,
                parsed_children.contains_directory,
            )?;
            pos = parsed_children.next_pos;
        } else {
            let size = read_u64(
                data,
                checked_offset(
                    meta, 0x0c,
                )?,
            )
            .ok_or(LmlmError::Truncated)?;
            state
                .0
                .push(
                    FileEntry {
                        path: full_path,
                        offset,
                        size,
                    },
                );
            pos = file_record_end(
                data,
                metadata_end,
                globally_final,
                state.0,
            )?;
            *state.2 = (*state.2).max(pos);
        }
    }
    Ok(
        ParsedEntries {
            next_pos: pos,
            contains_directory,
        },
    )
}
