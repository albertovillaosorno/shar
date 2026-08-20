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
//   - Unit evidence for algorithm CLI argument parsing.
// - Must-Not:
//   - Perform filesystem reconstruction or depend on proprietary source.
// - Allows:
//   - Exercise private CLI parsing through the source-owned test module.
// - Split-When:
//   - Additional CLI modes gain independent parsing contracts.
// - Merge-When:
//   - Another test module owns the identical CLI parsing evidence.
// - Summary:
//   - Algorithm CLI parser unit tests.
// - Description:
//   - Proves repeated source arguments retain their create-mode shape.
// - Usage:
//   - Loaded by the algorithm CLI module under cfg(test).
// - Defaults:
//   - Synthetic argument strings only.
//

//! Algorithm CLI parser unit tests.

use super::{Mode, parse};

#[test]
fn create_shape_accepts_repeated_sources() {
    let arguments = [
        "create", "--source", "one", "--source", "two", "--target", "target",
        "--output", "plan.txt",
    ]
    .map(str::to_owned);
    let parsed = parse(&arguments)
        .map(|invocation| (invocation.mode, invocation.sources.len()));
    assert_eq!(
        parsed,
        Ok((Mode::Create, 2)),
        "valid create invocation should retain both sources"
    );
}

#[test]
fn create_shape_associates_projection_with_previous_source() {
    let arguments = [
        "create",
        "--source",
        "canonical.bin",
        "--source-projection",
        "mask.json",
        "--source",
        "plain.bin",
        "--target",
        "target",
        "--output",
        "plan.txt",
    ]
    .map(str::to_owned);
    let parsed = parse(&arguments)
        .map(|invocation| (invocation.mode, invocation.source_projections));
    assert_eq!(
        parsed,
        Ok((Mode::Create, vec![Some("mask.json".into()), None])),
        "projection should bind only to its preceding create source"
    );
}

#[test]
fn replay_shape_rejects_authoring_projection_argument() {
    let arguments = [
        "replay",
        "--source",
        "variant.bin",
        "--source-projection",
        "mask.json",
        "--algorithm",
        "plan.txt",
        "--output",
        "output",
    ]
    .map(str::to_owned);
    assert!(
        parse(&arguments).is_err(),
        "replay must use the authenticated projection from the plan"
    );
}
