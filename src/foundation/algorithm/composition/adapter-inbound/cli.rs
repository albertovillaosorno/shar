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
//   - Command-line parsing and process orchestration for algorithm operations.
// - Must-Not:
//   - Own source admission or reconstruction policy.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Algorithm CLI adapter.
// - Description:
//   - Command-line parsing and process orchestration for algorithm operations.
// - Usage:
//   - Used through the owning algorithm function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Command-line adapter for generic algorithm authoring and replay.

use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};
use schoenwald_filesystem::adapters::driving::local;

use crate::{Settings, create_algorithm, replay_algorithm};

const USAGE: &str = "usage: algorithm <create|replay> --source <PATH>... \
[--settings <SETTINGS.json>] (--target <PATH> | --algorithm <FILE.txt>) \
--output <PATH>";
const DEFAULT_SETTINGS: &str =
    "src/foundation/algorithm/composition/adapter-inbound/settings.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Create,
    Replay,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Invocation {
    mode: Mode,
    sources: Vec<PathBuf>,
    settings: PathBuf,
    target: Option<PathBuf>,
    algorithm: Option<PathBuf>,
    output: PathBuf,
}

fn take_value<'a>(
    values: &mut impl Iterator<Item = &'a String>,
) -> Result<&'a str, ()> {
    values
        .next()
        .map(String::as_str)
        .filter(|value| !value.is_empty())
        .ok_or(())
}

fn set_once(slot: &mut Option<PathBuf>, value: &str) -> Result<(), ()> {
    if slot.is_some() {
        return Err(());
    }
    *slot = Some(PathBuf::from(value));
    Ok(())
}

fn parse(arguments: &[String]) -> Result<Invocation, ()> {
    let mut values = arguments.iter();
    let mode = match values.next().map(String::as_str) {
        Some("create") => Mode::Create,
        Some("replay") => Mode::Replay,
        _ => return Err(()),
    };
    let mut sources = Vec::new();
    let mut settings = None;
    let mut target = None;
    let mut algorithm = None;
    let mut output = None;
    while let Some(flag) = values.next() {
        match flag.as_str() {
            "--source" => sources.push(PathBuf::from(take_value(&mut values)?)),
            "--settings" => set_once(&mut settings, take_value(&mut values)?)?,
            "--target" => set_once(&mut target, take_value(&mut values)?)?,
            "--algorithm" => {
                set_once(&mut algorithm, take_value(&mut values)?)?;
            },
            "--output" => set_once(&mut output, take_value(&mut values)?)?,
            _ => return Err(()),
        }
    }
    let output = output.ok_or(())?;
    let valid_shape = !sources.is_empty()
        && match mode {
            Mode::Create => target.is_some() && algorithm.is_none(),
            Mode::Replay => target.is_none() && algorithm.is_some(),
        };
    if !valid_shape {
        return Err(());
    }
    Ok(Invocation {
        mode,
        sources,
        settings: settings.unwrap_or_else(|| PathBuf::from(DEFAULT_SETTINGS)),
        target,
        algorithm,
        output,
    })
}

fn execute_invocation(invocation: &Invocation) -> Result<(), String> {
    let settings_text = local::read_utf8(&invocation.settings)
        .map_err(|error| format!("cannot read algorithm settings: {error}"))?;
    let settings = Settings::from_json(&settings_text)
        .map_err(|error| error.to_string())?;
    match invocation.mode {
        Mode::Create => create_algorithm(
            &settings,
            &invocation.sources,
            invocation
                .target
                .as_deref()
                .ok_or_else(|| USAGE.to_owned())?,
            &invocation.output,
        ),
        Mode::Replay => replay_algorithm(
            &settings,
            &invocation.sources,
            invocation
                .algorithm
                .as_deref()
                .ok_or_else(|| USAGE.to_owned())?,
            &invocation.output,
        ),
    }
    .map_err(|error| error.to_string())
}

/// Process-neutral algorithm command program.
#[derive(Debug, Default, Clone, Copy)]
pub struct AlgorithmProgram;

impl CliProgram for AlgorithmProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        if matches!(arguments, [argument] if argument == "--help") {
            return CommandOutcome::success().stdout_line(USAGE);
        }
        let Ok(invocation) = parse(arguments) else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        match execute_invocation(&invocation) {
            Ok(()) => CommandOutcome::success()
                .stdout_line("algorithm operation completed"),
            Err(error) => CommandOutcome::failure().stderr_line(error),
        }
    }
}

/// Executes the algorithm CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&AlgorithmProgram)
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/foundation/algorithm/unit/adapter-inbound/cli/tests.rs"]
mod tests;
