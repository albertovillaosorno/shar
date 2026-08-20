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
//   - Environment arguments outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Environment arguments outbound adapter.
// - Description:
//   - Implements the declared responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Environment arguments outbound adapter.

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
            .map(|(index, value)| match value.into_string() {
                Ok(decoded) => Ok(decoded),
                Err(invalid_value) => {
                    drop(invalid_value);
                    Err(ArgumentError::non_unicode(index))
                },
            })
            .collect()
    }
}
