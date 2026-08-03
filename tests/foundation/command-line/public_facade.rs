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
//   - Public facade test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Public facade test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Public facade test module.

use schoenwald_cli::{
    ArgumentSource, EnvironmentArguments, OutputSink, RunInvocation,
    StandardStreams,
};

const fn assert_argument_source<T>()
where
    T: ArgumentSource,
{
}

const fn assert_output_sink<T>()
where
    T: OutputSink,
{
}

#[test]
fn stable_contracts_are_available_from_the_crate_root() {
    assert_argument_source::<EnvironmentArguments>();
    assert_output_sink::<StandardStreams>();
    assert_eq!(size_of::<RunInvocation>(), 0);
}
