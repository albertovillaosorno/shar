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
//   - Radp state test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Radp state test module.
// - Description:
//   - Implements the declared test module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Radp state test module.

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

fn radp_with_index(index: i16) -> Vec<u8> {
    let mut data = vec![0_u8; 0x800];
    assert!(
        copy_fixture_bytes(&mut data, 0, b"RSD4"),
        "fixture magic should fit"
    );
    assert!(
        copy_fixture_bytes(&mut data, 4, b"RADP"),
        "fixture encoding should fit"
    );
    assert!(
        copy_fixture_bytes(&mut data, 8, &1_u32.to_le_bytes(),),
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
    let reserved = vec![b'*'; 0x80 - 20];
    assert!(
        copy_fixture_bytes(&mut data, 20, &reserved,),
        "fixture reserved metadata should fit"
    );
    let padding = vec![b'-'; 0x800 - 0x80];
    assert!(
        copy_fixture_bytes(&mut data, 0x80, &padding,),
        "fixture sector padding should fit"
    );
    data.extend_from_slice(&index.to_le_bytes());
    data.extend_from_slice(&0_i16.to_le_bytes());
    data.extend(std::iter::repeat_n(0_u8, 16));
    data
}

#[test]
fn radp_out_of_range_indexes_are_rejected() {
    for index in [-1_i16, 89_i16] {
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
