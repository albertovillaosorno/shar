// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Log inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Log inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Log inbound adapter.

use std::path::{Path, PathBuf};

use schoenwald_filesystem::resolve_under;

/// Stable latest-run log relative to the pipeline working directory.
pub(super) const DEFAULT_LOG_FILE: &str = ".logs/pipeline/latest.jsonl";

/// One parsed logging selector and its consumed argument count.
pub(super) struct ParsedLogOption {
    /// Selected log destination, or `None` when logging is disabled.
    pub(super) log_file: Option<PathBuf>,
    /// Number of command arguments consumed by this selector.
    pub(super) consumed: usize,
}

/// Parse one logging selector without applying order-dependent overrides.
pub(super) fn parse_log_option(
    arguments: &[String],
    index: usize,
    argument: &str,
) -> Result<Option<ParsedLogOption>, String> {
    if argument == "--log" {
        let value = arguments
            .get(index.saturating_add(1))
            .ok_or_else(|| String::from("--log requires a path"))?;
        if value.starts_with('-') {
            return Err(format!("--log requires a path before {value}"));
        }
        return Ok(Some(ParsedLogOption {
            log_file: Some(parse_log_path(value)?),
            consumed: 2,
        }));
    }
    if argument == "--no-log" {
        return Ok(Some(ParsedLogOption {
            log_file: None,
            consumed: 1,
        }));
    }
    let Some(value) = argument.strip_prefix("--log=") else {
        return Ok(None);
    };
    Ok(Some(ParsedLogOption {
        log_file: Some(parse_log_path(value)?),
        consumed: 1,
    }))
}

/// Select one explicit logging mode without order-dependent overrides.
pub(super) fn select_logging(
    log_file: &mut Option<PathBuf>,
    selected: &mut bool,
    value: Option<PathBuf>,
) -> Result<(), String> {
    if *selected {
        return Err(String::from("logging may be specified only once"));
    }
    *log_file = value;
    *selected = true;
    Ok(())
}

/// Parse one nonempty portable log path.
fn parse_log_path(value: &str) -> Result<PathBuf, String> {
    if value.is_empty() {
        return Err(String::from("--log path must not be empty"));
    }
    if value.trim().is_empty() {
        return Err(String::from("--log path must not be blank"));
    }
    if value.ends_with('/') || value.ends_with(char::from(92)) {
        return Err(String::from("--log path must identify a file"));
    }
    let validation = resolve_under(Path::new("."), Path::new(value));
    let validated = match validation {
        Ok(path) => path,
        Err(error) => {
            return Err(format!("invalid --log path {value:?}: {error}"));
        },
    };
    let Ok(relative) = validated.strip_prefix(Path::new(".")) else {
        return Err(String::from("failed to normalize --log path"));
    };
    Ok(relative.to_path_buf())
}
