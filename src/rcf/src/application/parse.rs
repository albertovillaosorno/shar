// File:
//   - parse.rs
// Path:
//   - src/rcf/src/application/parse.rs
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
//   - rcf use-case orchestration for application parse.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when parse contains two independently testable contracts.
// - Merge-When:
//   - Another rcf module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - Archive parsing query use cases.
// - Description:
//   - Defines parse data and behavior for rcf application.
// - Usage:
//   - Called by entrypoints after ports and adapters are selected by the
//   - caller.
// - Defaults:
//   - No concrete adapter is selected unless the caller supplies one through a
//   - port.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: src/rcf/src/application/parse.rs has 625 effective lines after
//   - the
//   - required header and remains cohesive until a focused split lands.
//

//! Archive parsing query use cases.
//!
//! This boundary keeps archive parsing query use cases explicit and returns
//! deterministic results to rcf callers.
use std::collections::{BTreeMap, BTreeSet};

use crate::domain::path_policy::contains_unsafe_unicode_path_control;
use crate::domain::{
    Archive, ArchiveEntry, ArchiveError, ArchiveHeader, IndexRecord,
};
use crate::ports::{ArchiveByteReader, ArchiveSource};

/// The archive signature is fixed by the external RCF container contract.
const MAGIC: &[u8] = b"RADCORE CEMENT LIBRARY";
/// The fixed file-info structure ends after its header-position word.
const FILE_INFO_BYTES: u64 = 48;
/// The catalog header is four little-endian words before entry records begin.
const CATALOG_HEADER_BYTES: u64 = 16;
/// Each index row is three little-endian words in the external table layout.
const INDEX_RECORD_BYTES: usize = 12;
/// Major version byte in the fixed `radCFFileInfo` structure.
const FILE_INFO_VERSION_MAJOR_OFFSET: usize = 32;
/// Minor version byte in the fixed `radCFFileInfo` structure.
const FILE_INFO_VERSION_MINOR_OFFSET: usize = 33;
/// Endianness flag in the fixed `radCFFileInfo` structure.
const FILE_INFO_BIG_ENDIAN_OFFSET: usize = 34;
/// Completion flag in the fixed `radCFFileInfo` structure.
const FILE_INFO_VALID_OFFSET: usize = 35;
/// Alignment word in the fixed `radCFFileInfo` structure.
const FILE_INFO_ALIGNMENT_OFFSET: usize = 36;
/// Catalog position word in the fixed `radCFFileInfo` structure.
const FILE_INFO_HEADER_START_OFFSET: usize = 44;
/// Supported RCF major version from the original runtime contract.
const FORMAT_VERSION_MAJOR: u8 = 1;
/// Supported RCF minor version from the original runtime contract.
const FORMAT_VERSION_MINOR: u8 = 2;
/// Detailed file information starts with count and a serialized pointer slot.
const DETAILED_INFO_HEADER_BYTES: u64 = 8;
/// Each detailed row stores one little-endian modification timestamp.
const MODIFICATION_TIME_BYTES: u64 = 4;
/// Each detailed row prefixes the stored name with its byte length.
const NAME_LENGTH_BYTES: u64 = 4;
/// Parser offsets advance in little-endian word units throughout the table.
const U32_BYTES: usize = 4;
/// Reader ranges are u64, so the same word width is kept without casting.
const U32_BYTES_U64: u64 = 4;

/// Keeps catalog scalar fields together because index boundaries and archive
/// metadata must be derived from the same validated prefix.
struct CatalogFields {
    /// Entry count determines the exact index and name row counts.
    entry_count: usize,
    /// Name-table offset bounds the preceding index table.
    name_table_offset: u32,
    /// Declared start of the first stored file payload.
    first_file_offset: u32,
}

/// One parsed detailed-name row and the cursor position after it.
struct DetailedNameRecord {
    /// Last modification time stored beside the file name.
    modification_time: u32,
    /// Decoded archive-relative file name.
    name: String,
    /// Absolute offset immediately after the row.
    next_offset: u64,
}

/// Reads catalog scalar fields only after the fixed prefix has been validated.
fn parse_catalog_fields(bytes: &[u8]) -> Result<CatalogFields, ArchiveError> {
    let entry_count = usize::try_from(
        read_u32(
            bytes, 0,
        )?,
    )
    .map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("entry count does not fit usize: {source}"),
            )
        },
    )?;
    Ok(
        CatalogFields {
            entry_count,
            name_table_offset: read_u32(
                bytes, U32_BYTES,
            )?,
            first_file_offset: read_u32(
                bytes,
                U32_BYTES.saturating_mul(2),
            )?,
        },
    )
}

/// Validated catalog coordinates and metadata derived from file-info bytes.
struct CatalogLayout {
    /// Alignment reported by the fixed file-info structure.
    alignment: u32,
    /// Catalog scalar values read from the declared header position.
    catalog: CatalogFields,
    /// Absolute start of the hash index table.
    index_start: u64,
    /// Exact byte length of the hash index table.
    index_length: u64,
}

/// Reads and validates the fixed file-info and declared catalog header.
///
/// # Errors
///
/// Returns an error when either header is truncated, inconsistent, or outside
/// the declared archive length.
fn read_catalog_layout(
    reader: &mut dyn ArchiveByteReader,
    archive_size: u64,
) -> Result<CatalogLayout, ArchiveError> {
    if archive_size < FILE_INFO_BYTES {
        return Err(
            ArchiveError::invalid_archive(
                format!("archive is too small: {archive_size} bytes"),
            ),
        );
    }

    let file_info = reader.read_exact_range(
        0,
        FILE_INFO_BYTES,
    )?;
    validate_magic(&file_info)?;
    validate_version(&file_info)?;
    validate_completion(&file_info)?;
    validate_endianness(&file_info)?;

    let alignment = read_u32(
        &file_info,
        FILE_INFO_ALIGNMENT_OFFSET,
    )?;
    let header_start = u64::from(
        read_u32(
            &file_info,
            FILE_INFO_HEADER_START_OFFSET,
        )?,
    );
    if header_start < FILE_INFO_BYTES {
        return Err(
            ArchiveError::invalid_archive("catalog header overlaps file info"),
        );
    }
    let is_aligned = !alignment.is_power_of_two()
        || header_start.checked_rem(u64::from(alignment)) == Some(0);
    if !is_aligned {
        return Err(
            ArchiveError::invalid_archive("catalog header is not aligned"),
        );
    }
    let Some(index_start) = header_start.checked_add(CATALOG_HEADER_BYTES)
    else {
        return Err(
            ArchiveError::invalid_archive("catalog header end overflow"),
        );
    };
    if index_start > archive_size {
        return Err(
            ArchiveError::invalid_archive(
                "catalog header extends beyond archive data",
            ),
        );
    }

    let catalog_payload = reader.read_exact_range(
        header_start,
        CATALOG_HEADER_BYTES,
    )?;
    let catalog = parse_catalog_fields(&catalog_payload)?;
    let Some(index_bytes) = catalog
        .entry_count
        .checked_mul(INDEX_RECORD_BYTES)
    else {
        return Err(
            ArchiveError::invalid_archive("index table byte count overflow"),
        );
    };
    let index_length = usize_to_u64(
        index_bytes,
        "index byte count",
    )?;
    let Some(index_end) = index_start.checked_add(index_length) else {
        return Err(ArchiveError::invalid_archive("index table end overflow"));
    };
    validate_index_end(
        index_end,
        archive_size,
        catalog.name_table_offset,
    )?;

    let layout = CatalogLayout {
        alignment,
        catalog,
        index_start,
        index_length,
    };
    Ok(layout)
}

/// Parses an archive index without extracting payloads.
#[derive(Debug, Default, Clone, Copy)]
pub struct ArchiveParser;

impl ArchiveParser {
    /// Parses one archive source.
    ///
    /// # Errors
    ///
    /// Returns an error when the source cannot be opened or the archive is
    /// malformed.
    pub fn execute(
        source: &impl ArchiveSource
    ) -> Result<Archive, ArchiveError> {
        let mut reader = source.open_reader()?;
        Self::from_reader(reader.as_mut())
    }

    /// Parses an archive from a byte reader.
    ///
    /// # Errors
    ///
    /// Returns an error when required ranges cannot be read or index/name
    /// tables are inconsistent.
    pub fn from_reader(
        reader: &mut dyn ArchiveByteReader
    ) -> Result<Archive, ArchiveError> {
        let archive_size = reader.len()?;
        let layout = read_catalog_layout(
            reader,
            archive_size,
        )?;
        let CatalogLayout {
            alignment,
            catalog,
            index_start,
            index_length,
        } = layout;
        let index_payload = reader.read_exact_range(
            index_start,
            index_length,
        )?;
        let records = parse_records(
            &index_payload,
            catalog.entry_count,
            archive_size,
        )?;
        validate_hash_index_order(&records)?;
        let mut records_by_offset = records.clone();
        records_by_offset.sort_by_key(
            |record| {
                (
                    record.offset,
                    record.length,
                    record.hash,
                )
            },
        );
        validate_records_do_not_overlap(&records_by_offset)?;

        let (names, metadata_end) = parse_names(
            reader,
            u64::from(catalog.name_table_offset),
            catalog.entry_count,
            archive_size,
        )?;
        validate_records_after_metadata(
            &records,
            metadata_end,
        )?;
        let first_file_offset = u64::from(catalog.first_file_offset);
        validate_first_file_hint(
            &records,
            first_file_offset,
            alignment,
            archive_size,
        )?;
        let entries = build_entries(
            records, names,
        )?;

        let header = ArchiveHeader {
            entry_count: catalog.entry_count,
            name_table_offset: u64::from(catalog.name_table_offset),
            alignment,
            first_file_offset: u64::from(catalog.first_file_offset),
        };
        let archive = Archive {
            header,
            entries,
            archive_size,
        };
        Ok(archive)
    }
}

/// Lists an archive index.
#[derive(Debug, Default, Clone, Copy)]
pub struct ListArchive;

impl ListArchive {
    /// Parses and returns all resolved entries.
    ///
    /// # Errors
    ///
    /// Returns an error when parsing the archive index fails.
    pub fn execute(
        source: &impl ArchiveSource
    ) -> Result<Vec<ArchiveEntry>, ArchiveError> {
        ArchiveParser::execute(source).map(|archive| archive.entries)
    }
}

/// Converts parser offsets only at reader boundaries where the port requires
/// u64, keeping host pointer width out of archive math.
///
/// # Errors
///
/// Returns an error when a host-sized offset cannot fit the archive range type.
fn usize_to_u64(
    value: usize,
    context: &str,
) -> Result<u64, ArchiveError> {
    u64::try_from(value).map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("{context} does not fit u64: {source}"),
            )
        },
    )
}

/// Rejects non-container input before fixed-offset catalog reads can trust any
/// table offsets.
///
/// # Errors
///
/// Returns an error when the source prefix does not match the expected marker.
fn validate_magic(bytes: &[u8]) -> Result<(), ArchiveError> {
    let has_magic = bytes.get(..MAGIC.len()) == Some(MAGIC);
    let is_terminated = bytes
        .get(MAGIC.len())
        .copied()
        == Some(0);
    if !has_magic || !is_terminated {
        return Err(
            ArchiveError::invalid_archive(
                "missing RADCORE CEMENT LIBRARY magic",
            ),
        );
    }
    Ok(())
}

/// Rejects unsupported container revisions before table offsets are trusted.
///
/// # Errors
///
/// Returns an error when the file-info version differs from RCF 1.2.
fn validate_version(bytes: &[u8]) -> Result<(), ArchiveError> {
    let major = read_u8(
        bytes,
        FILE_INFO_VERSION_MAJOR_OFFSET,
        "major version",
    )?;
    let minor = read_u8(
        bytes,
        FILE_INFO_VERSION_MINOR_OFFSET,
        "minor version",
    )?;
    if major != FORMAT_VERSION_MAJOR || minor != FORMAT_VERSION_MINOR {
        return Err(
            ArchiveError::invalid_archive(
                format!("unsupported RCF version {major}.{minor}"),
            ),
        );
    }
    Ok(())
}

/// Rejects archives whose writer did not mark the full container complete.
///
/// # Errors
///
/// Returns an error when the file-info completion flag is not exactly one.
fn validate_completion(bytes: &[u8]) -> Result<(), ArchiveError> {
    let valid = read_u8(
        bytes,
        FILE_INFO_VALID_OFFSET,
        "valid flag",
    )?;
    if valid != 1 {
        return Err(
            ArchiveError::invalid_archive(
                format!("unsupported RCF valid flag: {valid}"),
            ),
        );
    }
    Ok(())
}

/// Rejects big-endian archives because every numeric decoder in this parser is
/// intentionally little-endian.
///
/// # Errors
///
/// Returns an error when the file-info header selects big-endian storage.
fn validate_endianness(bytes: &[u8]) -> Result<(), ArchiveError> {
    let big_endian = read_u8(
        bytes,
        FILE_INFO_BIG_ENDIAN_OFFSET,
        "big-endian flag",
    )?;
    if big_endian != 0 {
        return Err(
            ArchiveError::invalid_archive(
                format!("unsupported big-endian RCF flag: {big_endian}"),
            ),
        );
    }
    Ok(())
}

/// Reads one byte from a fixed file-info offset.
///
/// # Errors
///
/// Returns an error when the byte is outside the validated prefix.
fn read_u8(
    bytes: &[u8],
    offset: usize,
    field: &str,
) -> Result<u8, ArchiveError> {
    bytes
        .get(offset)
        .copied()
        .ok_or_else(
            || {
                ArchiveError::invalid_archive(
                    format!("{field} byte is outside the file info"),
                )
            },
        )
}

/// Centralizes little-endian word reads so every table access has the same
/// overflow and bounds behavior.
///
/// # Errors
///
/// Returns an error when the requested word extends beyond the supplied bytes.
fn read_u32(
    bytes: &[u8],
    offset: usize,
) -> Result<u32, ArchiveError> {
    let end = offset
        .checked_add(U32_BYTES)
        .ok_or_else(|| ArchiveError::invalid_archive("u32 read overflow"))?;
    let slice = bytes
        .get(offset..end)
        .ok_or_else(
            || {
                ArchiveError::invalid_archive(
                    format!("u32 read exceeds buffer at offset {offset}"),
                )
            },
        )?;
    Ok(
        u32::from_le_bytes(
            slice
                .try_into()
                .map_err(
                    |source| {
                        ArchiveError::invalid_archive(
                            format!(
                                "u32 slice length changed after bounds check: \
                                 {source}"
                            ),
                        )
                    },
                )?,
        ),
    )
}

/// Proves the index region fits the archive and precedes the name table.
///
/// # Errors
///
/// Returns an error when the index extends beyond either structural boundary.
fn validate_index_end(
    index_end: u64,
    archive_size: u64,
    name_table_offset: u32,
) -> Result<(), ArchiveError> {
    if index_end > archive_size {
        return Err(
            ArchiveError::invalid_archive(
                "index table extends beyond archive data",
            ),
        );
    }
    if index_end > u64::from(name_table_offset) {
        return Err(
            ArchiveError::invalid_archive(
                format!(
                    concat!(
                        "index end 0x{:x} exceeds ",
                        "name table 0x{:x}"
                    ),
                    index_end, name_table_offset
                ),
            ),
        );
    }
    Ok(())
}

/// Parses payload rows before names so payload ranges are proven safe before
/// extraction paths are considered.
///
/// # Errors
///
/// Returns an error when record offsets overflow or point outside the archive.
fn parse_records(
    bytes: &[u8],
    entry_count: usize,
    archive_size: u64,
) -> Result<Vec<IndexRecord>, ArchiveError> {
    let mut records = Vec::with_capacity(entry_count);
    for index in 0..entry_count {
        let offset = index
            .checked_mul(INDEX_RECORD_BYTES)
            .ok_or_else(
                || ArchiveError::invalid_archive("record offset overflow"),
            )?;
        let hash = read_u32(
            bytes, offset,
        )?;
        let payload_offset = u64::from(
            read_u32(
                bytes,
                offset
                    .checked_add(U32_BYTES)
                    .ok_or_else(
                        || {
                            ArchiveError::invalid_archive(
                                "record offset overflow",
                            )
                        },
                    )?,
            )?,
        );
        let length = u64::from(
            read_u32(
                bytes,
                offset
                    .checked_add(U32_BYTES.saturating_mul(2))
                    .ok_or_else(
                        || {
                            ArchiveError::invalid_archive(
                                "record offset overflow",
                            )
                        },
                    )?,
            )?,
        );
        let payload_end = payload_offset
            .checked_add(length)
            .ok_or_else(
                || {
                    ArchiveError::invalid_archive(
                        format!("entry {index} payload range overflows"),
                    )
                },
            )?;
        if payload_end > archive_size {
            return Err(
                ArchiveError::invalid_archive(
                    format!("entry {index} range exceeds archive size"),
                ),
            );
        }
        records.push(
            IndexRecord {
                hash,
                offset: payload_offset,
                length,
            },
        );
    }
    Ok(records)
}

/// Preserves the runtime binary-search precondition for hash index rows.
///
/// # Errors
///
/// Returns an error when archive rows are not ordered by increasing hash.
fn validate_hash_index_order(
    records: &[IndexRecord]
) -> Result<(), ArchiveError> {
    let mut previous_hash = 0_u32;
    let mut has_previous = false;
    for record in records {
        if has_previous && record.hash < previous_hash {
            return Err(
                ArchiveError::invalid_archive("hash index is not sorted"),
            );
        }
        previous_hash = record.hash;
        has_previous = true;
    }
    Ok(())
}

/// Rejects overlapping payload ranges because extraction writes each entry as
/// an independent file.
///
/// # Errors
///
/// Returns an error when an entry would reuse bytes claimed by a previous
/// entry.
fn validate_records_do_not_overlap(
    records: &[IndexRecord]
) -> Result<(), ArchiveError> {
    let mut previous_end = 0;
    for record in records {
        if record.length == 0 {
            continue;
        }
        if record.offset < previous_end {
            return Err(
                ArchiveError::invalid_archive(
                    format!(
                        "payload range overlaps previous entry at 0x{:x}",
                        record.offset
                    ),
                ),
            );
        }
        previous_end = record
            .offset
            .checked_add(record.length)
            .ok_or_else(
                || ArchiveError::invalid_archive("payload range end overflow"),
            )?;
    }
    Ok(())
}

/// Rejects payload pointers into the catalog, index, or complete name table.
///
/// # Errors
///
/// Returns an error when any payload begins before archive metadata ends.
fn validate_records_after_metadata(
    records: &[IndexRecord],
    metadata_end: u64,
) -> Result<(), ArchiveError> {
    if records
        .iter()
        .any(|record| record.offset < metadata_end)
    {
        return Err(
            ArchiveError::invalid_archive("payload overlaps archive metadata"),
        );
    }
    Ok(())
}

/// Validates the encoder's first-file hint without constraining payloads.
///
/// The original encoder computes `m_FirstFileStartPos` from the fixed
/// detailed-info structure size before any name row is measured, so shipped
/// archives routinely declare a hint inside the name-table region or below
/// the earliest payload row. The original runtime never reads the field, so
/// payload placement must be validated against the index records and the
/// metadata end, never against this hint. Only encoder invariants remain
/// checkable here: a nonempty archive stores a nonzero, in-range,
/// alignment-consistent hint.
///
/// # Errors
///
/// Returns an error when a nonempty archive stores a zero, out-of-range, or
/// misaligned first-file hint.
fn validate_first_file_hint(
    records: &[IndexRecord],
    first_file_offset: u64,
    alignment: u32,
    archive_size: u64,
) -> Result<(), ArchiveError> {
    if records.is_empty() {
        return Ok(());
    }
    if first_file_offset == 0 {
        return Err(ArchiveError::invalid_archive("first file hint is zero"));
    }
    if first_file_offset > archive_size {
        return Err(
            ArchiveError::invalid_archive(
                "first file hint extends beyond archive data",
            ),
        );
    }
    let is_aligned = !alignment.is_power_of_two()
        || first_file_offset.checked_rem(u64::from(alignment)) == Some(0);
    if !is_aligned {
        return Err(
            ArchiveError::invalid_archive("first file hint is not aligned"),
        );
    }
    Ok(())
}

/// Reads the name-table count through the same bounded word decoder.
fn read_name_count(
    reader: &mut dyn ArchiveByteReader,
    name_table_offset: u64,
) -> Result<usize, ArchiveError> {
    let count_bytes = reader.read_exact_range(
        name_table_offset,
        U32_BYTES_U64,
    )?;
    usize::try_from(
        read_u32(
            &count_bytes,
            0,
        )?,
    )
    .map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("name table count does not fit usize: {source}"),
            )
        },
    )
}

/// Parses one detailed file-information row.
///
/// # Errors
///
/// Returns an error when the length, name, timestamp, or decoded path is
/// invalid.
fn parse_detailed_name_record(
    reader: &mut dyn ArchiveByteReader,
    cursor: u64,
    index: usize,
    archive_size: u64,
) -> Result<DetailedNameRecord, ArchiveError> {
    let name_length_bytes = reader.read_exact_range(
        cursor,
        NAME_LENGTH_BYTES,
    )?;
    let name_len = usize::try_from(
        read_u32(
            &name_length_bytes,
            0,
        )?,
    )
    .map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("entry {index} name length invalid: {source}"),
            )
        },
    )?;
    let name_start = cursor
        .checked_add(NAME_LENGTH_BYTES)
        .ok_or_else(
            || ArchiveError::invalid_archive("name record cursor overflow"),
        )?;
    let name_len_u64 = u64::try_from(name_len).map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("name length does not fit u64: {source}"),
            )
        },
    )?;
    let name_end = name_start
        .checked_add(name_len_u64)
        .ok_or_else(
            || ArchiveError::invalid_archive("name range end overflow"),
        )?;
    if name_len == 0 || name_end > archive_size {
        return Err(
            ArchiveError::invalid_archive(
                format!("entry {index} name range is not valid"),
            ),
        );
    }
    let raw_name = reader.read_exact_range(
        name_start,
        name_len_u64,
    )?;
    let modification_time_bytes = reader.read_exact_range(
        name_end,
        MODIFICATION_TIME_BYTES,
    )?;
    let modification_time = read_u32(
        &modification_time_bytes,
        0,
    )?;
    let next_offset = name_end
        .checked_add(MODIFICATION_TIME_BYTES)
        .ok_or_else(
            || ArchiveError::invalid_archive("name record cursor overflow"),
        )?;
    Ok(
        DetailedNameRecord {
            modification_time,
            name: decode_name(
                &raw_name, index,
            )?,
            next_offset,
        },
    )
}

/// Parses names after the payload table so names cannot hide invalid payload
/// ranges.
///
/// # Errors
///
/// Returns an error when name counts, ranges, metadata, or decoding are
/// invalid.
fn parse_names(
    reader: &mut dyn ArchiveByteReader,
    name_table_offset: u64,
    expected_count: usize,
    archive_size: u64,
) -> Result<
    (
        Vec<(
            u32,
            String,
        )>,
        u64,
    ),
    ArchiveError,
> {
    let name_table_header_end = name_table_offset
        .checked_add(DETAILED_INFO_HEADER_BYTES)
        .ok_or_else(
            || ArchiveError::invalid_archive("name table header overflow"),
        )?;
    if name_table_header_end > archive_size {
        return Err(
            ArchiveError::invalid_archive(
                "name table begins beyond archive data",
            ),
        );
    }

    let actual_count = read_name_count(
        reader,
        name_table_offset,
    )?;
    if actual_count != expected_count {
        return Err(
            ArchiveError::invalid_archive(
                format!(
                    "name table count mismatch: expected {expected_count}, \
                     found {actual_count}"
                ),
            ),
        );
    }

    let mut names = Vec::with_capacity(expected_count);
    let mut cursor = name_table_header_end;
    for index in 0..expected_count {
        let record = parse_detailed_name_record(
            reader,
            cursor,
            index,
            archive_size,
        )?;
        cursor = record.next_offset;
        names.push(
            (
                record.modification_time,
                record.name,
            ),
        );
    }
    Ok(
        (
            names, cursor,
        ),
    )
}

/// Decodes a stored name only after stripping the container terminator that is
/// not part of the extraction path.
///
/// # Errors
///
/// Returns an error when the resulting name is empty or not UTF-8.
fn decode_name(
    raw_name: &[u8],
    index: usize,
) -> Result<String, ArchiveError> {
    let trimmed = raw_name
        .strip_suffix(&[0])
        .ok_or_else(
            || {
                ArchiveError::invalid_archive(
                    "entry name is not NUL-terminated",
                )
            },
        )?;
    if trimmed.is_empty() {
        return Err(
            ArchiveError::invalid_archive(
                format!("entry {index} decoded to an empty name"),
            ),
        );
    }
    if trimmed.contains(&0) {
        return Err(
            ArchiveError::invalid_archive(
                format!("entry {index} name contains an embedded NUL"),
            ),
        );
    }
    let decoded = std::str::from_utf8(trimmed).map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("entry {index} name is not UTF-8: {source}"),
            )
        },
    )?;
    if decoded
        .chars()
        .any(char::is_control)
    {
        return Err(
            ArchiveError::invalid_archive(
                format!("entry {index} name contains a control character"),
            ),
        );
    }
    if contains_unsafe_unicode_path_control(decoded) {
        return Err(
            ArchiveError::invalid_archive(
                format!("entry {index} name contains a Unicode path control"),
            ),
        );
    }
    Ok(decoded.to_owned())
}

/// Recomputes the lookup value so name records are matched by archive identity
/// instead of fragile table order.
fn name_hash32(text: &str) -> u32 {
    let mut value = 0_u32;
    for item in text.bytes() {
        let adjusted = if item < 97 {
            item.saturating_add(32)
        } else {
            item
        };
        value = value
            .wrapping_mul(31)
            .wrapping_add(u32::from(adjusted));
    }
    value
}

/// Normalizes stored separators so extraction produces the same relative
/// paths on every host platform.
fn normalize_output_name(text: &str) -> String {
    text.replace(
        '\\', "/",
    )
}

/// Joins names to payload rows by hash so mismatched tables fail before any
/// filesystem output.
///
/// # Errors
///
/// Returns an error when records and names cannot be paired one-to-one.
fn build_entries(
    records: Vec<IndexRecord>,
    names: Vec<(
        u32,
        String,
    )>,
) -> Result<Vec<ArchiveEntry>, ArchiveError> {
    let mut table = BTreeMap::new();
    for record in records {
        if table
            .insert(
                record.hash,
                record,
            )
            .is_some()
        {
            return Err(
                ArchiveError::invalid_archive("repeated archive lookup value"),
            );
        }
    }

    let mut entries = Vec::with_capacity(names.len());
    let mut output_identities = BTreeSet::new();
    for (modification_time, stored_name) in names {
        let lookup = name_hash32(&stored_name);
        let record = table
            .remove(&lookup)
            .ok_or_else(
                || {
                    ArchiveError::invalid_archive(
                        format!("missing archive lookup for {stored_name}"),
                    )
                },
            )?;
        let output_name = normalize_output_name(&stored_name);
        let output_identity = output_name.to_lowercase();
        if !output_identities.insert(output_identity) {
            return Err(
                ArchiveError::invalid_archive(
                    format!("duplicate output path: {output_name}"),
                ),
            );
        }
        entries.push(
            ArchiveEntry {
                name: output_name,
                hash: record.hash,
                offset: record.offset,
                length: record.length,
                modification_time,
            },
        );
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsupported_major_file_info_version() -> Result<(), String> {
        let mut bytes = vec![0_u8; FILE_INFO_VERSION_MINOR_OFFSET + 1];
        let Some(major) = bytes.get_mut(FILE_INFO_VERSION_MAJOR_OFFSET) else {
            return Err("major version fixture offset is invalid".to_owned());
        };
        *major = 2;
        let Some(minor) = bytes.get_mut(FILE_INFO_VERSION_MINOR_OFFSET) else {
            return Err("minor version fixture offset is invalid".to_owned());
        };
        *minor = FORMAT_VERSION_MINOR;

        let result = validate_version(&bytes);

        let Err(ArchiveError::InvalidArchive(message)) = result else {
            return Err("unsupported major version was accepted".to_owned());
        };
        if !message.contains("unsupported RCF version 2.2") {
            return Err(format!("unexpected version error: {message}"));
        }
        Ok(())
    }

    #[test]
    fn rejects_unsupported_minor_file_info_version() -> Result<(), String> {
        let mut bytes = vec![0_u8; FILE_INFO_VERSION_MINOR_OFFSET + 1];
        let Some(major) = bytes.get_mut(FILE_INFO_VERSION_MAJOR_OFFSET) else {
            return Err("major version fixture offset is invalid".to_owned());
        };
        *major = FORMAT_VERSION_MAJOR;
        let Some(minor) = bytes.get_mut(FILE_INFO_VERSION_MINOR_OFFSET) else {
            return Err("minor version fixture offset is invalid".to_owned());
        };
        *minor = 3;

        let result = validate_version(&bytes);

        let Err(ArchiveError::InvalidArchive(message)) = result else {
            return Err("unsupported minor version was accepted".to_owned());
        };
        if !message.contains("unsupported RCF version 1.3") {
            return Err(format!("unexpected version error: {message}"));
        }
        Ok(())
    }

    #[test]
    fn computes_original_cement_name_hashes() {
        assert_eq!(
            name_hash32(r"sound\scripts\knigh_v.spt"),
            0x062b_1126
        );
        assert_eq!(
            name_hash32(r"sound\scripts\ccube.spt"),
            0x0726_f620
        );
        assert_eq!(
            name_hash32(r"sound\scripts\csedan.spt"),
            0x0897_2da6
        );
    }
}
