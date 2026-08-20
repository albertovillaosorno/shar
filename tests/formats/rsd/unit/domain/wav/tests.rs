// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
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

use super::WavAudio;

#[test]
fn writes_native_pcm_wave_header() {
    let wav = WavAudio {
        channels: 2,
        bits_per_sample: 16,
        sample_rate: 22_050,
        pcm: vec![0; 8],
    };
    let serialized = wav.to_bytes();
    assert!(serialized.is_ok(), "validated WAV model should serialize");
    let Ok(bytes) = serialized else {
        return;
    };
    assert_eq!(bytes.get(0..4), Some(b"RIFF".as_slice()));
    assert_eq!(bytes.get(8..12), Some(b"WAVE".as_slice()));
    let channels = bytes
        .get(22..24)
        .and_then(|slice| <[u8; 2]>::try_from(slice).ok())
        .map(u16::from_le_bytes);
    assert_eq!(channels, Some(2));
    let sample_rate = bytes
        .get(24..28)
        .and_then(|slice| <[u8; 4]>::try_from(slice).ok())
        .map(u32::from_le_bytes);
    assert_eq!(sample_rate, Some(22_050));
    let bits = bytes
        .get(34..36)
        .and_then(|slice| <[u8; 2]>::try_from(slice).ok())
        .map(u16::from_le_bytes);
    assert_eq!(bits, Some(16));
    assert_eq!(bytes.get(36..40), Some(b"data".as_slice()));
}
