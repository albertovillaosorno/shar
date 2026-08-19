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
//   - Path-free source-audit failure value.
// - Must-Not:
//   - Depend on filesystem, process, or serialization capabilities.
// - Allows:
//   - Carry stable public-safe audit failure text.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Source-audit domain error.
// - Description:
//   - Path-free source-audit failure value.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Public-safe source-audit failure.

use core::fmt;

/// Error class for source auditing without private path evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceAuditError {
    message: String,
}

impl SourceAuditError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for SourceAuditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SourceAuditError {}
