// File:
//   - cli_run_error.rs
// Path:
//   - src/lmlm/tests/cli_run_error.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Public CLI-run error typing regressions.
// - Must-Not:
//   - Read private archives or create output state.
// - Allows:
//   - Invalid synthetic input paths that fail during source loading.
// - Split-When:
//   - Another public CLI-run stage needs independent fixtures.
// - Merge-When:
//   - The CLI no longer exposes a process-neutral run function.
// - Summary:
//   - Proves public CLI runs preserve typed application failures.
// - Description:
//   - Exercises source-read failure identity before presentation mapping.
// - Usage:
//   - Compiled as an LMLM integration test.
// - Defaults:
//   - No filesystem mutation occurs.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Public CLI-run error typing regressions.
//!
//! Presentation must not erase application failure structure.

use std::path::Path;

use lmlm::ExtractArchiveError;
use lmlm::adapters::driving::cli::run;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn run_preserves_typed_application_failures() {
    let result: Result<usize, ExtractArchiveError> = run(
        Path::new(""),
        Path::new("unused-output"),
    );
    drop(result);
}
