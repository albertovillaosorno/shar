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
//   - Cli run error test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Cli run error test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Cli run error test module.

use std::path::Path;

use lmlm::ExtractArchiveError;
use lmlm::adapters::driving::cli::run;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn run_preserves_typed_application_failures() {
    let result: Result<usize, ExtractArchiveError> =
        run(Path::new(""), Path::new("unused-output"));
    drop(result);
}
