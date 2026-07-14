// File:
//   - compact_pcm.rs
// Path:
//   - src/rsd/tests/compact_pcm.rs
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
//   - Public regression coverage for compact RSD4 PCM payload boundaries.
// - Must-Not:
//   - Depend on private audio inputs or internal parser functions.
// - Allows:
//   - Synthetic RSD4 fixtures and caller-visible PCM assertions.
// - Split-When:
//   - Split when another RSD revision needs separate fixtures.
// - Merge-When:
//   - Another RSD test module owns the same compact payload boundary.
// - Summary:
//   - Verifies compact RSD4 PCM payloads start at the short header boundary.
// - Description:
//   - Exercises public parsing and conversion for valid unpadded RSD4 PCM data.
// - Usage:
//   - Executed through cargo test for the rsd crate.
// - Defaults:
//   - Fixtures remain synthetic, deterministic, and repository-local.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Public regression coverage for compact RSD4 PCM payloads.

use rsd::RsdAudio;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn copy_fixture_bytes(
    data: &mut [u8],
    start: usize,
    bytes: &[u8],
) -> bool {
    let Some(end) = start.checked_add(bytes.len()) else {
        return false;
    };
    let Some(target) = data.get_mut(start..end) else {
        return false;
    };
    target.copy_from_slice(bytes);
    true
}

fn compact_pcm(payload: &[u8]) -> Vec<u8> {
    let mut data = vec![0_u8; 0x80];
    assert!(
        copy_fixture_bytes(
            &mut data, 0, b"RSD4"
        ),
        "fixture magic should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data, 4, b"PCM "
        ),
        "fixture encoding should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            8,
            &1_u32.to_le_bytes(),
        ),
        "fixture channel count should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            12,
            &16_u32.to_le_bytes(),
        ),
        "fixture bit depth should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            16,
            &24_000_u32.to_le_bytes(),
        ),
        "fixture sample rate should fit"
    );
    data.extend_from_slice(payload);
    data
}

#[test]
fn compact_pcm_payload_may_begin_with_padding_marker_bytes() {
    let payload = [
        b'-', b'-', b'-', b'-', 1, 0, 2, 0,
    ];
    let data = compact_pcm(&payload);

    let parsed = RsdAudio::parse(&data);
    assert!(
        parsed.is_ok(),
        "payload bytes must not be mistaken for legacy header padding"
    );
    let Ok(audio) = parsed else {
        return;
    };
    let converted = audio.to_wav();
    assert!(
        converted.is_ok(),
        "compact PCM beginning with dash samples should convert"
    );
    let Ok(wav) = converted else {
        return;
    };
    assert_eq!(
        wav.pcm,
        payload
    );
}

#[test]
fn compact_pcm_uses_the_short_payload_boundary() {
    let data = compact_pcm(
        &[
            1, 0, 2, 0,
        ],
    );

    let parsed = RsdAudio::parse(&data);
    assert!(
        parsed.is_ok(),
        "compact RSD4 PCM should parse"
    );
    let Ok(audio) = parsed else {
        return;
    };
    let converted = audio.to_wav();
    assert!(
        converted.is_ok(),
        "compact PCM should convert"
    );
    let Ok(wav) = converted else {
        return;
    };
    assert_eq!(
        wav.pcm,
        vec![
            1, 0, 2, 0
        ]
    );
}
