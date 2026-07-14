// File:
//   - wav_validation.rs
// Path:
//   - src/rsd/tests/wav_validation.rs
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
//   - Public regression coverage for WAV model invariants.
// - Must-Not:
//   - Depend on filesystem output or external audio fixtures.
// - Allows:
//   - Synthetic WAV models and caller-visible serialization assertions.
// - Split-When:
//   - Split when another output codec needs independent validation fixtures.
// - Merge-When:
//   - Another RSD test module owns the same WAV serialization contract.
// - Summary:
//   - Verifies invalid WAV models fail before RIFF serialization.
// - Description:
//   - Exercises public WAV output for invalid metadata and PCM frame shapes.
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

//! Public regression coverage for WAV model validation.
//!
//! Synthetic models prove invalid metadata cannot escape as RIFF bytes.

use rsd::{RsdError, WavAudio};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn invalid_wav_models_are_rejected() {
    let cases = [
        WavAudio {
            channels: 0,
            bits_per_sample: 16,
            sample_rate: 24_000,
            pcm: vec![
                0, 0,
            ],
        },
        WavAudio {
            channels: 1,
            bits_per_sample: 16,
            sample_rate: 0,
            pcm: vec![
                0, 0,
            ],
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
            pcm: vec![
                0, 0,
            ],
        },
    ];

    for wav in cases {
        assert!(
            wav.to_bytes()
                .is_err(),
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
        matches!(
            wav.to_bytes(),
            Err(RsdError::UnsupportedSampleRate(_))
        ),
        "tiny PCM payloads must not be reported as oversized"
    );
}
