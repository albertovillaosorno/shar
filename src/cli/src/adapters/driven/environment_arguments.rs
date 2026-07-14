// File:
//   - environment_arguments.rs
// Path:
//   - src/cli/src/adapters/driven/environment_arguments.rs
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
//   - Operating-system process argument decoding.
// - Must-Not:
//   - Interpret argument meaning or execute domain commands.
// - Allows:
//   - Exclude the executable and reject non-Unicode arguments explicitly.
// - Split-When:
//   - Split when another process source needs an independent adapter.
// - Merge-When:
//   - Another adapter owns the same environment argument contract.
// - Summary:
//   - Environment argument-source adapter.
// - Description:
//   - Implements argument collection without Unicode panics.
// - Usage:
//   - Selected by the standard process driving composition.
// - Defaults:
//   - Returned indices are zero-based after the executable name.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driven adapter for operating-system process arguments.
//!
//! Invalid Unicode becomes a typed argument error instead of a process panic.
use crate::domain::ArgumentError;
use crate::ports::ArgumentSource;

/// Reads arguments from the current process environment.
#[derive(Debug, Default, Clone, Copy)]
pub struct EnvironmentArguments;

impl ArgumentSource for EnvironmentArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        std::env::args_os()
            .skip(1)
            .enumerate()
            .map(
                |(index, value)| match value.into_string() {
                    Ok(decoded) => Ok(decoded),
                    Err(invalid_value) => {
                        drop(invalid_value);
                        Err(ArgumentError::non_unicode(index))
                    }
                },
            )
            .collect()
    }
}
