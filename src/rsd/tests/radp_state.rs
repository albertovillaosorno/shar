// File:
//   - radp_state.rs
// Path:
//   - src/rsd/tests/radp_state.rs
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
//   - Public regression coverage for malformed RADP predictor state.
// - Must-Not:
//   - Depend on private audio files or internal decoder functions.
// - Allows:
//   - Synthetic RADP frames and caller-visible PCM assertions.
// - Split-When:
//   - Split when RADP sample layout needs a separate fixture family.
// - Merge-When:
//   - Another RSD test module owns the same predictor-state contract.
// - Summary:
//   - Verifies invalid RADP state headers fail before sample decoding.
// - Description:
//   - Exercises public conversion for out-of-range RADP step indexes.
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

//! Public regression coverage for malformed RADP predictor state.
//!
//! Synthetic frames prove invalid indexes fail without private audio inputs.

use rsd::{RsdAudio, RsdError};
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

fn radp_with_index(index: i16) -> Vec<u8> {
    let mut data = vec![0_u8; 0x800];
    assert!(
        copy_fixture_bytes(
            &mut data, 0, b"RSD4"
        ),
        "fixture magic should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data, 4, b"RADP"
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
    let reserved = vec![b'*'; 0x80 - 20];
    assert!(
        copy_fixture_bytes(
            &mut data, 20, &reserved,
        ),
        "fixture reserved metadata should fit"
    );
    let padding = vec![b'-'; 0x800 - 0x80];
    assert!(
        copy_fixture_bytes(
            &mut data, 0x80, &padding,
        ),
        "fixture sector padding should fit"
    );
    data.extend_from_slice(&index.to_le_bytes());
    data.extend_from_slice(&0_i16.to_le_bytes());
    data.extend(
        std::iter::repeat_n(
            0_u8, 16,
        ),
    );
    data
}

#[test]
fn radp_out_of_range_indexes_are_rejected() {
    for index in [
        -1_i16, 89_i16,
    ] {
        let data = radp_with_index(index);
        let parsed = RsdAudio::parse(&data);
        assert!(
            parsed.is_ok(),
            "RADP container should parse before predictor-state validation"
        );
        let Ok(audio) = parsed else {
            return;
        };

        assert!(
            matches!(
                audio.to_wav(),
                Err(RsdError::InvalidStepIndex(value))
                    if value == i32::from(index)
            ),
            "RADP step indexes outside the decoder table must fail closed"
        );
    }
}
