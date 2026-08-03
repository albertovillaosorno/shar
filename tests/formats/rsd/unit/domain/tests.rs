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

use super::{RsdError, byte_buffer};

#[test]
fn impossible_buffer_capacity_returns_without_panicking() {
    let allocation = std::panic::catch_unwind(|| byte_buffer(usize::MAX));

    assert!(
        allocation.is_ok(),
        "untrusted buffer sizes must return a typed error instead of \
         panicking"
    );
    let Ok(result) = allocation else {
        return;
    };
    assert!(
        matches!(result, Err(RsdError::AllocationFailed(usize::MAX))),
        "impossible capacities must retain the requested byte count"
    );
}
