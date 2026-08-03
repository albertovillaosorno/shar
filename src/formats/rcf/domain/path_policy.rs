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
//   - Path policy domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Path policy domain module.
// - Description:
//   - Implements the declared domain module responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Path policy domain module.

const UNSAFE_UNICODE_PATH_CONTROLS: [char; 23] = [
    '\u{00ad}', '\u{061c}', '\u{180e}', '\u{200b}', '\u{200c}', '\u{200d}',
    '\u{200e}', '\u{200f}', '\u{202a}', '\u{202b}', '\u{202c}', '\u{202d}',
    '\u{202e}', '\u{2060}', '\u{2061}', '\u{2062}', '\u{2063}', '\u{2064}',
    '\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}', '\u{feff}',
];

/// Reports whether text contains an invisible Unicode path control.
// The crate-private policy is consumed by sibling parser and adapter modules.
#[expect(
    clippy::redundant_pub_crate,
    reason = "Sibling RCF boundaries share one domain path policy."
)]
pub(crate) fn contains_unsafe_unicode_path_control(value: &str) -> bool {
    value
        .chars()
        .any(|character| UNSAFE_UNICODE_PATH_CONTROLS.contains(&character))
}
