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

use super::{RcfProgram, USAGE, prepare_sources};

#[test]
fn unsafe_archive_output_directories_fail_before_io() {
    let archives = ["first/good.rcf", "second/CON.rcf"];
    let result = prepare_sources(&archives);

    assert!(
        result.is_err(),
        "unsafe archive stems must fail during batch preflight"
    );
}

#[test]
fn duplicate_archive_output_directories_fail_before_io() -> Result<(), String> {
    let arguments = vec![
        "extract-many".to_owned(),
        "output".to_owned(),
        "first/Music.rcf".to_owned(),
        "second/music.rcf".to_owned(),
    ];
    let outcome = RcfProgram.execute(&arguments);
    if outcome.status() != schoenwald_cli::ExitStatus::Failure {
        return Err("duplicate archive stems were accepted".to_owned());
    }
    let [chunk] = outcome.output() else {
        return Err("duplicate stems must emit one diagnostic".to_owned());
    };
    let diagnostic = chunk.text();
    if !diagnostic.contains("duplicate archive output directory") {
        let message =
            format!("unexpected duplicate-stem diagnostic: {diagnostic:?}");
        return Err(message);
    }
    Ok(())
}

#[test]
fn invalid_requests_return_one_prefixed_usage_diagnostic() -> Result<(), String>
{
    for arguments in [Vec::new(), vec!["list".to_owned()], vec![
        "extract-many".to_owned(),
        "output".to_owned(),
    ]] {
        let outcome = RcfProgram.execute(&arguments);
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err(format!("invalid request passed: {arguments:?}"));
        }
        let [chunk] = outcome.output() else {
            return Err("invalid request must emit one diagnostic".to_owned());
        };
        if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
            return Err("usage diagnostic must be written to stderr".to_owned());
        }
        let expected = format!("error: {USAGE}\n");
        if chunk.text() != expected {
            return Err(format!(
                "unexpected usage diagnostic: {:?}",
                chunk.text()
            ));
        }
    }
    Ok(())
}
