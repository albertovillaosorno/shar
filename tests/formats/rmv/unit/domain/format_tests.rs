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
//   - Format tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Format tests test module.
// - Description:
//   - Implements the declared test module responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Format tests test module.

use super::MovieKind;

/// Zeroed storage for one complete Bink header fixture.
const BINK_HEADER_STORAGE: [u8; 36] = [0_u8; 36];

fn ogg_crc(bytes: &[u8]) -> u32 {
    let mut crc = 0_u32;
    for value in bytes.iter().copied() {
        crc ^= u32::from(value) << 24;
        for _ in 0..8 {
            crc = if crc & 0x8000_0000 != 0 {
                (crc << 1) ^ 0x04c1_1db7
            } else {
                crc << 1
            };
        }
    }
    crc
}

fn ogg_page(payload: &[u8]) -> Result<Vec<u8>, String> {
    let payload_len = u8::try_from(payload.len())
        .map_err(|error| format!("test Ogg payload is too large: {error}"))?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OggS");
    bytes.extend_from_slice(&[0, 0x06]);
    bytes.extend_from_slice(&[0; 16]);
    bytes.extend_from_slice(&[0; 4]);
    bytes.push(1);
    bytes.push(payload_len);
    bytes.extend_from_slice(payload);
    let crc = ogg_crc(&bytes);
    let checksum = bytes.get_mut(22..26).ok_or_else(|| {
        "synthetic Ogg page lacks a checksum field".to_owned()
    })?;
    checksum.copy_from_slice(&crc.to_le_bytes());
    Ok(bytes)
}

fn xmv_header() -> Result<[u8; 36], String> {
    let mut bytes = [0_u8; 36];
    bytes
        .get_mut(4..8)
        .ok_or_else(|| "synthetic XMV header lacks packet size".to_owned())?
        .copy_from_slice(&36_u32.to_le_bytes());
    bytes
        .get_mut(12..16)
        .ok_or_else(|| "synthetic XMV header lacks signature".to_owned())?
        .copy_from_slice(b"xobX");
    bytes
        .get_mut(16..20)
        .ok_or_else(|| "synthetic XMV header lacks version".to_owned())?
        .copy_from_slice(&3_u32.to_le_bytes());
    Ok(bytes)
}

fn bink_header(signature: [u8; 4]) -> [u8; 36] {
    let mut bytes = BINK_HEADER_STORAGE;
    for (target, source) in bytes.iter_mut().zip(signature) {
        *target = source;
    }
    for (field_index, field) in bytes.chunks_mut(4).enumerate().skip(1) {
        let value = match field_index {
            1 => 28_u32,
            2 | 8 => 1,
            3 => 4,
            5 => 640,
            6 => 480,
            7 => 30,
            _ => 0,
        };
        field.copy_from_slice(&value.to_le_bytes());
    }
    bytes
}

fn set_header_field(
    bytes: &mut [u8; MovieKind::HEADER_PROBE_LEN],
    field_index: usize,
    value: u32,
) {
    for (index, field) in bytes.chunks_mut(4).enumerate() {
        if index == field_index {
            field.copy_from_slice(&value.to_le_bytes());
            return;
        }
    }
}

fn assert_rejects_header_field(
    signature: [u8; 4],
    field_index: usize,
    value: u32,
) {
    let mut bytes = bink_header(signature);
    set_header_field(&mut bytes, field_index, value);
    assert_eq!(MovieKind::from_bytes(&bytes), MovieKind::Unknown);
}

#[test]
fn rejects_unsupported_bink_revisions() {
    assert_eq!(MovieKind::from_prefix(b"BIKa"), MovieKind::Unknown);
    assert_eq!(MovieKind::from_prefix(b"BIKz"), MovieKind::Unknown);
    assert_eq!(MovieKind::from_prefix(b"KB2b"), MovieKind::Unknown);
    assert_eq!(MovieKind::from_prefix(b"KB2e"), MovieKind::Unknown);
    assert_eq!(MovieKind::from_prefix(b"BK2i"), MovieKind::Unknown);
}

#[test]
fn classifies_supported_structural_bink_headers() {
    let bink_v1 = bink_header(*b"BIKi");
    assert_eq!(MovieKind::from_bytes(&bink_v1), MovieKind::BinkV1);
    let bink_v2 = bink_header(*b"KB2i");
    assert_eq!(MovieKind::from_bytes(&bink_v2), MovieKind::BinkV2);
}

fn assert_rejects_required_fields(signature: [u8; 4]) {
    assert_rejects_header_field(signature, 1, 0);
    assert_rejects_header_field(signature, 2, 0);
    assert_rejects_header_field(signature, 2, 1_000_001);
    assert_rejects_header_field(signature, 3, 37);
    assert_rejects_header_field(signature, 5, 0);
    assert_rejects_header_field(signature, 5, 7_681);
    assert_rejects_header_field(signature, 6, 0);
    assert_rejects_header_field(signature, 6, 4_801);
    assert_rejects_header_field(signature, 7, 0);
    assert_rejects_header_field(signature, 8, 0);
}

#[test]
fn rejects_malformed_mandatory_bink_header_fields() {
    let bink_v1 = bink_header(*b"BIKi");
    assert_eq!(
        MovieKind::from_sized_header(&bink_v1[..35], 36,),
        MovieKind::Unknown
    );
    let bink_v2 = bink_header(*b"KB2i");
    assert_eq!(
        MovieKind::from_sized_header(&bink_v2[..35], 36,),
        MovieKind::Unknown
    );
    assert_rejects_required_fields(*b"BIKi");
    assert_rejects_required_fields(*b"KB2i");
}

#[test]
fn builds_bink_header_signature() {
    let bytes = bink_header(*b"BIKi");
    assert_eq!(MovieKind::from_prefix(&bytes), MovieKind::BinkV1);
}

#[test]
fn classifies_xbox_xmv_like_credit_movie_header() {
    let mut bytes = [0_u8; 32];
    bytes[12..16].copy_from_slice(b"xobX");
    assert_eq!(MovieKind::from_prefix(&bytes), MovieKind::XboxXmvLike);
}

#[test]
fn complete_classification_rejects_signature_only_non_bink_inputs() {
    let mut xbox = [0_u8; 16];
    xbox[12..16].copy_from_slice(b"xobX");
    for bytes in [b"OggS".as_slice(), xbox.as_slice(), b"rmv"] {
        assert_eq!(MovieKind::from_bytes(bytes), MovieKind::Unknown);
    }
}

#[test]
fn complete_classification_accepts_structural_ogg_and_xmv_inputs()
-> Result<(), String> {
    assert_eq!(
        MovieKind::from_bytes(&ogg_page(b"codec-header")?),
        MovieKind::OggNamedRmv
    );
    assert_eq!(
        MovieKind::from_bytes(&xmv_header()?),
        MovieKind::XboxXmvLike
    );
    Ok(())
}

#[test]
fn complete_classification_rejects_corrupt_ogg_and_xmv_headers()
-> Result<(), String> {
    let mut ogg = ogg_page(b"codec-header")?;
    let payload_byte = ogg
        .get_mut(28)
        .ok_or_else(|| "synthetic Ogg page lacks payload data".to_owned())?;
    *payload_byte ^= 1;
    assert_eq!(MovieKind::from_bytes(&ogg), MovieKind::Unknown);

    let mut xmv = xmv_header()?;
    xmv.get_mut(16..20)
        .ok_or_else(|| "synthetic XMV header lacks version".to_owned())?
        .copy_from_slice(&0_u32.to_le_bytes());
    assert_eq!(MovieKind::from_bytes(&xmv), MovieKind::Unknown);
    Ok(())
}

#[test]
fn rejects_truncated_bink_v1_signature() {
    assert_eq!(MovieKind::from_prefix(b"BIK"), MovieKind::Unknown);
}

#[test]
fn rejects_truncated_bink_v2_signature() {
    assert_eq!(MovieKind::from_prefix(b"KB2"), MovieKind::Unknown);
}

#[test]
fn rejects_bink_signatures_without_alphabetic_version_bytes() {
    for malformed in [b"BIK\0".as_slice(), b"BIK1", b"KB2\0", b"BK2-"] {
        assert_eq!(
            MovieKind::from_prefix(malformed),
            MovieKind::Unknown,
            "malformed Bink signature was accepted: {malformed:?}"
        );
    }
}

#[test]
fn rejects_rmv_substrings_outside_the_header_prefix() {
    assert_eq!(
        MovieKind::from_prefix(b"metadata-rmv-payload"),
        MovieKind::Unknown
    );
}

#[test]
fn classifies_radical_movie_header_prefix() {
    assert_eq!(
        MovieKind::from_prefix(b"rmvgcn10"),
        MovieKind::RadicalMovieHeader
    );
}
