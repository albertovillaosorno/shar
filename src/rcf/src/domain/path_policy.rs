// File:
//   - path_policy.rs
// Path:
//   - src/rcf/src/domain/path_policy.rs
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
//   - Host-independent Unicode path-control classification.
// - Must-Not:
//   - Perform filesystem IO or apply adapter-specific naming limits.
// - Allows:
//   - Pure classification shared by parser and filesystem boundaries.
// - Split-When:
//   - A second independent Unicode path policy needs separate ownership.
// - Merge-When:
//   - Unicode path controls are no longer shared across RCF boundaries.
// - Summary:
//   - Classifies invisible Unicode path controls.
// - Description:
//   - Centralizes scalars that can reorder, hide, or merge displayed paths.
// - Usage:
//   - Called before archive names or direct sink components become paths.
// - Defaults:
//   - Ordinary Unicode letters and symbols remain accepted.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Pure Unicode path-control classification.
//!
//! Parser and adapter boundaries share one deterministic spoofing policy.

/// Unicode direction and zero-width controls that can spoof displayed paths.
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
