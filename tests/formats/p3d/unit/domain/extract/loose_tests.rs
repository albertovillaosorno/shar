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
//   - Extract loose unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Extract loose unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Assertions fail explicitly.
//

//! Extract loose unit tests.

use super::*;

#[test]
fn p3dz_header_reader_rejects_offset_overflow() -> Result<(), String> {
    let read = read_u32(&[], usize::MAX);
    if read.is_ok() {
        return Err(String::from(
            "P3DZ header reads must reject an offset that cannot contain \
                 a u32",
        ));
    }
    Ok(())
}
