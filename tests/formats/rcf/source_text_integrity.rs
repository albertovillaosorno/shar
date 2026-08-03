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
//   - Source text integrity test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Source text integrity test module.
// - Description:
//   - Implements the declared test module responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source text integrity test module.

use std::fs;
use std::path::PathBuf;

use rcf as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn parser_fixture_sources_do_not_contain_nul_bytes() {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let sources = [
        "tests/formats/rcf/archive_parser.rs",
        "tests/formats/rcf/fixture/archive.rs",
    ];
    for relative in sources {
        let source = repository.join(relative);
        let read_result = fs::read(&source);
        assert!(
            read_result.is_ok(),
            "the guarded RCF source must remain readable: {relative}"
        );
        let Ok(source_bytes) = read_result else {
            continue;
        };
        let nul_offset = source_bytes.iter().position(|byte| *byte == 0);
        assert!(
            nul_offset.is_none(),
            "{relative} contains a NUL byte at offset {nul_offset:?}"
        );
    }
}
