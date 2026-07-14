// File:
//   - declared_header_offset.rs
// Path:
//   - src/rcf/tests/declared_header_offset.rs
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
//   - The caller-visible declared RCF catalog-offset regression.
// - Must-Not:
//   - Read private assets or assert parser implementation details.
// - Allows:
//   - Minimal synthetic archive bytes and public parser assertions.
// - Split-When:
//   - Another independent catalog-location contract is added.
// - Merge-When:
//   - Declared catalog offsets no longer need a focused test target.
// - Summary:
//   - Protects non-default RCF catalog placement.
// - Description:
//   - Verifies that the public parser honors the file-info header position.
// - Usage:
//   - Run through the RCF crate integration test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Caller-visible regression for declared RCF catalog placement.
//!
//! A minimal synthetic archive relocates its catalog while preserving every
//! public container contract, proving that parsing follows the file-info word.

use rcf::ArchiveParser;
use rcf::domain::ArchiveError;
use rcf::ports::ArchiveByteReader;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

const ARCHIVE_LENGTH: usize = 0x1801;
const ARCHIVE_LENGTH_U64: u64 = 0x1801;
const CATALOG_OFFSET: usize = 0x1000;
const CATALOG_OFFSET_U32: u32 = 0x1000;
const CATALOG_NAME_TABLE_FIELD: usize = 0x1004;
const CATALOG_PAYLOAD_FIELD: usize = 0x1008;
const INDEX_OFFSET: usize = 0x1010;
const INDEX_PAYLOAD_FIELD: usize = 0x1014;
const INDEX_LENGTH_FIELD: usize = 0x1018;
const NAME_TABLE_OFFSET: usize = 0x101c;
const NAME_TABLE_OFFSET_U32: u32 = 0x101c;
const NAME_LENGTH_OFFSET: usize = 0x1024;
const NAME_BYTES_OFFSET: usize = 0x1028;
const PAYLOAD_OFFSET: usize = 0x1800;
const PAYLOAD_OFFSET_U32: u32 = 0x1800;
const FILE_INFO_ALIGNMENT_OFFSET: usize = 36;
const FILE_INFO_HEADER_OFFSET: usize = 44;
const FILE_INFO_VALID_OFFSET: usize = 35;
const FILE_INFO_VERSION_MAJOR_OFFSET: usize = 32;
const FILE_INFO_VERSION_MINOR_OFFSET: usize = 33;
const MAGIC: &[u8] = b"RADCORE CEMENT LIBRARY";
const RAW_NAME: &[u8] = b"sound/file.rsd\0";
const RAW_NAME_HASH: u32 = 0x61db_3fd1;

#[test]
fn reads_catalog_from_declared_header_offset() {
    let fixture = build_archive();
    assert!(
        fixture.is_ok(),
        "the relocated archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let mut reader = MemoryReader(bytes);
    let result = ArchiveParser::from_reader(&mut reader);

    assert!(
        result.is_ok(),
        "the parser must read the catalog from m_HeaderStartPos"
    );
    let Ok(archive) = result else {
        return;
    };
    let entries = archive.entries;
    assert_eq!(
        entries.len(),
        1,
        "the relocated archive must retain its entry"
    );
    let entry_slice = entries.as_slice();
    let [first_entry] = entry_slice else {
        return;
    };
    let rcf::domain::ArchiveEntry {
        name,
        ..
    } = first_entry;
    assert_eq!(
        name,
        "sound/file.rsd"
    );
}

fn build_archive() -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = vec![0_u8; ARCHIVE_LENGTH];
    write_bytes(
        &mut bytes, 0, MAGIC,
    )?;
    write_byte(
        &mut bytes,
        FILE_INFO_VERSION_MAJOR_OFFSET,
        1,
    )?;
    write_byte(
        &mut bytes,
        FILE_INFO_VERSION_MINOR_OFFSET,
        2,
    )?;
    write_byte(
        &mut bytes,
        FILE_INFO_VALID_OFFSET,
        1,
    )?;
    write_u32(
        &mut bytes,
        FILE_INFO_ALIGNMENT_OFFSET,
        0x800,
    )?;
    write_u32(
        &mut bytes,
        FILE_INFO_HEADER_OFFSET,
        CATALOG_OFFSET_U32,
    )?;
    write_u32(
        &mut bytes,
        CATALOG_OFFSET,
        1,
    )?;
    write_u32(
        &mut bytes,
        CATALOG_NAME_TABLE_FIELD,
        NAME_TABLE_OFFSET_U32,
    )?;
    write_u32(
        &mut bytes,
        CATALOG_PAYLOAD_FIELD,
        PAYLOAD_OFFSET_U32,
    )?;
    write_u32(
        &mut bytes,
        INDEX_OFFSET,
        RAW_NAME_HASH,
    )?;
    write_u32(
        &mut bytes,
        INDEX_PAYLOAD_FIELD,
        PAYLOAD_OFFSET_U32,
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
    let raw_name_length = u32::try_from(RAW_NAME.len()).map_err(
        |source| {
            ArchiveError::invalid_archive(
                format!("fixture name length does not fit u32: {source}"),
            )
        },
    )?;
    write_u32(
        &mut bytes,
        NAME_LENGTH_OFFSET,
        raw_name_length,
    )?;
    write_bytes(
        &mut bytes,
        NAME_BYTES_OFFSET,
        RAW_NAME,
    )?;
    write_byte(
        &mut bytes,
        PAYLOAD_OFFSET,
        1,
    )?;
    Ok(bytes)
}

fn write_byte(
    bytes: &mut [u8],
    offset: usize,
    value: u8,
) -> Result<(), ArchiveError> {
    let target = bytes
        .get_mut(offset)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture byte offset is invalid"),
        )?;
    *target = value;
    Ok(())
}

fn write_u32(
    bytes: &mut [u8],
    offset: usize,
    value: u32,
) -> Result<(), ArchiveError> {
    write_bytes(
        bytes,
        offset,
        &value.to_le_bytes(),
    )
}

fn write_bytes(
    bytes: &mut [u8],
    offset: usize,
    value: &[u8],
) -> Result<(), ArchiveError> {
    let end = offset
        .checked_add(value.len())
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture range overflow"),
        )?;
    let target = bytes
        .get_mut(offset..end)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture range is invalid"),
        )?;
    target.copy_from_slice(value);
    Ok(())
}

struct MemoryReader(Vec<u8>);

impl ArchiveByteReader for MemoryReader {
    fn len(&self) -> Result<u64, ArchiveError> {
        Ok(ARCHIVE_LENGTH_U64)
    }

    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ArchiveError> {
        let Ok(start) = usize::try_from(offset) else {
            return Err(
                ArchiveError::invalid_archive(
                    "fixture offset does not fit usize",
                ),
            );
        };
        let Ok(count) = usize::try_from(length) else {
            return Err(
                ArchiveError::invalid_archive(
                    "fixture length does not fit usize",
                ),
            );
        };
        let Some(end) = start.checked_add(count) else {
            return Err(
                ArchiveError::invalid_archive("fixture range overflow"),
            );
        };
        let bytes = &self.0;
        let Some(range) = bytes.get(start..end) else {
            return Err(
                ArchiveError::invalid_archive("fixture range exceeds bytes"),
            );
        };
        Ok(range.to_owned())
    }
}
