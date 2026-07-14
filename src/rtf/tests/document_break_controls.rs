// File:
//   - document_break_controls.rs
// Path:
//   - src/rtf/tests/document_break_controls.rs
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
//   - Deterministic RTF regression coverage for document break controls.
// - Must-Not:
//   - Depend on private documents or parser implementation details.
// - Allows:
//   - Public fixtures and caller-visible Markdown structure assertions.
// - Split-When:
//   - Split when another break family needs independent output semantics.
// - Merge-When:
//   - Another RTF test module owns the same document-break contract.
// - Summary:
//   - Verifies that RTF line and paragraph controls remain distinct.
// - Description:
//   - Exercises public conversion behavior for hard lines and paragraphs.
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

//! Public regression coverage for RTF document break controls.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn line_and_paragraph_controls_keep_distinct_markdown_structure() {
    let markdown = rtf_to_markdown(br"{\rtf1 first\line second\par third}");

    assert_eq!(
        markdown,
        "first<br>\nsecond\n\nthird\n"
    );
}
