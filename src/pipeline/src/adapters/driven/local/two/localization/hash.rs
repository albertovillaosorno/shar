// File:
//   - hash.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/localization/hash.rs
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
//   - Stable localization key hashing.
// - Must-Not:
//   - Accept a zero modulus or hide invalid explicit hash syntax.
// - Allows:
//   - Legacy-compatible modular hashing and explicit hexadecimal identities.
// - Split-When:
//   - Another identity algorithm requires a separately versioned contract.
// - Merge-When:
//   - Another localization module owns the same key-to-hash mapping.
// - Summary:
//   - Deterministic localization identity functions.
// - Description:
//   - Keeps hashing independent from source IO and package classification.
// - Usage:
//   - Used by overlay merge and future StringTable planning.
// - Defaults:
//   - Malformed source data fails closed without implicit output.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Stable text hashing preserves source identities across pipeline runs.

use super::{Error, Outcome};

/// Reject a zero modulus before any key identity path executes.
fn validate_modulo(modulo: u32) -> Outcome<()> {
    if modulo == 0 {
        return Err(
            Error::invalid("language hash modulus must be greater than zero"),
        );
    }
    Ok(())
}

/// Compute the legacy language hash for one text key.
///
/// # Errors
///
/// Returns an error when the modulus is zero.
pub(super) fn hash_key(
    key: &str,
    modulo: u32,
) -> Outcome<u32> {
    validate_modulo(modulo)?;
    let mut hash = 0_u32;
    for byte in key.bytes() {
        let mixed = hash
            .wrapping_mul(64)
            .wrapping_add(u32::from(byte));
        hash = mixed
            .checked_rem(modulo)
            .ok_or_else(
                || Error::invalid("language hash modulus became invalid"),
            )?;
    }
    Ok(hash)
}

/// Resolve an explicit hexadecimal identity or hash a text key.
///
/// # Errors
///
/// Returns an error for malformed explicit hashes or a zero modulus.
pub(super) fn custom_entry_hash(
    key: &str,
    modulo: u32,
) -> Outcome<u32> {
    validate_modulo(modulo)?;
    let trimmed = key.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return u32::from_str_radix(
            hex, 16,
        )
        .map_err(
            |error| {
                Error::invalid(
                    format!("custom-text hash '{trimmed}' is invalid: {error}"),
                )
            },
        );
    }
    hash_key(
        trimmed, modulo,
    )
}

#[cfg(test)]
mod tests {
    use super::{custom_entry_hash, hash_key};

    #[test]
    fn hashes_known_key_deterministically() -> Result<(), String> {
        let first = hash_key(
            "MISSION_TITLE_L1",
            1009,
        )
        .map_err(|error| error.to_string())?;
        let second = hash_key(
            "MISSION_TITLE_L1",
            1009,
        )
        .map_err(|error| error.to_string())?;
        if first == second {
            Ok(())
        } else {
            Err("stable hash changed across repeated calls".to_owned())
        }
    }

    #[test]
    fn rejects_zero_modulus() -> Result<(), String> {
        if hash_key(
            "KEY", 0,
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("zero hash modulus was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_zero_modulus_for_explicit_hash() -> Result<(), String> {
        if custom_entry_hash(
            "0x10", 0,
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("explicit hash accepted zero modulus".to_owned())
        }
    }

    #[test]
    fn parses_explicit_hash() -> Result<(), String> {
        let value = custom_entry_hash(
            "0x10", 1009,
        )
        .map_err(|error| error.to_string())?;
        if value == 16 {
            Ok(())
        } else {
            Err(format!("unexpected explicit hash: {value}"))
        }
    }
}
