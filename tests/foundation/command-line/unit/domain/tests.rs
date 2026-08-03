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

use super::{CommandOutcome, ExitStatus, OutputStream};

#[test]
fn line_helpers_preserve_output_order_and_add_one_newline() {
    let outcome = CommandOutcome::failure()
        .stdout("raw")
        .stderr_line("problem")
        .stdout_line("done");
    let expected = vec![
        super::OutputChunk::new(OutputStream::Stdout, "raw"),
        super::OutputChunk::new(
            OutputStream::Stderr,
            "problem
",
        ),
        super::OutputChunk::new(
            OutputStream::Stdout,
            "done
",
        ),
    ];

    assert_eq!(outcome.status(), ExitStatus::Failure);
    assert_eq!(outcome.output(), expected.as_slice());
}
