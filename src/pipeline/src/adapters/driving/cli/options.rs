// File:
//   - options.rs
// Path:
//   - src/pipeline/src/adapters/driving/cli/options.rs
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
//   - Common pipeline verbosity and diagnostic-log command options.
// - Must-Not:
//   - Execute pipeline stages or infer machine-specific filesystem roots.
// - Allows:
//   - Separate process options from command-specific positional arguments.
// - Split-When:
//   - Split when another option family gains independent parsing policy.
// - Merge-When:
//   - The pipeline CLI can own these options without obscuring commands.
// - Summary:
//   - Parses portable pipeline progress and logging options.
// - Description:
//   - Resolves detailed or minimal progress, local log selection, and
//   - untouched positional arguments for the driving adapter.
// - Usage:
//   - Called once after the command name is validated.
// - Defaults:
//   - Detailed progress and `logs/pipeline/latest.jsonl` logging are enabled.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Parses portable progress and diagnostic-log options for pipeline commands.
use std::path::PathBuf;

use self::log::{DEFAULT_LOG_FILE, parse_log_option, select_logging};
use self::run::{RunOptions, parse_run_option};
use crate::adapters::driven::ProgressVerbosity;

mod log;
mod run;

/// Common process options separated from command-specific positionals.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct ParsedArguments {
    /// Command-specific positional values in their original order.
    pub(super) positionals: Vec<String>,
    /// Selected terminal detail level.
    pub(super) verbosity: ProgressVerbosity,
    /// Selected latest-run log, or `None` when logging is disabled.
    pub(super) log_file: Option<PathBuf>,
    /// Whether the caller explicitly selected the logging policy.
    pub(super) logging_explicit: bool,
    /// Whether `fbx-export` should embed compatibility texture payloads.
    pub(super) embed_textures: bool,
    /// Cooperative process-registry selectors.
    run: RunOptions,
}

impl ParsedArguments {
    /// Return the selected cooperative execution mode.
    pub(super) const fn run_mode(&self) -> crate::adapters::driven::RunMode {
        self.run
            .mode
    }

    /// Return the optional caller-provided run label.
    pub(super) fn run_label(&self) -> Option<String> {
        self.run
            .label
            .clone()
    }

    /// Resolve the diagnostic log for one acquired run identity.
    pub(super) fn log_file_for_run(
        &self,
        run_id: &str,
    ) -> Option<PathBuf> {
        self.run
            .log_file_for_run(
                self.log_file
                    .as_deref(),
                self.logging_explicit,
                run_id,
            )
    }
}

impl Default for ParsedArguments {
    fn default() -> Self {
        Self {
            positionals: Vec::new(),
            verbosity: ProgressVerbosity::Detailed,
            log_file: Some(PathBuf::from(DEFAULT_LOG_FILE)),
            logging_explicit: false,
            embed_textures: false,
            run: RunOptions::default(),
        }
    }
}

/// Select one explicit verbosity without order-dependent overrides.
fn select_verbosity(
    verbosity: &mut ProgressVerbosity,
    selected: &mut bool,
    value: ProgressVerbosity,
) -> Result<(), String> {
    if *selected {
        return Err(String::from("verbosity may be specified only once"));
    }
    *verbosity = value;
    *selected = true;
    Ok(())
}

/// Parse progress and logging options without changing positional paths.
///
/// # Errors
///
/// Returns an error for unknown options, missing values, empty log paths, or
/// unsupported verbosity names.
pub(super) fn parse_common_arguments(
    arguments: &[String]
) -> Result<ParsedArguments, String> {
    let mut parsed = ParsedArguments::default();
    let mut verbosity_selected = false;
    let mut logging_selected = false;
    let mut parse_options = true;
    let mut index = 0usize;
    while let Some(argument) = arguments.get(index) {
        if parse_options && argument == "--" {
            parse_options = false;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument == "--verbosity" {
            let value = arguments
                .get(index.saturating_add(1))
                .ok_or_else(|| String::from("--verbosity requires a value"))?;
            if value.starts_with('-') {
                return Err(
                    format!("--verbosity requires a value before {value}"),
                );
            }
            select_verbosity(
                &mut parsed.verbosity,
                &mut verbosity_selected,
                parse_verbosity(value)?,
            )?;
            index = index.saturating_add(2);
            continue;
        }
        if parse_options {
            let parsed_log_option = parse_log_option(
                arguments, index, argument,
            )?;
            if let Some(log_option) = parsed_log_option {
                select_logging(
                    &mut parsed.log_file,
                    &mut logging_selected,
                    log_option.log_file,
                )?;
                parsed.logging_explicit = true;
                index = index.saturating_add(log_option.consumed);
                continue;
            }
        }
        if parse_options
            && let Some(consumed) = parse_run_option(
                arguments,
                index,
                argument,
                &mut parsed.run,
            )?
        {
            index = index.saturating_add(consumed);
            continue;
        }
        if parse_options && argument == "--minimal" {
            select_verbosity(
                &mut parsed.verbosity,
                &mut verbosity_selected,
                ProgressVerbosity::Minimal,
            )?;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument == "--detailed" {
            select_verbosity(
                &mut parsed.verbosity,
                &mut verbosity_selected,
                ProgressVerbosity::Detailed,
            )?;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument == "--embed-textures" {
            parsed.embed_textures = true;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options
            && let Some(value) = argument.strip_prefix("--verbosity=")
        {
            select_verbosity(
                &mut parsed.verbosity,
                &mut verbosity_selected,
                parse_verbosity(value)?,
            )?;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument.starts_with('-') {
            return Err(format!("unknown option: {argument}"));
        }
        parsed
            .positionals
            .push(argument.clone());
        index = index.saturating_add(1);
    }
    Ok(parsed)
}

/// Parse one exact supported verbosity value.
fn parse_verbosity(value: &str) -> Result<ProgressVerbosity, String> {
    match value {
        "detailed" => Ok(ProgressVerbosity::Detailed),
        "minimal" => Ok(ProgressVerbosity::Minimal),
        _ => Err(
            format!(
                "unsupported verbosity {value:?}; expected detailed or minimal"
            ),
        ),
    }
}

#[cfg(test)]
#[path = "options_run_tests.rs"]
mod run_tests;
#[cfg(test)]
#[path = "options_tests.rs"]
mod tests;
