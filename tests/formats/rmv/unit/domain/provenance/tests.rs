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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use super::ProvenanceEvidence;

#[test]
fn rejects_source_extensions_followed_by_backup_suffixes() {
    let evidence = ProvenanceEvidence::from_bytes(b"movie.mov.bak");
    assert!(evidence.embedded_source_names.is_empty());
}

#[test]
fn reports_absent_pre_bink_master_when_no_source_name_is_embedded() {
    let evidence = ProvenanceEvidence::from_bytes(b"BIKi\0\0not much here");
    assert_eq!(evidence.summary(), "pre-bink-master-not-embedded");
}

#[test]
fn preserves_unicode_expansion_source_names() {
    let evidence =
        ProvenanceEvidence::from_bytes("straße.mov\0strasse.mov\0".as_bytes());
    assert_eq!(evidence.embedded_source_names, vec![
        "straße.mov",
        "strasse.mov",
    ]);
}

#[test]
fn deduplicates_unicode_case_variants_of_source_names() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice("VÍDEO.MOV".as_bytes());
    bytes.push(0);
    bytes.extend_from_slice("vídeo.mov".as_bytes());
    let evidence = ProvenanceEvidence::from_bytes(&bytes);
    assert_eq!(evidence.embedded_source_names, vec!["VÍDEO.MOV"]);
}

#[test]
fn deduplicates_case_variants_of_embedded_source_names() {
    let evidence =
        ProvenanceEvidence::from_bytes(b"SOURCE.MOV\0source.mov\0Source.Mov\0");
    assert_eq!(evidence.embedded_source_names, vec!["SOURCE.MOV"]);
}

#[test]
fn deduplicates_repeated_embedded_source_names() {
    let evidence =
        ProvenanceEvidence::from_bytes(b"source.mov\0source.mov\0source.mov\0");
    assert_eq!(evidence.embedded_source_names, vec!["source.mov"]);
}

#[test]
fn ignores_media_extensions_used_as_backup_name_prefixes() {
    let evidence = ProvenanceEvidence::from_bytes(
        b"preview.mov-backup soundtrack.wav-old",
    );
    assert!(evidence.embedded_source_names.is_empty());
}

#[test]
fn ignores_media_extensions_on_directory_components() {
    let evidence = ProvenanceEvidence::from_bytes(
        br"assets.mov/metadata audio.wav\details",
    );
    assert!(evidence.embedded_source_names.is_empty());
}

#[test]
fn ignores_extensions_without_filename_stems() {
    let evidence = ProvenanceEvidence::from_bytes(b".mov .wav .bk2");
    assert!(evidence.embedded_source_names.is_empty());
}

#[test]
fn ignores_extension_prefixes_inside_unrelated_words() {
    let evidence = ProvenanceEvidence::from_bytes(
        b"metadata preview.movie placeholder.waveform",
    );
    assert!(evidence.embedded_source_names.is_empty());
}

#[test]
fn preserves_utf16le_names_around_unpaired_surrogates() {
    let mut bytes = Vec::new();
    for unit in "before.mov".encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.extend_from_slice(&0xd800_u16.to_le_bytes());
    for unit in "after.mov".encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    let evidence = ProvenanceEvidence::from_bytes(&bytes);
    assert_eq!(evidence.embedded_source_names, vec![
        "before.mov",
        "after.mov",
    ]);
}

#[test]
fn preserves_odd_aligned_utf16le_source_names() {
    let mut bytes = vec![0xff];
    for unit in "source.mov".encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    let evidence = ProvenanceEvidence::from_bytes(&bytes);
    assert_eq!(evidence.embedded_source_names, vec!["source.mov"]);
}

#[test]
fn preserves_utf16le_source_names() {
    let mut bytes = Vec::new();
    for unit in "source.mov".encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    let evidence = ProvenanceEvidence::from_bytes(&bytes);
    assert_eq!(evidence.embedded_source_names, vec!["source.mov"]);
}

#[test]
fn preserves_utf8_source_names() {
    let evidence = ProvenanceEvidence::from_bytes("vídeo.mov".as_bytes());
    assert_eq!(evidence.embedded_source_names, vec!["vídeo.mov"]);
}

#[test]
fn preserves_more_than_sixteen_unique_source_names() {
    let mut bytes = Vec::new();
    for index in 0_u8..17 {
        bytes.extend_from_slice(format!("source-{index}.mov").as_bytes());
        bytes.push(0);
    }
    let evidence = ProvenanceEvidence::from_bytes(&bytes);
    assert_eq!(evidence.embedded_source_names.len(), 17);
}

#[test]
fn captures_source_names_after_the_first_megabyte() {
    let mut bytes = vec![0_u8; 1024 * 1024 + 1];
    bytes.extend_from_slice(b"late-source.mov");
    let evidence = ProvenanceEvidence::from_bytes(&bytes);
    assert_eq!(evidence.embedded_source_names, vec!["late-source.mov"]);
}

#[test]
fn captures_embedded_source_like_names_when_present() {
    let evidence =
        ProvenanceEvidence::from_bytes(b"abc original_intro.mov\0other");
    assert_eq!(evidence.embedded_source_names, vec![
        "abc original_intro.mov"
    ]);
}
