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

use super::ScalePolicy;

#[test]
fn accepts_positive_finite_unit_scale() -> Result<(), String> {
    let policy = ScalePolicy::new(
        1.0, true,
    )
    .map_err(|error| format!("valid scale policy failed: {error:?}"))?;
    if policy
        .unit_scale
        .to_bits()
        == 1.0_f32.to_bits()
        && policy.preserves_source_axes
    {
        Ok(())
    } else {
        Err(format!("unexpected scale policy: {policy:?}"))
    }
}

#[test]
fn rejects_nonpositive_or_nonfinite_unit_scale() -> Result<(), String> {
    for value in [
        0.0,
        -1.0,
        f32::INFINITY,
        f32::NAN,
    ] {
        if ScalePolicy::new(
            value, false,
        )
        .is_ok()
        {
            return Err(
                format!("invalid unit scale was accepted: {value}"),
            );
        }
    }
    Ok(())
}
