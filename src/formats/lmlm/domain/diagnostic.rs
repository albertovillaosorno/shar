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
//   - Diagnostic domain helper.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Diagnostic domain helper.
// - Description:
//   - Implements the declared application service responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Diagnostic domain helper.

#![expect(
    clippy::redundant_pub_crate,
    reason = "domain and composition layers require one crate-visible \
              diagnostic contract"
)]

use core::fmt::{self, Write as _};

/// Returns one owned rendering of untrusted text without control characters.
pub(crate) fn escaped_string(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        output.extend(character.escape_default());
    }
    output
}

/// Display wrapper that escapes every untrusted character.
pub(crate) struct EscapedText<'a>(&'a str);

impl<'a> EscapedText<'a> {
    /// Wraps one untrusted diagnostic value.
    pub(crate) const fn new(value: &'a str) -> Self {
        Self(value)
    }
}

impl fmt::Display for EscapedText<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for character in self.0.chars() {
            for escaped in character.escape_default() {
                formatter.write_char(escaped)?;
            }
        }
        Ok(())
    }
}
