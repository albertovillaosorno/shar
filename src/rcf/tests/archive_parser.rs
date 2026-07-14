// File:
//   - archive_parser.rs
// Path:
//   - src/rcf/tests/archive_parser.rs
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
//   - Caller-visible RCF parser regressions.
// - Must-Not:
//   - Read private assets or assert parser implementation details.
// - Allows:
//   - Synthetic archive rows and public parser assertions.
// - Split-When:
//   - A non-parser use case needs an independent test target.
// - Merge-When:
//   - Parser regressions no longer need a distinct integration boundary.
// - Summary:
//   - Protects RCF archive parsing contracts.
// - Description:
//   - Exercises public parsing with synthetic malformed archive evidence.
// - Usage:
//   - Run through the RCF crate integration test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Public parser regressions remain grouped by one synthetic archive
//   - boundary while focused fixture construction stays in fixture/archive.rs.
//

//! Caller-visible regressions for the RCF parser.
//!
//! Synthetic archives verify failures through the public parser boundary.

use schoenwald_cli as _;
use schoenwald_filesystem as _;
#[path = "fixture/archive.rs"]
mod fixture;

use fixture::{
    archive_with_alignment_five, archive_with_big_endian_flag,
    archive_with_first_file_offset, archive_with_header_overlap,
    archive_with_index_beyond_declared_length, archive_with_magic_suffix,
    archive_with_modification_time, archive_with_multi_first_file_offset,
    archive_with_payload_inside_modification_time,
    archive_with_payload_inside_name_table, archive_with_stored_name,
    archive_with_stored_names, archive_with_unaligned_header,
    archive_with_valid_flag, parse_archive, parse_archive_with_declared_length,
};
use rcf::ArchiveError;

#[test]
fn rejects_non_terminated_magic_identifier() {
    let fixture = archive_with_magic_suffix(b'X');
    assert!(
        fixture.is_ok(),
        "the malformed magic fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("magic")
        ),
        "the runtime strcmp contract requires a terminated identifier"
    );
}

#[test]
fn accepts_encoder_non_power_of_two_alignment() {
    let fixture = archive_with_alignment_five();
    assert!(
        fixture.is_ok(),
        "the encoder-compatible alignment fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        result.is_ok(),
        "the encoder accepts alignment five for a header at byte 48"
    );
}

#[test]
fn accepts_aligned_first_file_hint_before_actual_payload() {
    let fixture = archive_with_first_file_offset(0x800);
    assert!(
        fixture.is_ok(),
        "the early first-file hint fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        result.is_ok(),
        "the encoder hint may precede the earliest indexed payload"
    );
}

#[test]
fn accepts_aligned_first_file_hint_inside_name_metadata() {
    let fixture = archive_with_multi_first_file_offset(0x1000);
    assert!(
        fixture.is_ok(),
        "the metadata-overlap hint fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        result.is_ok(),
        "the encoder hint may identify the name-table region"
    );
}

#[test]
fn rejects_zero_first_file_hint_for_nonempty_archive() {
    let fixture = archive_with_first_file_offset(0);
    assert!(
        fixture.is_ok(),
        "the zero first-file hint fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("first file hint is zero")
        ),
        "nonempty archives must declare a nonzero encoder hint"
    );
}

#[test]
fn rejects_first_file_hint_beyond_archive() {
    let fixture = archive_with_first_file_offset(0x2000);
    assert!(
        fixture.is_ok(),
        "the out-of-range first-file hint fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("first file hint extends beyond archive")
        ),
        "the encoder hint must remain inside the declared archive"
    );
}

#[test]
fn rejects_unaligned_catalog_headers() {
    let fixture = archive_with_unaligned_header();
    assert!(
        fixture.is_ok(),
        "the unaligned-header fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("catalog header is not aligned")
        ),
        "nonzero archive alignment must constrain the catalog start"
    );
}

#[test]
fn rejects_catalog_headers_inside_file_info() {
    let fixture = archive_with_header_overlap();
    assert!(
        fixture.is_ok(),
        "the overlapping-header fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("catalog header overlaps file info")
        ),
        "catalog bytes must begin after the fixed file-info structure"
    );
}

#[test]
fn preserves_first_file_offset() {
    let fixture = archive_with_stored_name(b"sound/file.rsd\0");
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        result
            .as_ref()
            .is_ok_and(
                |archive| {
                    archive
                        .header
                        .first_file_offset
                        == 0x1000
                },
            ),
        "the public header must report m_FirstFileStartPos"
    );
}

#[test]
fn preserves_file_info_alignment() {
    let fixture = archive_with_stored_name(b"sound/file.rsd\0");
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        result
            .as_ref()
            .is_ok_and(
                |archive| {
                    archive
                        .header
                        .alignment
                        == 0x800
                },
            ),
        "archive alignment must come from the fixed file-info structure"
    );
}

#[test]
fn rejects_incomplete_archive_file_info() {
    let fixture = archive_with_valid_flag(0);
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("valid flag")
        ),
        "archives not marked complete must fail before table parsing"
    );
}

#[test]
fn rejects_big_endian_file_info() {
    let fixture = archive_with_big_endian_flag(1);
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("big-endian")
        ),
        "the little-endian parser must reject big-endian file-info headers"
    );
}

#[test]
fn rejects_unterminated_stored_names() {
    let fixture = archive_with_stored_name(b"sound/file.rsd");
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("not NUL-terminated")
        ),
        "name-table rows must include their stored terminator"
    );
}

#[test]
fn rejects_embedded_nul_bytes_in_stored_names() {
    let fixture = archive_with_stored_name(b"sound\0/file.rsd\0");
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("embedded NUL")
        ),
        "stored names must not hide bytes after an embedded terminator"
    );
}

#[test]
fn preserves_utf8_while_normalizing_separators() {
    let fixture =
        archive_with_stored_name("sound\\caf\u{00e9}.rsd\0".as_bytes());
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        result
            .as_ref()
            .is_ok_and(
                |archive| {
                    archive
                        .entries
                        .first()
                        .is_some_and(
                            |entry| entry.name == "sound/caf\u{00e9}.rsd",
                        )
                },
            ),
        "separator normalization must not reinterpret UTF-8 bytes"
    );
}

#[test]
fn rejects_control_characters_in_stored_names() {
    let fixture = archive_with_stored_name(
        b"sound/line
feed.rsd\0",
    );
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("control character")
        ),
        "archive names must not inject control characters into paths or logs"
    );
}

#[test]
fn rejects_invisible_unicode_direction_controls() {
    let fixture =
        archive_with_stored_name("sound/\u{202e}file.rsd\0".as_bytes());
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("Unicode path control")
        ),
        "invisible Unicode formatting must not spoof extracted paths"
    );
}

#[test]
fn preserves_file_modification_times() {
    let expected = 0x1234_5678;
    let fixture = archive_with_modification_time(
        b"sound/file.rsd\0",
        expected,
    );
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        result
            .as_ref()
            .is_ok_and(
                |archive| {
                    archive
                        .entries
                        .first()
                        .is_some_and(
                            |entry| entry.modification_time == expected,
                        )
                },
            ),
        "detailed rows must preserve their file modification time"
    );
}

#[test]
fn rejects_payloads_inside_modification_times() {
    let fixture =
        archive_with_payload_inside_modification_time(b"sound/file.rsd\0");
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("payload overlaps archive metadata")
        ),
        "payload bytes must begin after each modification-time field"
    );
}

#[test]
fn rejects_payloads_inside_the_name_table() {
    let fixture = archive_with_payload_inside_name_table(b"sound/file.rsd\0");
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("payload overlaps archive metadata")
        ),
        "payload bytes must begin after the complete name table"
    );
}
#[test]
fn rejects_index_tables_beyond_the_declared_archive() {
    let fixture = archive_with_index_beyond_declared_length();
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok((bytes, declared_length)) = fixture else {
        return;
    };
    let result = parse_archive_with_declared_length(
        bytes,
        declared_length,
    );

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("index table extends beyond archive data")
        ),
        "index bytes must be bounded before the reader is asked for them"
    );
}
#[test]
fn rejects_names_that_normalize_to_the_same_output_path() {
    let fixture = archive_with_stored_names(
        &[
            b"sound/file.rsd\x00",
            b"sound\x5cfile.rsd\x00",
        ],
    );
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("duplicate output path")
        ),
        "separator aliases must not overwrite the same extracted file"
    );
}

#[test]
fn rejects_unicode_case_folded_output_path_collisions() {
    let upper_name = "sound/CAFÉ.rsd\0";
    let lower_name = "sound/café.rsd\0";
    let fixture = archive_with_stored_names(
        &[
            upper_name.as_bytes(),
            lower_name.as_bytes(),
        ],
    );
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("duplicate output path")
        ),
        "Unicode case aliases must not overwrite one Windows output file"
    );
}

#[test]
fn rejects_unsorted_hash_index_rows() {
    let fixture = archive_with_stored_names(
        &[
            b"sound/other.rsd\0",
            b"sound/file.rsd\0",
        ],
    );
    assert!(
        fixture.is_ok(),
        "the synthetic archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let result = parse_archive(bytes);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("hash index is not sorted")
        ),
        "RCF hash rows must remain sorted for runtime binary search"
    );
}
