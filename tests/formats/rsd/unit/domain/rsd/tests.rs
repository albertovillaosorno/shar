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

use super::{
    PADDED_DATA_OFFSET, RSD3_MINIMUM_HEADER_SIZE, RsdAudio, RsdEncoding,
    RsdError,
};

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

fn rsd_with(tag: [u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut data = vec![0_u8; PADDED_DATA_OFFSET];
    assert!(copy_fixture_bytes(&mut data, 0, b"RSD4"));
    assert!(copy_fixture_bytes(&mut data, 4, &tag));
    assert!(copy_fixture_bytes(&mut data, 8, &1_u32.to_le_bytes(),));
    assert!(copy_fixture_bytes(&mut data, 12, &16_u32.to_le_bytes(),));
    assert!(copy_fixture_bytes(&mut data, 16, &24_000_u32.to_le_bytes(),));
    let reserved = vec![b'*'; 0x80 - 20];
    assert!(copy_fixture_bytes(&mut data, 20, &reserved,));
    let padding = vec![b'-'; 0x800 - 0x80];
    assert!(copy_fixture_bytes(&mut data, 0x80, &padding,));
    data.extend_from_slice(payload);
    data
}

fn rsd3_pcm(payload: &[u8], data_offset: u32) -> Vec<u8> {
    let header_len = usize::try_from(data_offset)
        .unwrap_or(RSD3_MINIMUM_HEADER_SIZE)
        .max(RSD3_MINIMUM_HEADER_SIZE);
    let mut data = vec![b'*'; header_len];
    assert!(copy_fixture_bytes(&mut data, 0, b"RSD3"));
    assert!(copy_fixture_bytes(&mut data, 4, b"PCM "));
    assert!(copy_fixture_bytes(&mut data, 8, &1_u32.to_le_bytes()));
    assert!(copy_fixture_bytes(&mut data, 12, &16_u32.to_le_bytes()));
    assert!(copy_fixture_bytes(&mut data, 16, &11_025_u32.to_le_bytes()));
    assert!(copy_fixture_bytes(&mut data, 20, &2_u32.to_le_bytes()));
    assert!(copy_fixture_bytes(
        &mut data,
        24,
        &data_offset.to_le_bytes()
    ));
    data.extend_from_slice(payload);
    data
}

#[test]
fn rsd3_pcm_uses_declared_payload_offset() {
    let payload = [1_u8, 0, 2, 0];
    let data = rsd3_pcm(&payload, 0xa0);
    let parsed = RsdAudio::parse(&data);
    assert!(parsed.is_ok(), "RSD3 PCM should parse");
    let Ok(audio) = parsed else {
        return;
    };
    assert_eq!(audio.header.encoding, RsdEncoding::PcmLittleEndian);
    assert_eq!(audio.header.channels, 1);
    assert_eq!(audio.header.bits_per_sample, 16);
    assert_eq!(audio.header.sample_rate, 11_025);
    let converted = audio.to_wav();
    assert!(converted.is_ok(), "RSD3 PCM should convert to WAV");
    let Ok(wav) = converted else {
        return;
    };
    assert_eq!(wav.pcm, payload);
}

#[test]
fn rsd3_rejects_payload_offset_inside_header() {
    let mut data = rsd3_pcm(&[1_u8, 0], 0xa0);
    assert!(copy_fixture_bytes(&mut data, 24, &20_u32.to_le_bytes()));
    assert!(matches!(
        RsdAudio::parse(&data),
        Err(RsdError::InvalidDataOffset(20))
    ));
}

#[test]
fn rsd3_rejects_payload_offset_beyond_file() {
    let mut data = rsd3_pcm(&[1_u8, 0], 0xa0);
    assert!(copy_fixture_bytes(&mut data, 24, &0x1000_u32.to_le_bytes()));
    assert!(matches!(
        RsdAudio::parse(&data),
        Err(RsdError::InvalidDataOffset(0x1000))
    ));
}

#[test]
fn unsupported_encoding_precedes_numeric_narrowing() {
    let mut data = vec![0_u8; 20];
    assert!(copy_fixture_bytes(&mut data, 0, b"RSD4"));
    assert!(copy_fixture_bytes(&mut data, 4, b"BAD!"));
    assert!(copy_fixture_bytes(&mut data, 8, &u32::MAX.to_le_bytes(),));
    assert!(copy_fixture_bytes(&mut data, 12, &16_u32.to_le_bytes(),));
    assert!(copy_fixture_bytes(&mut data, 16, &24_000_u32.to_le_bytes(),));

    assert!(
        matches!(
            RsdAudio::parse(&data),
            Err(RsdError::UnsupportedEncoding(tag)) if tag == *b"BAD!"
        ),
        "unsupported codecs must fail before numeric field narrowing"
    );
}

#[test]
fn invalid_header_is_rejected_before_body_access() {
    let mut data = vec![0_u8; 20];
    assert!(copy_fixture_bytes(&mut data, 0, b"RSD4"));
    assert!(copy_fixture_bytes(&mut data, 4, b"PCM "));
    assert!(copy_fixture_bytes(&mut data, 8, &0_u32.to_le_bytes(),));
    assert!(copy_fixture_bytes(&mut data, 12, &16_u32.to_le_bytes(),));
    assert!(copy_fixture_bytes(&mut data, 16, &24_000_u32.to_le_bytes(),));

    let result = RsdAudio::parse(&data);

    assert!(
        matches!(
            result,
            Err(RsdError::UnsupportedChannels(value))
                if value == 0_u32
        ),
        "invalid header fields must fail before payload validation"
    );
}

#[test]
fn parses_pcm_header_and_preserves_native_encoding() {
    let data = rsd_with(*b"PCM ", &[1, 0, 2, 0]);
    let parsed = RsdAudio::parse(&data);
    assert!(parsed.is_ok(), "RSD header should parse");
    let Ok(audio) = parsed else {
        return;
    };
    assert_eq!(audio.header.encoding, RsdEncoding::PcmLittleEndian);
    assert_eq!(audio.header.channels, 1);
    assert_eq!(audio.header.bits_per_sample, 16);
    assert_eq!(audio.header.sample_rate, 24_000);
    let converted = audio.to_wav();
    assert!(converted.is_ok(), "PCM audio should convert to WAV");
    let Ok(wav) = converted else {
        return;
    };
    assert_eq!(wav.pcm, vec![1, 0, 2, 0]);
}

#[test]
fn decodes_silent_radical_adpcm_frame_to_silent_pcm() {
    let mut frame = Vec::new();
    frame.extend_from_slice(&0_i16.to_le_bytes());
    frame.extend_from_slice(&0_i16.to_le_bytes());
    frame.extend(std::iter::repeat_n(0_u8, 16));
    let data = rsd_with(*b"RADP", &frame);
    let parsed = RsdAudio::parse(&data);
    assert!(parsed.is_ok(), "RSD header should parse");
    let Ok(audio) = parsed else {
        return;
    };
    let converted = audio.to_wav();
    assert!(converted.is_ok(), "RADP audio should decode to WAV");
    let Ok(wav) = converted else {
        return;
    };
    assert_eq!(wav.pcm.len(), 32 * 2);
    assert!(
        wav.pcm.chunks(2).all(|sample| sample == [0, 0]),
        "silent RADP frame should decode to zero PCM samples"
    );
}
