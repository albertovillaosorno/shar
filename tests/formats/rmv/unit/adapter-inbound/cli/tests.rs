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

use schoenwald_cli::{CliProgram, OutputStream};

use super::{RmvAuditProgram, USAGE, report_outcome};
use crate::domain::AuditReport;

#[test]
fn successful_audit_summaries_use_standard_output() -> Result<(), String> {
    let report = AuditReport::default();
    let outcome = report_outcome(&report);
    if outcome.status() != schoenwald_cli::ExitStatus::Success {
        return Err("successful RMV report returned failure".to_owned());
    }
    if outcome.output().is_empty() {
        return Err("successful RMV report emitted no summary".to_owned());
    }
    for chunk in outcome.output() {
        if chunk.stream() != OutputStream::Stdout {
            return Err(format!(
                "successful RMV summary used diagnostic stream: {:?}",
                chunk.text()
            ));
        }
    }
    Ok(())
}

#[test]
fn invalid_arguments_return_one_usage_diagnostic() -> Result<(), String> {
    for arguments in [
        Vec::new(),
        vec!["output".to_owned()],
        vec![String::new(), "input".to_owned()],
        vec!["output".to_owned(), String::new()],
    ] {
        let outcome = RmvAuditProgram.execute(&arguments);
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err(format!("invalid RMV arguments passed: {arguments:?}"));
        }
        let [chunk] = outcome.output() else {
            return Err("RMV usage must emit one diagnostic".to_owned());
        };
        if chunk.stream() != OutputStream::Stderr {
            return Err("RMV usage must be written to stderr".to_owned());
        }
        if chunk.text() != format!("{USAGE}\n") {
            return Err(format!("unexpected RMV usage: {:?}", chunk.text()));
        }
    }
    Ok(())
}
