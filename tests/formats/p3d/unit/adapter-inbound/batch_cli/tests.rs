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

use schoenwald_cli::CliProgram;

use super::{BatchExtractProgram, USAGE};

#[test]
fn missing_output_or_input_roots_return_batch_usage() -> Result<(), String> {
    for arguments in [Vec::new(), vec!["output".to_owned()]] {
        let outcome = BatchExtractProgram.execute(&arguments);
        if !outcome.is_failure_with_stderr_line(USAGE) {
            return Err(format!(
                "invalid batch usage outcome for arguments: \
                     {arguments:?}"
            ));
        }
    }
    Ok(())
}
