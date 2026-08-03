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
//   - Wav validation test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Wav validation test module.
// - Description:
//   - Implements the declared test module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Wav validation test module.

use rsd::{RsdError, WavAudio};
use same_file as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn invalid_wav_models_are_rejected() {
    let cases = [
        WavAudio {
            channels: 0,
            bits_per_sample: 16,
            sample_rate: 24_000,
            pcm: vec![0, 0],
        },
        WavAudio {
            channels: 1,
            bits_per_sample: 16,
            sample_rate: 0,
            pcm: vec![0, 0],
        },
        WavAudio {
            channels: 1,
            bits_per_sample: 8,
            sample_rate: 24_000,
            pcm: vec![0],
        },
        WavAudio {
            channels: 1,
            bits_per_sample: 16,
            sample_rate: 24_000,
            pcm: Vec::new(),
        },
        WavAudio {
            channels: 2,
            bits_per_sample: 16,
            sample_rate: 24_000,
            pcm: vec![0, 0],
        },
    ];

    for wav in cases {
        assert!(
            wav.to_bytes().is_err(),
            "invalid WAV metadata or incomplete PCM frames must fail"
        );
    }
}

#[test]
fn byte_rate_overflow_is_a_sample_rate_error() {
    let wav = WavAudio {
        channels: 16,
        bits_per_sample: 16,
        sample_rate: i32::MAX.unsigned_abs(),
        pcm: vec![0; 32],
    };

    assert!(
        matches!(wav.to_bytes(), Err(RsdError::UnsupportedSampleRate(_))),
        "tiny PCM payloads must not be reported as oversized"
    );
}
