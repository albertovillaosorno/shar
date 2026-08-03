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

use super::{LmlmExtractProgram, USAGE, success_outcome};

#[test]
fn missing_or_excess_arguments_return_the_same_usage_contract()
-> Result<(), String> {
    for arguments in [Vec::new(), vec!["input".to_owned()], vec![
        "input".to_owned(),
        "output".to_owned(),
        "extra".to_owned(),
    ]] {
        let outcome = LmlmExtractProgram.execute(&arguments);
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err(format!(
                "invalid arguments unexpectedly passed: {arguments:?}"
            ));
        }
        let [chunk] = outcome.output() else {
            return Err("usage failure must emit one diagnostic".to_owned());
        };
        if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
            return Err("LMLM usage must be written to stderr".to_owned());
        }
        if chunk.text()
            != format!(
                "{USAGE}
"
            )
        {
            return Err(format!("unexpected LMLM usage: {:?}", chunk.text()));
        }
    }
    Ok(())
}

#[test]
fn one_file_uses_the_singular_success_noun() {
    let outcome = success_outcome(1, "output");
    let rendered = outcome
        .output()
        .first()
        .map(schoenwald_cli::OutputChunk::text);

    assert_eq!(rendered, Some("extracted 1 file to output\n"));
}

#[test]
fn successful_extractions_escape_control_characters_in_output_paths()
-> Result<(), String> {
    let outcome = success_outcome(1, "bad\nroot");
    let Some(first) = outcome.output().first() else {
        return Err("successful extraction emitted no diagnostic".to_owned());
    };
    let Some(line) = first.text().strip_suffix('\n') else {
        return Err(
            "successful extraction omitted its line terminator".to_owned()
        );
    };
    if line.contains('\n') {
        return Err(
            "successful extraction emitted a raw path newline".to_owned()
        );
    }
    if !line.contains(r"bad\nroot") {
        return Err(format!(
            "successful extraction lost escaped path evidence: {:?}",
            first.text()
        ));
    }
    Ok(())
}
