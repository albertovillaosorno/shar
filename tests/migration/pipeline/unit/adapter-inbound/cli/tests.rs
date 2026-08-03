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

use schoenwald_cli::{CliProgram, ExitStatus, OutputStream};

use super::{PipelineCli, USAGE};

#[test]
fn manifest_rejects_extra_positionals() -> Result<(), String> {
    let outcome = super::run_fbx_manifest(&[
        "missing-index.jsonl".to_owned(),
        "type:model".to_owned(),
        "output".to_owned(),
        "extra".to_owned(),
    ]);
    if outcome.status() != ExitStatus::Failure {
        return Err("extra manifest positional must fail".to_owned());
    }
    let [diagnostic] = outcome.output() else {
        return Err("extra positional must emit one diagnostic".to_owned());
    };
    let expected = "unexpected positional argument: extra
";
    if diagnostic.text() != expected {
        return Err(format!(
            "unexpected extra-position diagnostic: {:?}",
            diagnostic.text()
        ));
    }
    Ok(())
}

#[test]
fn missing_command_returns_usage_on_stderr() -> Result<(), String> {
    let outcome = PipelineCli.execute(&[]);
    if outcome.status() != ExitStatus::Failure {
        return Err("missing command must fail".to_owned());
    }
    let [chunk] = outcome.output() else {
        return Err("missing command must emit one usage chunk".to_owned());
    };
    if chunk.stream() != OutputStream::Stderr {
        return Err("usage must be written to stderr".to_owned());
    }
    let expected = format!(
        "{USAGE}
"
    );
    if chunk.text() != expected {
        return Err(format!("unexpected usage output: {:?}", chunk.text()));
    }
    Ok(())
}

#[test]
fn unknown_command_returns_name_and_usage() -> Result<(), String> {
    let outcome = PipelineCli.execute(&["unknown".to_owned()]);
    if outcome.status() != ExitStatus::Failure {
        return Err("unknown command must fail".to_owned());
    }
    let [unknown, usage] = outcome.output() else {
        return Err("unknown command must emit diagnostic and usage".to_owned());
    };
    if unknown.text()
        != "unknown command: unknown
"
    {
        return Err(format!(
            "unexpected command diagnostic: {:?}",
            unknown.text()
        ));
    }
    let expected = format!(
        "{USAGE}
"
    );
    if usage.text() != expected {
        return Err(format!("unexpected usage output: {:?}", usage.text()));
    }
    Ok(())
}
