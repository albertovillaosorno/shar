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
//   - Payload validation test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Payload validation test module.
// - Description:
//   - Implements the declared test module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Payload validation test module.

use rsd::{RsdAudio, RsdError};
use same_file as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn copy_fixture_bytes(data: &mut [u8], start: usize, bytes: &[u8]) -> bool {
    let Some(end) = start.checked_add(bytes.len()) else {
        return false;
    };
    let Some(target) = data.get_mut(start..end) else {
        return false;
    };
    target.copy_from_slice(bytes);
    true
}

fn rsd_with(
    tag: [u8; 4],
    channels: u32,
    data_offset: usize,
    payload: &[u8],
) -> Vec<u8> {
    let mut data = vec![0_u8; data_offset];
    assert!(
        copy_fixture_bytes(&mut data, 0, b"RSD4"),
        "fixture magic should fit"
    );
    assert!(
        copy_fixture_bytes(&mut data, 4, &tag),
        "fixture encoding should fit"
    );
    assert!(
        copy_fixture_bytes(&mut data, 8, &channels.to_le_bytes(),),
        "fixture channel count should fit"
    );
    assert!(
        copy_fixture_bytes(&mut data, 12, &16_u32.to_le_bytes(),),
        "fixture bit depth should fit"
    );
    assert!(
        copy_fixture_bytes(&mut data, 16, &24_000_u32.to_le_bytes(),),
        "fixture sample rate should fit"
    );
    data.extend_from_slice(payload);
    data
}

#[test]
fn padded_headers_reject_corrupt_reserved_bytes() {
    let mut data = rsd_with(*b"PCM ", 1_u32, 0x800, &[1_u8, 0_u8]);
    let reserved = vec![b'*'; 0x80 - 20];
    assert!(copy_fixture_bytes(&mut data, 20, &reserved,));
    let padding = vec![b'-'; 0x800 - 0x80];
    assert!(copy_fixture_bytes(&mut data, 0x80, &padding,));
    assert!(copy_fixture_bytes(&mut data, 64, b"!"));

    assert!(
        matches!(RsdAudio::parse(&data), Err(RsdError::InvalidHeaderPadding)),
        "padded RSD metadata corruption must fail before decoding"
    );
}

#[test]
fn padded_pcm_rejects_corrupt_sector_padding() {
    let mut data = rsd_with(*b"PCM ", 1_u32, 0x800, &[1_u8, 0_u8]);
    let reserved = vec![b'*'; 0x80 - 20];
    assert!(copy_fixture_bytes(&mut data, 20, &reserved,));
    let padding = vec![b'-'; 0x800 - 0x80];
    assert!(copy_fixture_bytes(&mut data, 0x80, &padding,));
    assert!(copy_fixture_bytes(&mut data, 0x100, b"!"));

    assert!(
        matches!(RsdAudio::parse(&data), Err(RsdError::InvalidHeaderPadding)),
        "padded PCM corruption must not be reclassified as compact audio"
    );
}

#[test]
fn big_endian_pcm_requires_legacy_padding() {
    let data = rsd_with(*b"PCMB", 1_u32, 0x80, &[0_u8, 1_u8]);

    assert!(
        matches!(RsdAudio::parse(&data), Err(RsdError::InvalidHeaderPadding)),
        "big-endian PCM must not use the compact extension layout"
    );
}

#[test]
fn radical_adpcm_rejects_corrupt_sector_padding() {
    let mut data = rsd_with(*b"RADP", 1_u32, 0x800, &[0_u8; 20]);
    let reserved = vec![b'*'; 0x80 - 20];
    assert!(copy_fixture_bytes(&mut data, 20, &reserved,));
    let padding = vec![b'-'; 0x800 - 0x80];
    assert!(copy_fixture_bytes(&mut data, 0x80, &padding,));
    assert!(copy_fixture_bytes(&mut data, 0x100, b"!"));

    assert!(
        matches!(RsdAudio::parse(&data), Err(RsdError::InvalidHeaderPadding)),
        "RADP sector-padding corruption must fail before decoding"
    );
}

#[test]
fn radical_adpcm_rejects_sixteen_channels() {
    let data = rsd_with(*b"RADP", 16_u32, 0x800, &[0_u8; 20 * 16]);

    assert!(
        RsdAudio::parse(&data).is_err(),
        "RADP decoder state supports fewer than sixteen channels"
    );
}

#[test]
fn misaligned_payloads_fail_during_parse() {
    let incomplete_stereo_pcm = rsd_with(*b"PCM ", 2_u32, 0x80, &[0, 0]);
    let incomplete_radp = rsd_with(*b"RADP", 1_u32, 0x800, &[0; 19]);

    for data in [incomplete_stereo_pcm, incomplete_radp] {
        assert!(
            RsdAudio::parse(&data).is_err(),
            "incomplete codec frames must fail at the parsing boundary"
        );
    }
}
