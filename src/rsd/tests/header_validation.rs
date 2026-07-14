// File:
//   - header_validation.rs
// Path:
//   - src/rsd/tests/header_validation.rs
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
//   - Public regression coverage for RSD header invariants.
// - Must-Not:
//   - Depend on private audio files or internal parser functions.
// - Allows:
//   - Synthetic headers and caller-visible parsing assertions.
// - Split-When:
//   - Split when codec payload behavior needs separate fixture ownership.
// - Merge-When:
//   - Another RSD test module owns the same header invariant family.
// - Summary:
//   - Verifies malformed RSD headers fail before audio export.
// - Description:
//   - Exercises public parsing for invalid playback and container metadata.
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

//! Public regression coverage for RSD header validation.
//!
//! Synthetic headers keep malformed playback metadata reproducible.

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

fn compact_pcm_with_header(
    channels: u32,
    sample_rate: u32,
) -> Vec<u8> {
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
            &channels.to_le_bytes(),
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
            &sample_rate.to_le_bytes(),
        ),
        "fixture sample rate should fit"
    );
    data.extend_from_slice(
        &[
            0, 0,
        ],
    );
    data
}

fn compact_pcm_with_sample_rate(sample_rate: u32) -> Vec<u8> {
    compact_pcm_with_header(
        1_u32,
        sample_rate,
    )
}

#[test]
fn zero_sample_rate_is_rejected() {
    let data = compact_pcm_with_sample_rate(0);

    assert!(
        RsdAudio::parse(&data).is_err(),
        "zero-Hz audio cannot produce a valid WAV playback rate"
    );
}

#[test]
fn empty_compact_pcm_payload_is_rejected() {
    let mut data = compact_pcm_with_sample_rate(24_000);
    data.truncate(0x80);

    assert!(
        RsdAudio::parse(&data).is_err(),
        "an RSD header without audio bytes is truncated"
    );
}

#[test]
fn negative_encoded_sample_rate_is_rejected() {
    let data = compact_pcm_with_sample_rate(u32::MAX);

    assert!(
        RsdAudio::parse(&data).is_err(),
        "negative signed sample rates must not become enormous unsigned rates"
    );
}

#[test]
fn wav_byte_rate_overflow_is_rejected_during_parse() {
    let data = compact_pcm_with_header(
        16_u32,
        200_000_000_u32,
    );

    assert!(
        RsdAudio::parse(&data).is_err(),
        "headers that cannot encode a WAV byte rate are unsupported"
    );
}
