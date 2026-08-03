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

use super::TimingPolicy;

#[test]
fn accepts_positive_finite_frame_rate() -> Result<(), String> {
    let policy = TimingPolicy::new(30., true)
        .map_err(|error| format!("valid timing policy failed: {error:?}"))?;
    if policy.frames_per_second.to_bits() == 30f32.to_bits()
        && policy.preserves_cycles
    {
        Ok(())
    } else {
        Err(format!("unexpected timing policy: {policy:?}"))
    }
}

#[test]
fn rejects_nonpositive_or_nonfinite_frame_rate() -> Result<(), String> {
    for value in [0., -1., f32::INFINITY, f32::NAN] {
        if TimingPolicy::new(value, false).is_ok() {
            return Err(format!("invalid frame rate was accepted: {value}"));
        }
    }
    Ok(())
}
