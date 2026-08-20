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

use super::{SingleExtractProgram, USAGE};

#[test]
fn invalid_argument_counts_return_single_extract_usage() -> Result<(), String> {
    for arguments in [Vec::new(), vec!["input".to_owned()], vec![
        "input".to_owned(),
        "output".to_owned(),
        "extra".to_owned(),
    ]] {
        let outcome = SingleExtractProgram.execute(&arguments);
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err(format!("invalid arguments passed: {arguments:?}"));
        }
        let [chunk] = outcome.output() else {
            return Err("single usage must emit one diagnostic".to_owned());
        };
        if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
            return Err("single usage must be written to stderr".to_owned());
        }
        if chunk.text() != format!("{USAGE}\n") {
            return Err(format!("unexpected single usage: {:?}", chunk.text()));
        }
    }
    Ok(())
}
