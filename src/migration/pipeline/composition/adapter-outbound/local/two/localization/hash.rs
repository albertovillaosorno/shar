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
//   - Hash outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Hash outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Hash outbound adapter.

use super::{Error, Outcome};

/// Reject a zero modulus before any key identity path executes.
fn validate_modulo(modulo: u32) -> Outcome<()> {
    if modulo == 0 {
        return Err(Error::invalid(
            "language hash modulus must be greater than zero",
        ));
    }
    Ok(())
}

/// Compute the legacy language hash for one text key.
///
/// # Errors
///
/// Returns an error when the modulus is zero.
pub(super) fn hash_key(key: &str, modulo: u32) -> Outcome<u32> {
    validate_modulo(modulo)?;
    let mut hash = 0_u32;
    for byte in key.bytes() {
        let mixed = hash.wrapping_mul(64).wrapping_add(u32::from(byte));
        hash = mixed.checked_rem(modulo).ok_or_else(|| {
            Error::invalid("language hash modulus became invalid")
        })?;
    }
    Ok(hash)
}

/// Resolve an explicit hexadecimal identity or hash a text key.
///
/// # Errors
///
/// Returns an error for malformed explicit hashes or a zero modulus.
pub(super) fn custom_entry_hash(key: &str, modulo: u32) -> Outcome<u32> {
    validate_modulo(modulo)?;
    let trimmed = key.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return u32::from_str_radix(hex, 16).map_err(|error| {
            Error::invalid(format!(
                "custom-text hash '{trimmed}' is invalid: {error}"
            ))
        });
    }
    hash_key(trimmed, modulo)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/localization/hash/tests.rs"]
mod tests;
