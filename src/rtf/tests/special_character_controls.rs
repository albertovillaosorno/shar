// File:
//   - special_character_controls.rs
// Path:
//   - src/rtf/tests/special_character_controls.rs
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
//   - Deterministic RTF regression coverage for special character controls.
// - Must-Not:
//   - Depend on private inputs or implementation-specific parser structure.
// - Allows:
//   - Public conversion fixtures and caller-visible Markdown assertions.
// - Split-When:
//   - Split when special character controls require separate fixture families.
// - Merge-When:
//   - Another RTF test module owns the same special-character contract.
// - Summary:
//   - Verifies semantic preservation of RTF special character controls.
// - Description:
//   - Exercises public RTF conversion behavior for character control symbols.
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

//! Public regression coverage for RTF special character controls.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn nonbreaking_hyphen_remains_nonbreaking() {
    let markdown = rtf_to_markdown(br"{\rtf1 co\_operate}");

    assert_eq!(
        markdown,
        "co‑operate\n"
    );
}
