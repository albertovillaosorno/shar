// File:
//   - archive.rs
// Path:
//   - src/rcf/tests/fixture/archive.rs
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
//   - Checked synthetic RCF container bytes for integration regressions.
// - Must-Not:
//   - Read private assets, perform filesystem discovery, or assert outcomes.
// - Allows:
//   - Minimal byte framing and an in-memory archive reader.
// - Split-When:
//   - A second container layout requires independent fixture construction.
// - Merge-When:
//   - RCF integration tests no longer share synthetic archive framing.
// - Summary:
//   - Builds public-safe in-memory RCF archives.
// - Description:
//   - Provides deterministic valid and malformed byte containers to tests.
// - Usage:
//   - Included as a nested module by RCF integration targets.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: The checked synthetic archive builder covers multiple malformed
//   - parser layouts and exceeds the generated source-size threshold.
//

//! Checked synthetic RCF archive construction.
//!
//! The fixture models external framing only and never reads local game assets.
use rcf::ArchiveParser;
use rcf::domain::{Archive, ArchiveError};
use rcf::ports::ArchiveByteReader;

const CATALOG_OFFSET: usize = 0x800;
const CATALOG_NAME_TABLE_FIELD: usize = 0x804;
const CATALOG_FIRST_FILE_FIELD: usize = 0x808;
const INDEX_OFFSET: usize = 0x810;
const INDEX_PAYLOAD_OFFSET_FIELD: usize = 0x814;
const INDEX_LENGTH_FIELD: usize = 0x818;
const SECOND_INDEX_PAYLOAD_FIELD: usize = 0x820;
const NAME_TABLE_OFFSET: usize = 0x81c;
const NAME_LENGTH_FIELD: usize = 0x824;
const NAME_BYTES_OFFSET: usize = 0x828;
const PAYLOAD_OFFSET: usize = 0x1000;
const ARCHIVE_LENGTH: usize = 0x1001;
const MULTI_NAME_TABLE_OFFSET: usize = 0x1000;
const MULTI_PAYLOAD_OFFSET: usize = 0x2000;
const MAGIC: &[u8] = b"RADCORE CEMENT LIBRARY";
const FILE_INFO_ALIGNMENT_OFFSET: usize = 36;
const FILE_INFO_BIG_ENDIAN_OFFSET: usize = 34;
const FILE_INFO_HEADER_OFFSET: usize = 44;
const FILE_INFO_PAD_NET_OFFSET: usize = 40;
const FILE_INFO_VALID_OFFSET: usize = 35;
const FILE_INFO_VERSION_MAJOR_OFFSET: usize = 32;
const FILE_INFO_VERSION_MINOR_OFFSET: usize = 33;
const FORMAT_ALIGNMENT: u32 = 0x800;
const FORMAT_VERSION_MAJOR: u8 = 1;
const FORMAT_VERSION_MINOR: u8 = 2;

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn parse_archive(bytes: Vec<u8>) -> Result<Archive, ArchiveError> {
    ArchiveParser::from_reader(
        &mut MemoryReader {
            bytes,
            declared_length: None,
        },
    )
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn parse_archive_with_declared_length(
    bytes: Vec<u8>,
    declared_length: u64,
) -> Result<Archive, ArchiveError> {
    ArchiveParser::from_reader(
        &mut MemoryReader {
            bytes,
            declared_length: Some(declared_length),
        },
    )
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_alignment_five() -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_unaligned_header()?;
    write_u32(
        &mut bytes,
        FILE_INFO_ALIGNMENT_OFFSET,
        5,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_big_endian_flag(
    value: u8
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_stored_name(b"sound/file.rsd\0")?;
    write_u8(
        &mut bytes,
        FILE_INFO_BIG_ENDIAN_OFFSET,
        value,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_valid_flag(
    value: u8
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_stored_name(b"sound/file.rsd\0")?;
    write_u8(
        &mut bytes,
        FILE_INFO_VALID_OFFSET,
        value,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_first_file_offset(
    value: u32
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_stored_name(b"sound/file.rsd\0")?;
    write_u32(
        &mut bytes,
        CATALOG_FIRST_FILE_FIELD,
        value,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_multi_first_file_offset(
    value: u32
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_stored_names(
        &[
            b"sound/second.rsd\0",
            b"sound/first.rsd\0",
        ],
    )?;
    write_u32(
        &mut bytes,
        CATALOG_FIRST_FILE_FIELD,
        value,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_header_overlap() -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = vec![0_u8; 56];
    write_file_info(&mut bytes)?;
    write_u32(
        &mut bytes,
        FILE_INFO_HEADER_OFFSET,
        24,
    )?;
    write_u32(
        &mut bytes, 24, 0,
    )?;
    write_u32(
        &mut bytes, 28, 48,
    )?;
    write_u32(
        &mut bytes, 48, 0,
    )?;
    write_u32(
        &mut bytes, 52, 0,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_unaligned_header() -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = vec![0_u8; 72];
    write_file_info(&mut bytes)?;
    write_u32(
        &mut bytes,
        FILE_INFO_HEADER_OFFSET,
        48,
    )?;
    write_u32(
        &mut bytes, 48, 0,
    )?;
    write_u32(
        &mut bytes, 52, 64,
    )?;
    write_u32(
        &mut bytes, 64, 0,
    )?;
    write_u32(
        &mut bytes, 68, 0,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_stored_name(
    raw_name: &[u8]
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = vec![0_u8; ARCHIVE_LENGTH];
    write_file_info(&mut bytes)?;
    write_u32(
        &mut bytes,
        CATALOG_OFFSET,
        1,
    )?;
    write_u32(
        &mut bytes,
        CATALOG_NAME_TABLE_FIELD,
        usize_to_u32(
            NAME_TABLE_OFFSET,
            "name table offset",
        )?,
    )?;
    write_u32(
        &mut bytes,
        CATALOG_FIRST_FILE_FIELD,
        usize_to_u32(
            PAYLOAD_OFFSET,
            "first file offset",
        )?,
    )?;
    write_u32(
        &mut bytes,
        INDEX_OFFSET,
        name_hash32(
            raw_name
                .strip_suffix(&[0])
                .unwrap_or(raw_name),
        ),
    )?;
    write_u32(
        &mut bytes,
        INDEX_PAYLOAD_OFFSET_FIELD,
        usize_to_u32(
            PAYLOAD_OFFSET,
            "payload offset",
        )?,
    )?;
    write_u32(
        &mut bytes,
        INDEX_LENGTH_FIELD,
        1,
    )?;
    write_u32(
        &mut bytes,
        NAME_TABLE_OFFSET,
        1,
    )?;
    write_u32(
        &mut bytes,
        NAME_LENGTH_FIELD,
        usize_to_u32(
            raw_name.len(),
            "name length",
        )?,
    )?;
    copy_at(
        &mut bytes,
        NAME_BYTES_OFFSET,
        raw_name,
    )?;
    let payload = bytes
        .get_mut(PAYLOAD_OFFSET)
        .ok_or_else(
            || {
                ArchiveError::invalid_archive(
                    "fixture payload offset is invalid",
                )
            },
        )?;
    *payload = 1;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_magic_suffix(
    value: u8
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_stored_name(b"sound/file.rsd\0")?;
    write_u8(
        &mut bytes,
        MAGIC.len(),
        value,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_modification_time(
    raw_name: &[u8],
    modification_time: u32,
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_stored_name(raw_name)?;
    let modification_time_offset = checked_offset(
        NAME_BYTES_OFFSET,
        raw_name.len(),
        "modification time offset",
    )?;
    write_u32(
        &mut bytes,
        modification_time_offset,
        modification_time,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_payload_inside_modification_time(
    raw_name: &[u8]
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_modification_time(
        raw_name,
        0x1234_5678,
    )?;
    let modification_time_offset = checked_offset(
        NAME_BYTES_OFFSET,
        raw_name.len(),
        "modification time offset",
    )?;
    write_u32(
        &mut bytes,
        INDEX_PAYLOAD_OFFSET_FIELD,
        usize_to_u32(
            modification_time_offset,
            "modification time payload offset",
        )?,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_payload_inside_name_table(
    raw_name: &[u8]
) -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = archive_with_stored_name(raw_name)?;
    write_u32(
        &mut bytes,
        INDEX_PAYLOAD_OFFSET_FIELD,
        usize_to_u32(
            NAME_BYTES_OFFSET,
            "name bytes offset",
        )?,
    )?;
    Ok(bytes)
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_index_beyond_declared_length() -> Result<
    (
        Vec<u8>,
        u64,
    ),
    ArchiveError,
> {
    let mut bytes = vec![0_u8; ARCHIVE_LENGTH];
    write_file_info(&mut bytes)?;
    write_u32(
        &mut bytes,
        CATALOG_OFFSET,
        2,
    )?;
    write_u32(
        &mut bytes,
        CATALOG_NAME_TABLE_FIELD,
        usize_to_u32(
            NAME_BYTES_OFFSET,
            "oversized name table offset",
        )?,
    )?;
    write_u32(
        &mut bytes,
        INDEX_OFFSET,
        1,
    )?;
    write_u32(
        &mut bytes,
        INDEX_PAYLOAD_OFFSET_FIELD,
        usize_to_u32(
            SECOND_INDEX_PAYLOAD_FIELD,
            "declared archive length",
        )?,
    )?;
    write_u32(
        &mut bytes,
        NAME_TABLE_OFFSET,
        2,
    )?;
    write_u32(
        &mut bytes,
        SECOND_INDEX_PAYLOAD_FIELD,
        usize_to_u32(
            SECOND_INDEX_PAYLOAD_FIELD,
            "second payload offset",
        )?,
    )?;
    Ok(
        (
            bytes,
            u64::try_from(SECOND_INDEX_PAYLOAD_FIELD).map_err(
                |source| {
                    ArchiveError::invalid_archive(
                        format!("fixture length does not fit u64: {source}"),
                    )
                },
            )?,
        ),
    )
}

// The nested fixture exposes this helper only to its parent integration test.
#[expect(
    clippy::redundant_pub_crate,
    reason = "The nested fixture exposes this helper only to its parent test."
)]
pub(super) fn archive_with_stored_names(
    raw_names: &[&[u8]]
) -> Result<Vec<u8>, ArchiveError> {
    let archive_length = checked_offset(
        MULTI_PAYLOAD_OFFSET,
        raw_names.len(),
        "archive length",
    )?;
    let mut bytes = vec![0_u8; archive_length];
    write_multi_header(
        &mut bytes,
        raw_names.len(),
    )?;
    let mut name_cursor = checked_offset(
        MULTI_NAME_TABLE_OFFSET,
        8,
        "name cursor",
    )?;
    for (index, raw_name) in raw_names
        .iter()
        .enumerate()
    {
        name_cursor = write_multi_entry(
            &mut bytes,
            name_cursor,
            index,
            raw_name,
        )?;
    }
    Ok(bytes)
}

fn write_multi_header(
    bytes: &mut [u8],
    count: usize,
) -> Result<(), ArchiveError> {
    write_file_info(bytes)?;
    let count_u32 = usize_to_u32(
        count,
        "entry count",
    )?;
    write_u32(
        bytes,
        CATALOG_OFFSET,
        count_u32,
    )?;
    write_u32(
        bytes,
        CATALOG_NAME_TABLE_FIELD,
        usize_to_u32(
            MULTI_NAME_TABLE_OFFSET,
            "multi-name table offset",
        )?,
    )?;
    write_u32(
        bytes,
        CATALOG_FIRST_FILE_FIELD,
        usize_to_u32(
            MULTI_PAYLOAD_OFFSET,
            "multi first file offset",
        )?,
    )?;
    write_u32(
        bytes,
        MULTI_NAME_TABLE_OFFSET,
        count_u32,
    )?;
    write_u32(
        bytes,
        checked_offset(
            MULTI_NAME_TABLE_OFFSET,
            4,
            "detailed-info pointer slot",
        )?,
        0,
    )
}

fn write_multi_entry(
    bytes: &mut [u8],
    name_cursor: usize,
    index: usize,
    raw_name: &[u8],
) -> Result<usize, ArchiveError> {
    let record_delta = index
        .checked_mul(12)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture index row overflow"),
        )?;
    let record_offset = checked_offset(
        INDEX_OFFSET,
        record_delta,
        "index offset",
    )?;
    let payload_offset = checked_offset(
        MULTI_PAYLOAD_OFFSET,
        index,
        "payload offset",
    )?;
    write_multi_record(
        bytes,
        record_offset,
        payload_offset,
        raw_name,
    )?;
    let next_name_cursor = write_multi_name(
        bytes,
        name_cursor,
        raw_name,
    )?;
    let payload = bytes
        .get_mut(payload_offset)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture payload is invalid"),
        )?;
    *payload = u8::try_from(index.saturating_add(1)).map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("fixture payload value does not fit u8: {source}"),
            )
        },
    )?;
    Ok(next_name_cursor)
}

fn write_multi_record(
    bytes: &mut [u8],
    record_offset: usize,
    payload_offset: usize,
    raw_name: &[u8],
) -> Result<(), ArchiveError> {
    write_u32(
        bytes,
        record_offset,
        name_hash32(
            raw_name
                .strip_suffix(&[0])
                .unwrap_or(raw_name),
        ),
    )?;
    write_u32(
        bytes,
        checked_offset(
            record_offset,
            4,
            "payload field",
        )?,
        usize_to_u32(
            payload_offset,
            "multi payload offset",
        )?,
    )?;
    write_u32(
        bytes,
        checked_offset(
            record_offset,
            8,
            "payload length field",
        )?,
        1,
    )
}

fn write_multi_name(
    bytes: &mut [u8],
    name_cursor: usize,
    raw_name: &[u8],
) -> Result<usize, ArchiveError> {
    write_u32(
        bytes,
        name_cursor,
        usize_to_u32(
            raw_name.len(),
            "multi name length",
        )?,
    )?;
    let name_start = checked_offset(
        name_cursor,
        4,
        "name bytes",
    )?;
    copy_at(
        bytes, name_start, raw_name,
    )?;
    let modification_time_offset = checked_offset(
        name_start,
        raw_name.len(),
        "modification time offset",
    )?;
    write_u32(
        bytes,
        modification_time_offset,
        0,
    )?;
    checked_offset(
        modification_time_offset,
        4,
        "name record end",
    )
}

fn checked_offset(
    base: usize,
    delta: usize,
    context: &str,
) -> Result<usize, ArchiveError> {
    base.checked_add(delta)
        .ok_or_else(
            || {
                ArchiveError::invalid_archive(
                    format!("fixture {context} overflow"),
                )
            },
        )
}

fn usize_to_u32(
    value: usize,
    context: &str,
) -> Result<u32, ArchiveError> {
    u32::try_from(value).map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("fixture {context} does not fit u32: {source}"),
            )
        },
    )
}

fn write_file_info(bytes: &mut [u8]) -> Result<(), ArchiveError> {
    copy_at(
        bytes, 0, MAGIC,
    )?;
    write_u8(
        bytes,
        FILE_INFO_VERSION_MAJOR_OFFSET,
        FORMAT_VERSION_MAJOR,
    )?;
    write_u8(
        bytes,
        FILE_INFO_VERSION_MINOR_OFFSET,
        FORMAT_VERSION_MINOR,
    )?;
    write_u8(
        bytes,
        FILE_INFO_BIG_ENDIAN_OFFSET,
        0,
    )?;
    write_u8(
        bytes,
        FILE_INFO_VALID_OFFSET,
        1,
    )?;
    write_u32(
        bytes,
        FILE_INFO_ALIGNMENT_OFFSET,
        FORMAT_ALIGNMENT,
    )?;
    write_u32(
        bytes,
        FILE_INFO_PAD_NET_OFFSET,
        0,
    )?;
    write_u32(
        bytes,
        FILE_INFO_HEADER_OFFSET,
        FORMAT_ALIGNMENT,
    )
}

fn write_u8(
    bytes: &mut [u8],
    offset: usize,
    value: u8,
) -> Result<(), ArchiveError> {
    let target = bytes
        .get_mut(offset)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture u8 offset is invalid"),
        )?;
    *target = value;
    Ok(())
}

fn write_u32(
    bytes: &mut [u8],
    offset: usize,
    value: u32,
) -> Result<(), ArchiveError> {
    copy_at(
        bytes,
        offset,
        &value.to_le_bytes(),
    )
}

fn copy_at(
    bytes: &mut [u8],
    offset: usize,
    value: &[u8],
) -> Result<(), ArchiveError> {
    let end = offset
        .checked_add(value.len())
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture write range overflow"),
        )?;
    let target = bytes
        .get_mut(offset..end)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture write exceeds bytes"),
        )?;
    target.copy_from_slice(value);
    Ok(())
}

fn name_hash32(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .fold(
            0_u32,
            |value, item| {
                let adjusted = if *item < b'a' {
                    item.saturating_add(32)
                } else {
                    *item
                };
                value
                    .wrapping_mul(31)
                    .wrapping_add(u32::from(adjusted))
            },
        )
}

struct MemoryReader {
    bytes: Vec<u8>,
    declared_length: Option<u64>,
}

impl ArchiveByteReader for MemoryReader {
    fn len(&self) -> Result<u64, ArchiveError> {
        if let Some(length) = self.declared_length {
            return Ok(length);
        }
        u64::try_from(
            self.bytes
                .len(),
        )
        .map_err(
            |source| {
                ArchiveError::invalid_archive(
                    format!("fixture length does not fit u64: {source}"),
                )
            },
        )
    }

    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ArchiveError> {
        let start = usize::try_from(offset).map_err(
            |source| {
                ArchiveError::invalid_archive(
                    format!("fixture offset does not fit usize: {source}"),
                )
            },
        )?;
        let count = usize::try_from(length).map_err(
            |source| {
                ArchiveError::invalid_archive(
                    format!("fixture length does not fit usize: {source}"),
                )
            },
        )?;
        let end = start
            .checked_add(count)
            .ok_or_else(
                || ArchiveError::invalid_archive("fixture range overflow"),
            )?;
        self.bytes
            .get(start..end)
            .map(ToOwned::to_owned)
            .ok_or_else(
                || ArchiveError::invalid_archive("fixture range exceeds bytes"),
            )
    }
}
