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
