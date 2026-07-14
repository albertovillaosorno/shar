// File:
//   - diagnostic.rs
// Path:
//   - src/lmlm/src/diagnostic.rs
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
//   - Single-line rendering of untrusted crate diagnostic text.
// - Must-Not:
//   - Classify errors, perform IO, or select presentation streams.
// - Allows:
//   - Share escaped text rendering across domain, application, and adapters.
// - Split-When:
//   - Another diagnostic encoding requires independent policy.
// - Merge-When:
//   - No crate error renders untrusted text.
// - Summary:
//   - Escapes control characters in crate diagnostics.
// - Description:
//   - Preserves evidence without permitting terminal-line injection.
// - Usage:
//   - Used by parser, extraction, and filesystem error displays.
// - Defaults:
//   - Every nonliteral character uses `escape_default`.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Escaped crate diagnostic text.
//!
//! Control characters and nonliteral Unicode are rendered visibly.

// Sibling layers share this contract without exposing it publicly.
#![expect(
    clippy::redundant_pub_crate,
    reason = "application and adapter layers require one crate-visible \
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
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for character in self
            .0
            .chars()
        {
            for escaped in character.escape_default() {
                formatter.write_char(escaped)?;
            }
        }
        Ok(())
    }
}
