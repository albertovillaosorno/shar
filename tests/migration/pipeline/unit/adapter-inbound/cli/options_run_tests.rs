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
//   - Options run tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Options run tests test module.
// - Description:
//   - Implements the declared test module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Options run tests test module.

use std::path::PathBuf;

use super::parse_common_arguments;
use crate::adapters::driven::RunMode;

#[test]
fn concurrent_run_label_and_default_log_isolation_are_explicit()
-> Result<(), String> {
    let parsed = parse_common_arguments(&[
        String::from("--allow-concurrent"),
        String::from("--run-label=world-b"),
        String::from("game"),
    ])?;
    if parsed.run_mode() != RunMode::Concurrent {
        return Err(String::from("concurrent mode was not selected"));
    }
    if parsed.run_label().as_deref() != Some("world-b") {
        return Err(String::from("run label was not preserved"));
    }
    let expected = PathBuf::from(".logs")
        .join("pipeline")
        .join("runs")
        .join("run-test.jsonl");
    if parsed.log_file_for_run("run-test") != Some(expected) {
        return Err(String::from("concurrent default log was not isolated"));
    }
    if parsed.positionals != [String::from("game")] {
        return Err(String::from("run options changed command positionals"));
    }
    Ok(())
}

#[test]
fn explicit_concurrent_log_path_is_preserved() -> Result<(), String> {
    let parsed = parse_common_arguments(&[
        String::from("--allow-concurrent"),
        String::from("--log=.logs/custom/shared.jsonl"),
    ])?;
    let expected = Some(PathBuf::from(".logs/custom/shared.jsonl"));
    if parsed.log_file_for_run("run-test") != expected {
        return Err(String::from("explicit concurrent log path changed"));
    }
    Ok(())
}

#[test]
fn repeated_or_nonportable_run_options_are_rejected() {
    for arguments in [
        vec![
            String::from("--allow-concurrent"),
            String::from("--allow-concurrent"),
        ],
        vec![
            String::from("--run-label=first"),
            String::from("--run-label=second"),
        ],
        vec![String::from("--run-label=has space")],
        vec![String::from("--run-label=../escape")],
        vec![String::from("--run-label=")],
    ] {
        assert!(parse_common_arguments(&arguments).is_err());
    }
}
