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
//   - Path helpers test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Path helpers test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Path helpers test module.

use std::path::Path;

use game_manifest::{NO_EXTENSION, extension_of, obfuscate_component};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn extension_of_treats_trailing_dot_as_missing() {
    assert_eq!(extension_of(Path::new("asset.")), NO_EXTENSION);
}

#[test]
fn extension_of_lowercases_unicode() {
    assert_eq!(extension_of(Path::new("asset.ÄBC")), "äbc");
}

#[test]
fn obfuscate_component_lowercases_unicode() {
    assert_eq!(obfuscate_component("ÄZ"), "äz");
}
