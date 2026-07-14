// File:
//   - malformed_binary_controls.rs
// Path:
//   - src/rtf/tests/malformed_binary_controls.rs
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
//   - Deterministic RTF regression coverage for malformed binary controls.
// - Must-Not:
//   - Depend on private documents or parser implementation details.
// - Allows:
//   - Public malformed fixtures and fail-closed output assertions.
// - Split-When:
//   - Split when another binary-control family needs independent fixtures.
// - Merge-When:
//   - Another RTF test module owns the same malformed-binary contract.
// - Summary:
//   - Verifies invalid binary lengths cannot expose opaque payload bytes.
// - Description:
//   - Exercises public fail-closed behavior for missing and negative lengths.
// - Usage:
//   - Executed through cargo test for the rtf crate.
// - Defaults:
//   - Fixtures remain deterministic and repository-local.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Public regression coverage for malformed RTF binary controls.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn invalid_binary_lengths_hide_opaque_group_payloads() {
    let missing = rtf_to_markdown(br"{\rtf1 A{\bin payload}B}");
    let negative = rtf_to_markdown(br"{\rtf1 A{\bin-1 payload}B}");

    assert_eq!(
        missing,
        "AB\n"
    );
    assert_eq!(
        negative,
        "AB\n"
    );
}
