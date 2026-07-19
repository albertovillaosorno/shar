// File:
//   - options_run_tests.rs
// Path:
//   - src/pipeline/src/adapters/driving/cli/options_run_tests.rs
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
//   - Cooperative-run option parsing and log-isolation regressions.
// - Must-Not:
//   - Acquire runtime leases, inspect processes, or execute pipeline stages.
// - Allows:
//   - Parse explicit argument vectors and compare derived run options.
// - Summary:
//   - Pipeline run-option parser tests.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression tests for cooperative pipeline run options.

use std::path::PathBuf;

use super::parse_common_arguments;
use crate::adapters::driven::RunMode;

#[test]
fn concurrent_run_label_and_default_log_isolation_are_explicit()
-> Result<(), String> {
    let parsed = parse_common_arguments(
        &[
            String::from("--allow-concurrent"),
            String::from("--run-label=world-b"),
            String::from("game"),
        ],
    )?;
    if parsed.run_mode() != RunMode::Concurrent {
        return Err(String::from("concurrent mode was not selected"));
    }
    if parsed
        .run_label()
        .as_deref()
        != Some("world-b")
    {
        return Err(String::from("run label was not preserved"));
    }
    let expected = PathBuf::from("logs")
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
    let parsed = parse_common_arguments(
        &[
            String::from("--allow-concurrent"),
            String::from("--log=logs/custom/shared.jsonl"),
        ],
    )?;
    let expected = Some(PathBuf::from("logs/custom/shared.jsonl"));
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
