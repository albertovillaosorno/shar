// File:
//   - run.rs
// Path:
//   - src/pipeline/src/adapters/driving/cli/options/run.rs
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
//   - Cooperative-run CLI options, labels, and concurrent log isolation.
// - Must-Not:
//   - Acquire runtime leases, inspect active processes, or execute commands.
// - Allows:
//   - Parse portable run selectors and derive one local diagnostic log path.
// - Summary:
//   - Pipeline run-registry command options.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - true
//   - Reason: parsing and log isolation share one run-option contract.
//   - Split: separate label validation if labels gain another consumer.
//   - Validation: option regressions and canonical pipeline validation.
//   - Review: required when concurrency acknowledgement changes.
//

//! Cooperative run-registry options for pipeline commands.

use std::path::{Path, PathBuf};

use crate::adapters::driven::RunMode;

/// Maximum portable display-label length.
const MAX_LABEL_BYTES: usize = 64;

/// Parsed cooperative-run selectors for one normal pipeline invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RunOptions {
    /// Default exclusive or explicitly concurrent execution mode.
    pub(super) mode: RunMode,
    /// Optional portable display label shown by `pipeline active`.
    pub(super) label: Option<String>,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            mode: RunMode::Exclusive,
            label: None,
        }
    }
}

impl RunOptions {
    /// Derive one isolated default log for an explicitly concurrent run.
    pub(super) fn log_file_for_run(
        &self,
        configured: Option<&Path>,
        logging_explicit: bool,
        run_id: &str,
    ) -> Option<PathBuf> {
        if self.mode == RunMode::Concurrent
            && !logging_explicit
            && configured.is_some()
        {
            Some(
                PathBuf::from("logs")
                    .join("pipeline")
                    .join("runs")
                    .join(format!("{run_id}.jsonl")),
            )
        } else {
            configured.map(Path::to_path_buf)
        }
    }
}

/// Parse one cooperative-run option and return its consumed argument count.
///
/// # Errors
///
/// Returns an error for repeated selectors, missing labels, or nonportable
/// display-label identities.
pub(super) fn parse_run_option(
    arguments: &[String],
    index: usize,
    argument: &str,
    options: &mut RunOptions,
) -> Result<Option<usize>, String> {
    if argument == "--allow-concurrent" {
        if options.mode == RunMode::Concurrent {
            return Err(
                String::from("--allow-concurrent may be specified only once"),
            );
        }
        options.mode = RunMode::Concurrent;
        return Ok(Some(1));
    }
    if argument == "--run-label" {
        let value = arguments
            .get(index.saturating_add(1))
            .ok_or_else(|| String::from("--run-label requires a value"))?;
        if value.starts_with('-') {
            return Err(format!("--run-label requires a value before {value}"));
        }
        select_label(
            options, value,
        )?;
        return Ok(Some(2));
    }
    let Some(value) = argument.strip_prefix("--run-label=") else {
        return Ok(None);
    };
    select_label(
        options, value,
    )?;
    Ok(Some(1))
}

/// Select one portable run label without order-dependent overrides.
fn select_label(
    options: &mut RunOptions,
    value: &str,
) -> Result<(), String> {
    if options
        .label
        .is_some()
    {
        return Err(String::from("run label may be specified only once"));
    }
    validate_label(value)?;
    options.label = Some(value.to_owned());
    Ok(())
}

/// Validate one path-safe display label.
fn validate_label(value: &str) -> Result<(), String> {
    let valid = !value.is_empty()
        && value.len() <= MAX_LABEL_BYTES
        && value == value.trim()
        && value
            .bytes()
            .all(
                |byte| {
                    byte.is_ascii_alphanumeric()
                        || matches!(
                            byte,
                            b'-' | b'_' | b'.'
                        )
                },
            );
    if valid {
        Ok(())
    } else {
        Err(
            String::from(
                "--run-label must be 1-64 ASCII letters, digits, dots, \
                 dashes, or underscores",
            ),
        )
    }
}
