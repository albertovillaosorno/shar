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
//   - Unit evidence for algorithm application byte codecs.
// - Must-Not:
//   - Perform reconstruction I/O or depend on proprietary source.
// - Allows:
//   - Exercise private hexadecimal helpers through the source-owned test
//     module.
// - Split-When:
//   - Additional application codecs gain independent test contracts.
// - Merge-When:
//   - Another test module owns the identical codec evidence.
// - Summary:
//   - Algorithm application codec unit tests.
// - Description:
//   - Proves hexadecimal encoding and decoding preserve exact bytes.
// - Usage:
//   - Loaded by the algorithm engine module under cfg(test).
// - Defaults:
//   - Synthetic byte payloads only.
//

//! Algorithm application codec unit tests.

use super::{decode_hex, hex_bytes};

#[test]
fn hexadecimal_round_trip_is_exact() {
    let input = b"source-bound\0payload";
    let encoded = hex_bytes(input);
    assert_eq!(
        decode_hex(&encoded),
        Ok(input.to_vec()),
        "hex round trip must preserve bytes"
    );
}
