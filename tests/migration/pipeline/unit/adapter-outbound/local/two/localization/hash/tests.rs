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

use super::{custom_entry_hash, hash_key};

#[test]
fn hashes_known_key_deterministically() -> Result<(), String> {
    let first = hash_key("MISSION_TITLE_L1", 1009)
        .map_err(|error| error.to_string())?;
    let second = hash_key("MISSION_TITLE_L1", 1009)
        .map_err(|error| error.to_string())?;
    if first == second {
        Ok(())
    } else {
        Err("stable hash changed across repeated calls".to_owned())
    }
}

#[test]
fn rejects_zero_modulus() -> Result<(), String> {
    if hash_key("KEY", 0).is_err() {
        Ok(())
    } else {
        Err("zero hash modulus was accepted".to_owned())
    }
}

#[test]
fn rejects_zero_modulus_for_explicit_hash() -> Result<(), String> {
    if custom_entry_hash("0x10", 0).is_err() {
        Ok(())
    } else {
        Err("explicit hash accepted zero modulus".to_owned())
    }
}

#[test]
fn parses_explicit_hash() -> Result<(), String> {
    let value =
        custom_entry_hash("0x10", 1009).map_err(|error| error.to_string())?;
    if value == 16 {
        Ok(())
    } else {
        Err(format!("unexpected explicit hash: {value}"))
    }
}
