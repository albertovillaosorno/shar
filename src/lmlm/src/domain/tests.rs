// File:
//   - tests.rs
// Path:
//   - src/lmlm/src/domain/tests.rs
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
//   - Deterministic regression fixtures for the LMLM parser boundary.
// - Must-Not:
//   - Read private archives or write extraction outputs.
// - Allows:
//   - Synthetic archive construction and public parser assertions.
// - Split-When:
//   - A stable parser sub-boundary gains an independently owned fixture set.
// - Merge-When:
//   - Another LMLM test module proves the same invariant without distinction.
// - Summary:
//   - Proves malformed LSPA containers fail closed before extraction.
// - Description:
//   - Defines synthetic fixtures and regression behavior for the LMLM parser.
// - Usage:
//   - Compiled only by the lmlm crate test target.
// - Defaults:
//   - Fixtures contain no proprietary package bytes.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Synthetic binary-layout helpers and malformed-input regressions
//   - share one test-only archive contract and no production responsibility.
//

//! Synthetic regression coverage for the public LMLM parser.

use super::layout::{BLOCK, FIRST_ENTRY, JEBANO_TITLE_LF, MAGIC, ROOT_BLOCK};
use super::{LmlmError, parse};

const ARCHIVE_LEN: usize = 0x4000;
const META_OFFSET: usize = 0x3000;
const TEST_ENTRY: usize = FIRST_ENTRY + BLOCK * 3;

fn copy_fixture_bytes(
    archive: &mut [u8],
    start: usize,
    bytes: &[u8],
) -> bool {
    let Some(end) = start.checked_add(bytes.len()) else {
        return false;
    };
    let Some(target) = archive.get_mut(start..end) else {
        return false;
    };
    target.copy_from_slice(bytes);
    true
}

fn with_crlf(bytes: &[u8]) -> Vec<u8> {
    let mut converted = Vec::with_capacity(
        bytes
            .len()
            .saturating_add(1),
    );
    for byte in bytes {
        if *byte == b'\n' {
            converted.extend_from_slice(b"\r\n");
        } else {
            converted.push(*byte);
        }
    }
    converted
}

fn write_file_entry(
    archive: &mut [u8],
    position: usize,
    name: &str,
    offset: u64,
    size: u64,
) -> Result<(), String> {
    if !copy_fixture_bytes(
        archive,
        position,
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture entry kind must fit".to_owned());
    }
    let mut encoded_name = Vec::new();
    for unit in name.encode_utf16() {
        encoded_name.extend_from_slice(&unit.to_le_bytes());
    }
    encoded_name.extend_from_slice(&0_u16.to_le_bytes());
    if !copy_fixture_bytes(
        archive,
        position.saturating_add(2),
        &encoded_name,
    ) {
        return Err("fixture entry name must fit".to_owned());
    }
    let metadata = position.saturating_add(BLOCK);
    if !copy_fixture_bytes(
        archive,
        metadata.saturating_add(0x0c),
        &size.to_le_bytes(),
    ) {
        return Err("fixture entry size must fit".to_owned());
    }
    if !copy_fixture_bytes(
        archive,
        metadata.saturating_add(0x14),
        &offset.to_le_bytes(),
    ) {
        return Err("fixture entry offset must fit".to_owned());
    }
    Ok(())
}

fn write_directory_entry(
    archive: &mut [u8],
    position: usize,
    name: &str,
    child_count: u16,
    contains_directory: bool,
) -> Result<(), String> {
    if !copy_fixture_bytes(
        archive,
        position,
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture directory kind must fit".to_owned());
    }
    let mut encoded_name = Vec::new();
    for unit in name.encode_utf16() {
        encoded_name.extend_from_slice(&unit.to_le_bytes());
    }
    encoded_name.extend_from_slice(&0_u16.to_le_bytes());
    if !copy_fixture_bytes(
        archive,
        position.saturating_add(2),
        &encoded_name,
    ) {
        return Err("fixture directory name must fit".to_owned());
    }
    let metadata = position.saturating_add(BLOCK);
    if !copy_fixture_bytes(
        archive,
        metadata.saturating_add(0x0c),
        &u64::from(child_count).to_le_bytes(),
    ) {
        return Err("fixture child count must fit".to_owned());
    }
    if !copy_fixture_bytes(
        archive,
        metadata.saturating_add(0x0e),
        &[u8::from(contains_directory)],
    ) {
        return Err("fixture directory control must fit".to_owned());
    }
    Ok(())
}

fn empty_archive_with(marker: &[u8]) -> Vec<u8> {
    let mut archive = vec![0_u8; ARCHIVE_LEN];
    assert!(
        copy_fixture_bytes(
            &mut archive,
            0,
            MAGIC
        ),
        "fixture magic range must fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut archive,
            4,
            &5_u32.to_le_bytes(),
        ),
        "fixture version range must fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut archive,
            0x0c,
            &0x0200_0000_u32.to_le_bytes(),
        ),
        "fixture header flags range must fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut archive,
            ROOT_BLOCK.saturating_add(2),
            &1_u16.to_le_bytes(),
        ),
        "fixture root-count range must fit"
    );
    assert!(
        write_file_entry(
            &mut archive,
            FIRST_ENTRY,
            "Meta.ini",
            u64::try_from(META_OFFSET).unwrap_or(u64::MAX),
            u64::try_from(marker.len()).unwrap_or(u64::MAX),
        )
        .is_ok(),
        "fixture metadata entry must fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut archive,
            META_OFFSET,
            marker
        ),
        "fixture metadata payload must fit"
    );
    archive
}

#[test]
fn truncated_magic_reports_truncation() -> Result<(), String> {
    match parse(b"LSP") {
        Err(LmlmError::Truncated) => Ok(()),
        other => Err(
            format!("truncated magic must report truncation, got {other:?}"),
        ),
    }
}

#[test]
fn wrong_magic_reports_the_observed_bytes() -> Result<(), String> {
    match parse(b"NOPE") {
        Err(LmlmError::BadMagic {
            observed,
        }) if observed == *b"NOPE" => Ok(()),
        other => Err(format!("wrong magic lost observed bytes: {other:?}")),
    }
}

#[test]
fn empty_container_reports_unsupported_package() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    let Some(unused) = archive.get_mut(FIRST_ENTRY..) else {
        return Err("fixture unused archive range must fit".to_owned());
    };
    unused.fill(0);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &0_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::UnsupportedPackage) => Ok(()),
        other => Err(
            format!("empty LSPA container must be unsupported, got {other:?}"),
        ),
    }
}

#[test]
fn rejects_nonzero_unclaimed_archive_bytes() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    let unclaimed_offset = 0x2000;
    if !copy_fixture_bytes(
        &mut archive,
        unclaimed_offset,
        &[1],
    ) {
        return Err("fixture unclaimed byte must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroUnclaimedData {
            offset,
            value: 1,
        }) if offset == unclaimed_offset => Ok(()),
        other => Err(
            format!("nonzero unclaimed archive byte must fail, got {other:?}"),
        ),
    }
}

#[test]
fn rejects_nonzero_trailing_archive_padding() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    let trailing_offset = archive
        .len()
        .checked_sub(1)
        .ok_or_else(|| "fixture archive must not be empty".to_owned())?;
    if !copy_fixture_bytes(
        &mut archive,
        trailing_offset,
        &[1],
    ) {
        return Err("fixture trailing byte must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroTrailingData {
            offset,
            value: 1,
        }) if offset == trailing_offset => Ok(()),
        other => Err(
            format!("nonzero trailing archive byte must fail, got {other:?}"),
        ),
    }
}

#[test]
fn rejects_payloads_inside_the_entry_table() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "table.bin",
        0x800,
        1,
    )?;
    match parse(&archive) {
        Err(LmlmError::EntryPayloadOverlapsTable {
            path,
            offset: 0x800,
            table_end: 0x1000,
        }) if path == "table.bin" => Ok(()),
        other => {
            Err(format!("payload inside entry table must fail, got {other:?}"))
        }
    }
}

#[test]
fn accepts_final_payload_immediately_after_metadata() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    let payload_offset = TEST_ENTRY.saturating_add(BLOCK.saturating_mul(2));
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "inline-final.bin",
        u64::try_from(payload_offset).unwrap_or(u64::MAX),
        1,
    )?;
    if !copy_fixture_bytes(
        &mut archive,
        payload_offset,
        &[1],
    ) {
        return Err("fixture final payload must fit".to_owned());
    }
    match parse(&archive) {
        Ok(entries)
            if entries
                .iter()
                .any(|entry| entry.path == "inline-final.bin") =>
        {
            Ok(())
        }
        other => Err(
            format!(
                "final payload may replace its transition block: {other:?}"
            ),
        ),
    }
}

#[test]
fn rejects_unaligned_payload_offsets() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "unaligned.bin",
        0x3201,
        1,
    )?;
    match parse(&archive) {
        Err(LmlmError::UnalignedEntryOffset {
            path,
            offset: 0x3201,
        }) if path == "unaligned.bin" => Ok(()),
        other => {
            Err(format!("unaligned payload offset must fail, got {other:?}"))
        }
    }
}

#[test]
fn accepts_supported_file_record_control_flag() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    let control_offset = FIRST_ENTRY.saturating_add(BLOCK.saturating_mul(2));
    if !copy_fixture_bytes(
        &mut archive,
        control_offset,
        &[1],
    ) {
        return Err("fixture file transition control must fit".to_owned());
    }
    parse(&archive)
        .map(|_| ())
        .map_err(
            |error| {
                format!("supported file transition control failed: {error}")
            },
        )
}

#[test]
fn rejects_unsupported_file_record_control() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    let control_offset = FIRST_ENTRY.saturating_add(BLOCK.saturating_mul(2));
    if !copy_fixture_bytes(
        &mut archive,
        control_offset,
        &[2],
    ) {
        return Err("fixture file transition control must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::UnsupportedFileRecordControl {
            offset,
            value: 2,
        }) if offset == control_offset => Ok(()),
        other => Err(
            format!("unsupported file transition control must fail: {other:?}"),
        ),
    }
}

#[test]
fn rejects_nonzero_file_record_control_padding() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    let padding_offset = FIRST_ENTRY
        .saturating_add(BLOCK.saturating_mul(2))
        .saturating_add(1);
    if !copy_fixture_bytes(
        &mut archive,
        padding_offset,
        &[1],
    ) {
        return Err("fixture file transition padding must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroFileRecordPadding {
            offset,
            value: 1,
        }) if offset == padding_offset => Ok(()),
        other => {
            Err(format!("nonzero file transition padding must fail: {other:?}"))
        }
    }
}

#[test]
fn accepts_directory_control_for_immediate_subdirectory() -> Result<(), String>
{
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_directory_entry(
        &mut archive,
        TEST_ENTRY,
        "parent",
        1,
        true,
    )?;
    write_directory_entry(
        &mut archive,
        TEST_ENTRY.saturating_add(BLOCK.saturating_mul(2)),
        "child",
        0,
        false,
    )?;
    parse(&archive)
        .map(|_| ())
        .map_err(
            |error| format!("directory child-kind control failed: {error}"),
        )
}

#[test]
fn rejects_unsupported_directory_record_control() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_directory_entry(
        &mut archive,
        TEST_ENTRY,
        "empty",
        0,
        false,
    )?;
    let control_offset = TEST_ENTRY
        .saturating_add(BLOCK)
        .saturating_add(0x0e);
    if !copy_fixture_bytes(
        &mut archive,
        control_offset,
        &[2],
    ) {
        return Err("fixture directory control must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::UnsupportedDirectoryRecordControl {
            offset,
            value: 2,
        }) if offset == control_offset => Ok(()),
        other => {
            Err(format!("unsupported directory control must fail: {other:?}"))
        }
    }
}

#[test]
fn rejects_nonzero_directory_record_padding() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_directory_entry(
        &mut archive,
        TEST_ENTRY,
        "empty",
        0,
        false,
    )?;
    let padding_offset = TEST_ENTRY
        .saturating_add(BLOCK)
        .saturating_add(0x0f);
    if !copy_fixture_bytes(
        &mut archive,
        padding_offset,
        &[1],
    ) {
        return Err("fixture directory padding must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroMetadataPadding {
            offset,
            value: 1,
        }) if offset == padding_offset => Ok(()),
        other => Err(format!("nonzero directory padding must fail: {other:?}")),
    }
}

#[test]
fn rejects_directory_control_missing_subdirectory() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_directory_entry(
        &mut archive,
        TEST_ENTRY,
        "parent",
        1,
        false,
    )?;
    write_directory_entry(
        &mut archive,
        TEST_ENTRY.saturating_add(BLOCK.saturating_mul(2)),
        "child",
        0,
        false,
    )?;
    let control_offset = TEST_ENTRY
        .saturating_add(BLOCK)
        .saturating_add(0x0e);
    match parse(&archive) {
        Err(LmlmError::DirectoryRecordControlMismatch {
            path,
            offset,
            declared: 0,
            expected: 1,
        }) if path == "parent" && offset == control_offset => Ok(()),
        other => {
            Err(format!("missing subdirectory control must fail: {other:?}"))
        }
    }
}

#[test]
fn rejects_directory_control_without_subdirectory() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_directory_entry(
        &mut archive,
        TEST_ENTRY,
        "parent",
        1,
        true,
    )?;
    write_file_entry(
        &mut archive,
        TEST_ENTRY.saturating_add(BLOCK.saturating_mul(2)),
        "child.bin",
        0x3200,
        1,
    )?;
    let control_offset = TEST_ENTRY
        .saturating_add(BLOCK)
        .saturating_add(0x0e);
    match parse(&archive) {
        Err(LmlmError::DirectoryRecordControlMismatch {
            path,
            offset,
            declared: 1,
            expected: 0,
        }) if path == "parent" && offset == control_offset => Ok(()),
        other => {
            Err(format!("spurious subdirectory control must fail: {other:?}"))
        }
    }
}

#[test]
fn rejects_nonzero_reserved_metadata_bytes() -> Result<(), String> {
    for relative_offset in [
        0, 0x0b,
    ] {
        let mut archive = empty_archive_with(JEBANO_TITLE_LF);
        let metadata_offset = FIRST_ENTRY
            .saturating_add(BLOCK)
            .saturating_add(relative_offset);
        if !copy_fixture_bytes(
            &mut archive,
            metadata_offset,
            &[1],
        ) {
            return Err("fixture reserved metadata byte must fit".to_owned());
        }
        match parse(&archive) {
            Err(LmlmError::NonZeroMetadataPadding {
                offset,
                value: 1,
            }) if offset == metadata_offset => {}
            other => {
                return Err(
                    format!(
                        "reserved byte {relative_offset:#x} must fail, got \
                         {other:?}"
                    ),
                );
            }
        }
    }
    Ok(())
}

#[test]
fn rejects_nonzero_metadata_padding() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "metadata.bin",
        0x3200,
        1,
    )?;
    let padding_offset = TEST_ENTRY
        .saturating_add(BLOCK)
        .saturating_add(0x1c);
    if !copy_fixture_bytes(
        &mut archive,
        padding_offset,
        &[1],
    ) {
        return Err("fixture metadata padding byte must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroMetadataPadding {
            offset,
            value: 1,
        }) if offset == padding_offset => Ok(()),
        other => {
            Err(format!("nonzero metadata padding must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_nonzero_name_padding() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "pad.bin",
        0x3200,
        1,
    )?;
    let padding_offset = TEST_ENTRY.saturating_add(18);
    if !copy_fixture_bytes(
        &mut archive,
        padding_offset,
        &[1],
    ) {
        return Err("fixture name padding byte must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroNamePadding {
            offset,
            value: 1,
        }) if offset == padding_offset => Ok(()),
        other => Err(format!("nonzero name padding must fail, got {other:?}")),
    }
}

#[test]
fn rejects_unterminated_entry_names() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        TEST_ENTRY,
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture entry kind must fit".to_owned());
    }
    let name_bytes = vec![b'A'; BLOCK.saturating_sub(2)];
    if !copy_fixture_bytes(
        &mut archive,
        TEST_ENTRY.saturating_add(2),
        &name_bytes,
    ) {
        return Err("fixture unterminated name must fit".to_owned());
    }
    let metadata = TEST_ENTRY.saturating_add(BLOCK);
    if !copy_fixture_bytes(
        &mut archive,
        metadata.saturating_add(0x0c),
        &1_u64.to_le_bytes(),
    ) {
        return Err("fixture entry size must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        metadata.saturating_add(0x14),
        &0x3200_u64.to_le_bytes(),
    ) {
        return Err("fixture entry offset must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::UnterminatedName {
            offset: TEST_ENTRY,
        }) => Ok(()),
        other => {
            Err(format!("unterminated entry name must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_invalid_entry_kind() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        FIRST_ENTRY,
        &3_u16.to_le_bytes(),
    ) {
        return Err("fixture entry kind must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::InvalidEntryKind {
            offset: FIRST_ENTRY,
            value: 3,
        }) => Ok(()),
        other => Err(format!("invalid entry kind must fail, got {other:?}")),
    }
}

#[test]
fn rejects_nonzero_reserved_root_bytes() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK,
        &[1],
    ) {
        return Err("fixture reserved root byte must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroReservedRootBlock {
            offset,
            value: 1,
        }) if offset == ROOT_BLOCK => Ok(()),
        other => {
            Err(format!("nonzero reserved root byte must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_nonzero_reserved_container_block() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        BLOCK,
        &[1],
    ) {
        return Err("fixture reserved block byte must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroReservedContainerBlock {
            offset,
            value: 1,
        }) if offset == BLOCK => Ok(()),
        other => Err(
            format!(
                "nonzero reserved container block must fail, got {other:?}"
            ),
        ),
    }
}

#[test]
fn rejects_nonzero_reserved_header_bytes() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        8,
        &[1],
    ) {
        return Err("fixture reserved header byte must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::NonZeroReservedHeader {
            offset: 8,
            value: 1,
        }) => Ok(()),
        other => Err(
            format!("nonzero reserved header byte must fail, got {other:?}"),
        ),
    }
}

#[test]
fn rejects_unsupported_header_flags() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        0x0c,
        &0_u32.to_le_bytes(),
    ) {
        return Err("fixture header flags range must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::UnsupportedHeaderFlags {
            offset: 0x0c,
            value: 0,
        }) => Ok(()),
        other => {
            Err(format!("unsupported header flags must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_unsupported_archive_version() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        4,
        &4_u32.to_le_bytes(),
    ) {
        return Err("fixture version range must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::UnsupportedVersion {
            offset: 4,
            value: 4,
        }) => Ok(()),
        other => {
            Err(format!("unsupported archive version must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_excessive_directory_nesting() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    archive.resize(
        0x22000, 0,
    );
    let meta_metadata = FIRST_ENTRY.saturating_add(BLOCK);
    if !copy_fixture_bytes(
        &mut archive,
        meta_metadata.saturating_add(0x14),
        &0x20000_u64.to_le_bytes(),
    ) {
        return Err("fixture metadata offset must fit".to_owned());
    }
    let old_meta_end = META_OFFSET.saturating_add(JEBANO_TITLE_LF.len());
    let Some(old_meta) = archive.get_mut(META_OFFSET..old_meta_end) else {
        return Err("fixture old metadata range must fit".to_owned());
    };
    old_meta.fill(0);
    if !copy_fixture_bytes(
        &mut archive,
        0x20000,
        JEBANO_TITLE_LF,
    ) {
        return Err("fixture relocated metadata must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    let mut position = TEST_ENTRY;
    for index in 0_u8..65_u8 {
        write_directory_entry(
            &mut archive,
            position,
            "d",
            1,
            index < 64,
        )?;
        position = position.saturating_add(BLOCK.saturating_mul(2));
    }
    write_file_entry(
        &mut archive,
        position,
        "leaf.bin",
        0x20200,
        1,
    )?;
    match parse(&archive) {
        Err(LmlmError::ExcessiveDirectoryDepth {
            depth: 65,
            ..
        }) => Ok(()),
        other => {
            Err(format!("excessive directory nesting must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_overlong_portable_paths() -> Result<(), String> {
    let directory = "d".repeat(130);
    let file = "f".repeat(130);
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_directory_entry(
        &mut archive,
        TEST_ENTRY,
        &directory,
        1,
        false,
    )?;
    write_file_entry(
        &mut archive,
        TEST_ENTRY.saturating_add(BLOCK.saturating_mul(2)),
        &file,
        0x3200,
        1,
    )?;
    let expected = format!("{directory}/{file}");
    match parse(&archive) {
        Err(LmlmError::UnsafePath(path)) if path == expected => Ok(()),
        other => {
            Err(format!("overlong portable path must fail, got {other:?}"))
        }
    }
}

#[test]
fn accepts_portable_unicode_paths_with_long_utf8_encodings()
-> Result<(), String> {
    let directory = "é".repeat(70);
    let file = "é".repeat(70);
    let expected = format!("{directory}/{file}");
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_directory_entry(
        &mut archive,
        TEST_ENTRY,
        &directory,
        1,
        false,
    )?;
    write_file_entry(
        &mut archive,
        TEST_ENTRY.saturating_add(BLOCK.saturating_mul(2)),
        &file,
        0x3200,
        1,
    )?;
    match parse(&archive) {
        Ok(entries)
            if entries
                .iter()
                .any(|entry| entry.path == expected) =>
        {
            Ok(())
        }
        other => {
            Err(format!("portable Unicode path must parse, got {other:?}"))
        }
    }
}

#[test]
fn accepts_astral_unicode_components_within_the_utf16_limit()
-> Result<(), String> {
    let name = "😀".repeat(64);
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        &name,
        0x3200,
        1,
    )?;
    match parse(&archive) {
        Ok(entries)
            if entries
                .iter()
                .any(|entry| entry.path == name) =>
        {
            Ok(())
        }
        other => Err(
            format!(
                "astral Unicode component within the UTF-16 limit must parse, \
                 got {other:?}"
            ),
        ),
    }
}

#[test]
fn rejects_unicode_path_modifiers() -> Result<(), String> {
    let name = "report\u{202e}cod.exe";
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        name,
        0x3200,
        1,
    )?;
    match parse(&archive) {
        Err(LmlmError::UnsafePath(path)) if path == name => Ok(()),
        other => Err(format!("Unicode path modifier must fail, got {other:?}")),
    }
}

#[test]
fn rejects_extended_windows_reserved_aliases() -> Result<(), String> {
    for name in [
        "CONIN$",
        "CONOUT$",
        "COM¹.txt",
        "LPT².log",
        "CON .txt",
        "AUX..txt",
    ] {
        let mut archive = empty_archive_with(JEBANO_TITLE_LF);
        if !copy_fixture_bytes(
            &mut archive,
            ROOT_BLOCK.saturating_add(2),
            &2_u16.to_le_bytes(),
        ) {
            return Err("fixture root-count range must fit".to_owned());
        }
        write_file_entry(
            &mut archive,
            TEST_ENTRY,
            name,
            0x3200,
            1,
        )?;
        match parse(&archive) {
            Err(LmlmError::UnsafePath(path)) if path == name => {}
            other => {
                return Err(
                    format!(
                        "Windows reserved alias {name:?} must fail, got \
                         {other:?}"
                    ),
                );
            }
        }
    }
    Ok(())
}

#[test]
fn rejects_nonportable_entry_names() -> Result<(), String> {
    for name in [
        "CON",
        "aux.txt",
        "bad:name.bin",
        "bad?.bin",
        "trailing.",
        "trailing ",
        "control\u{1}.bin",
    ] {
        let mut archive = empty_archive_with(JEBANO_TITLE_LF);
        if !copy_fixture_bytes(
            &mut archive,
            ROOT_BLOCK.saturating_add(2),
            &2_u16.to_le_bytes(),
        ) {
            return Err("fixture root-count range must fit".to_owned());
        }
        write_file_entry(
            &mut archive,
            TEST_ENTRY,
            name,
            0x3200,
            1,
        )?;
        match parse(&archive) {
            Err(LmlmError::UnsafePath(path)) if path == name => {}
            other => {
                return Err(
                    format!(
                        "nonportable entry name {name:?} must fail, got \
                         {other:?}"
                    ),
                );
            }
        }
    }
    Ok(())
}

#[test]
fn rejects_portable_path_collisions() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &3_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "Shared.bin",
        0x3200,
        0x200,
    )?;
    write_file_entry(
        &mut archive,
        TEST_ENTRY.saturating_add(BLOCK.saturating_mul(3)),
        "shared.bin",
        0x3400,
        0x200,
    )?;
    match parse(&archive) {
        Err(LmlmError::PathCollision {
            ..
        }) => Ok(()),
        other => {
            Err(format!("colliding archive paths must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_unicode_case_equivalent_path_collisions() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &3_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "Σ.bin",
        0x3200,
        0x200,
    )?;
    write_file_entry(
        &mut archive,
        TEST_ENTRY.saturating_add(BLOCK.saturating_mul(3)),
        "ς.bin",
        0x3400,
        0x200,
    )?;
    match parse(&archive) {
        Err(LmlmError::PathCollision {
            ..
        }) => Ok(()),
        other => Err(
            format!(
                "Unicode case-equivalent archive paths must fail, got \
                 {other:?}"
            ),
        ),
    }
}

#[test]
fn rejects_overlapping_payload_ranges() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &3_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "first.bin",
        0x3200,
        0x300,
    )?;
    write_file_entry(
        &mut archive,
        TEST_ENTRY.saturating_add(BLOCK.saturating_mul(3)),
        "second.bin",
        0x3400,
        0x200,
    )?;
    match parse(&archive) {
        Err(LmlmError::OverlappingEntryRanges {
            first_offset: 0x3200,
            first_size: 0x300,
            second_offset: 0x3400,
            second_size: 0x200,
            ..
        }) => Ok(()),
        other => {
            Err(format!("overlapping payload ranges must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_payload_past_archive_end() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        TEST_ENTRY,
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture entry kind must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        TEST_ENTRY.saturating_add(2),
        &[
            b'b', 0, b'a', 0, b'd', 0, b'.', 0, b'b', 0, b'i', 0, b'n', 0, 0, 0,
        ],
    ) {
        return Err("fixture name range must fit".to_owned());
    }
    let metadata = TEST_ENTRY.saturating_add(BLOCK);
    if !copy_fixture_bytes(
        &mut archive,
        metadata.saturating_add(0x0c),
        &0x200_u64.to_le_bytes(),
    ) {
        return Err("fixture data size must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        metadata.saturating_add(0x14),
        &0x4000_u64.to_le_bytes(),
    ) {
        return Err("fixture data offset must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::InvalidEntryRange {
            ..
        }) => Ok(()),
        other => Err(format!("out-of-bounds payload must fail, got {other:?}")),
    }
}

#[test]
fn rejects_invalid_utf16_entry_name() -> Result<(), String> {
    let mut archive = empty_archive_with(JEBANO_TITLE_LF);
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        TEST_ENTRY,
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture entry kind must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        TEST_ENTRY.saturating_add(2),
        &[
            0x00, 0xd8, 0x00, 0x00,
        ],
    ) {
        return Err("fixture name range must fit".to_owned());
    }
    let metadata = TEST_ENTRY.saturating_add(BLOCK);
    if !copy_fixture_bytes(
        &mut archive,
        metadata.saturating_add(0x0c),
        &1_u64.to_le_bytes(),
    ) {
        return Err("fixture data size must fit".to_owned());
    }
    if !copy_fixture_bytes(
        &mut archive,
        metadata.saturating_add(0x14),
        &0x3200_u64.to_le_bytes(),
    ) {
        return Err("fixture data offset must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::InvalidNameEncoding {
            offset,
            message,
        }) if offset == TEST_ENTRY => {
            let diagnostic = LmlmError::InvalidNameEncoding {
                offset,
                message,
            }
            .to_string();
            let expected_offset = format!("{TEST_ENTRY:#x}");
            if diagnostic.contains(&expected_offset) {
                Ok(())
            } else {
                Err(
                    format!(
                        "invalid UTF-16 diagnostic lost entry offset: \
                         {diagnostic:?}"
                    ),
                )
            }
        }
        other => Err(format!("invalid UTF-16 name must fail, got {other:?}")),
    }
}

#[test]
fn accepts_exact_jebano_title_with_lf_or_crlf() {
    assert!(parse(&empty_archive_with(JEBANO_TITLE_LF)).is_ok());
    assert!(parse(&empty_archive_with(&with_crlf(JEBANO_TITLE_LF))).is_ok());
}

#[test]
fn rejects_package_title_prefix_spoofs() {
    let mut marker = JEBANO_TITLE_LF.to_vec();
    marker.extend_from_slice(
        b"Evil
",
    );
    let archive = empty_archive_with(&marker);
    assert!(
        matches!(
            parse(&archive),
            Err(LmlmError::UnsupportedPackage)
        )
    );
}

#[test]
fn rejects_conflicting_duplicate_package_titles() {
    let mut metadata = JEBANO_TITLE_LF.to_vec();
    metadata.extend_from_slice(b"\nTitle=Different Package");
    let archive = empty_archive_with(&metadata);
    assert!(
        matches!(
            parse(&archive),
            Err(LmlmError::UnsupportedPackage)
        )
    );
}

#[test]
fn rejects_title_marker_outside_metadata_entry() -> Result<(), String> {
    let mut archive = empty_archive_with(
        b"[Miscellaneous]
Title=Another Mod",
    );
    if !copy_fixture_bytes(
        &mut archive,
        ROOT_BLOCK.saturating_add(2),
        &2_u16.to_le_bytes(),
    ) {
        return Err("fixture root-count range must fit".to_owned());
    }
    write_file_entry(
        &mut archive,
        TEST_ENTRY,
        "decoy.bin",
        0x3200,
        u64::try_from(JEBANO_TITLE_LF.len()).unwrap_or(u64::MAX),
    )?;
    if !copy_fixture_bytes(
        &mut archive,
        0x3200,
        JEBANO_TITLE_LF,
    ) {
        return Err("fixture decoy payload must fit".to_owned());
    }
    match parse(&archive) {
        Err(LmlmError::UnsupportedPackage) => Ok(()),
        other => {
            Err(format!("non-metadata package marker must fail, got {other:?}"))
        }
    }
}

#[test]
fn rejects_other_lspa_packages() {
    let archive = empty_archive_with(
        b"[Miscellaneous]
Title=Another Mod",
    );
    assert!(
        matches!(
            parse(&archive),
            Err(LmlmError::UnsupportedPackage)
        )
    );
}
