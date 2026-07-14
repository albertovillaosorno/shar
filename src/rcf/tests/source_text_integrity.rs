// File:
//   - source_text_integrity.rs
// Path:
//   - src/rcf/tests/source_text_integrity.rs
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
//   - RCF Rust source text-integrity regressions.
// - Must-Not:
//   - Read private assets, generated outputs, or files outside the RCF crate.
// - Allows:
//   - Deterministic reads of checked-in Rust source under src and tests.
// - Split-When:
//   - Another source-format invariant requires an independent test target.
// - Merge-When:
//   - RCF source text integrity no longer needs a distinct integration test.
// - Summary:
//   - Protects RCF Rust files from binary control bytes.
// - Description:
//   - Scans crate-owned Rust sources so text tooling can inspect every file.
// - Usage:
//   - Run through the RCF crate integration test target.
// - Defaults:
//   - Only CARGO_MANIFEST_DIR/src and CARGO_MANIFEST_DIR/tests are scanned.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! RCF Rust source text-integrity regressions.
//!
//! Fixture-oriented Rust sources must remain plain text so repository search,
//! diff, classification, and review tools never treat them as binary files.

use std::fs;
use std::path::PathBuf;

use rcf as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn parser_fixture_sources_do_not_contain_nul_bytes() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sources = [
        "tests/archive_parser.rs",
        "tests/fixture/archive.rs",
    ];
    for relative in sources {
        let source = manifest.join(relative);
        let read_result = fs::read(&source);
        assert!(
            read_result.is_ok(),
            "the guarded RCF source must remain readable: {relative}"
        );
        let Ok(source_bytes) = read_result else {
            continue;
        };
        let nul_offset = source_bytes
            .iter()
            .position(|byte| *byte == 0);
        assert!(
            nul_offset.is_none(),
            "{relative} contains a NUL byte at offset {nul_offset:?}"
        );
    }
}
