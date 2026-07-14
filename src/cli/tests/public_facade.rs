// File:
//   - public_facade.rs
// Path:
//   - src/cli/tests/public_facade.rs
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
//   - Regression coverage for the crate-level public facade.
// - Must-Not:
//   - Import stable contracts through internal module paths.
// - Allows:
//   - Verify concrete adapters and their ports from the crate root.
// - Split-When:
//   - Another public facade family needs separate coverage.
// - Merge-When:
//   - The crate no longer exposes shared CLI mechanisms.
// - Summary:
//   - Public facade regression.
// - Description:
//   - Proves stable CLI contracts have one shallow import surface.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - Concrete process adapters satisfy their public ports.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for the shallow crate-level facade.
//!
//! Consumers must not need internal module paths for stable CLI contracts.

use std::mem::size_of;

use schoenwald_cli::{
    ArgumentSource, EnvironmentArguments, OutputSink, RunInvocation,
    StandardStreams,
};

const fn assert_argument_source<T: ArgumentSource>() {}

const fn assert_output_sink<T: OutputSink>() {}

#[test]
fn stable_contracts_are_available_from_the_crate_root() {
    assert_argument_source::<EnvironmentArguments>();
    assert_output_sink::<StandardStreams>();
    assert_eq!(
        size_of::<RunInvocation>(),
        0
    );
}
