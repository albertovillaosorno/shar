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

use super::{USAGE, ValidateManifestCli};

#[test]
fn excess_arguments_return_validator_usage_without_storage_access()
-> Result<(), String> {
    let outcome =
        ValidateManifestCli.execute(&["first".to_owned(), "second".to_owned()]);
    if outcome.status() != schoenwald_cli::ExitStatus::Failure {
        return Err("excess validator arguments must fail".to_owned());
    }
    let [chunk] = outcome.output() else {
        return Err("validator usage must emit one diagnostic".to_owned());
    };
    if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
        return Err("validator usage must be written to stderr".to_owned());
    }
    if chunk.text() != format!("{USAGE}\n") {
        return Err(format!("unexpected validator usage: {:?}", chunk.text()));
    }
    Ok(())
}
