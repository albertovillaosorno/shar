// File:
//   - operation_error.rs
// Path:
//   - src/cli/src/adapters/driven/standard_streams/operation_error.rs
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
//   - Standard-stream operation context around provider I/O failures.
// - Must-Not:
//   - Write bytes, flush writers, or choose process output streams.
// - Allows:
//   - Preserve provider kind and source while identifying write or flush.
// - Split-When:
//   - Another adapter requires an independent operation-error vocabulary.
// - Merge-When:
//   - Standard-stream failures no longer need operation provenance.
// - Summary:
//   - Standard-stream operation errors.
// - Description:
//   - Wraps provider errors without erasing their source chain or kind.
// - Usage:
//   - Used only by the standard-stream driven adapter.
// - Defaults:
//   - Provider controls are escaped after the operation prefix.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Contextual provider failures from standard-stream operations.
//!
//! The original I/O error remains available through the error source chain.

use std::io;

/// Standard-stream operation that failed.
#[derive(Debug, Clone, Copy)]
pub(super) enum StreamOperation {
    /// Writing the complete text bytes failed after partial acceptance.
    Write {
        /// Bytes accepted before the permanent failure.
        accepted_bytes: usize,
        /// Total bytes required for complete delivery.
        total_bytes: usize,
    },
    /// Flushing accepted bytes failed.
    Flush {
        /// Bytes accepted before the flush attempt.
        accepted_bytes: usize,
    },
}

/// Returns the stable byte-count unit for one displayed quantity.
const fn byte_unit(count: usize) -> &'static str {
    if count == 1 {
        "byte"
    } else {
        "bytes"
    }
}

/// Returns untrusted provider text without raw control characters.
fn escaped_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::escape_default)
        .collect()
}

impl core::fmt::Display for StreamOperation {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Write {
                accepted_bytes,
                total_bytes,
            } => {
                let unit = byte_unit(*total_bytes);
                write!(
                    formatter,
                    "write standard stream after {accepted_bytes} of "
                )?;
                write!(
                    formatter,
                    "{total_bytes} {unit}"
                )
            }
            Self::Flush {
                accepted_bytes,
            } => {
                let unit = byte_unit(*accepted_bytes);
                write!(
                    formatter,
                    "flush standard stream after accepting {accepted_bytes} "
                )?;
                formatter.write_str(unit)
            }
        }
    }
}

/// Invalid byte count returned by one standard-stream writer.
#[derive(Debug)]
struct InvalidWriteCount {
    /// Bytes reported as accepted by the writer.
    reported_bytes: usize,
    /// Bytes supplied to the writer for the operation.
    available_bytes: usize,
}

impl core::fmt::Display for InvalidWriteCount {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let reported_bytes = self.reported_bytes;
        let available_bytes = self.available_bytes;
        write!(
            formatter,
            "writer reported {reported_bytes} bytes for a "
        )?;
        write!(
            formatter,
            "{available_bytes}-byte buffer"
        )
    }
}

impl std::error::Error for InvalidWriteCount {}

/// Creates one invalid writer-count error with exact byte evidence.
pub(super) fn invalid_write_count_error(
    reported_bytes: usize,
    available_bytes: usize,
) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        InvalidWriteCount {
            reported_bytes,
            available_bytes,
        },
    )
}

/// Provider error paired with its failed standard-stream operation.
#[derive(Debug)]
struct StreamOperationError {
    /// Operation attempted by the adapter.
    operation: StreamOperation,
    /// Original provider I/O error.
    source: io::Error,
}

impl core::fmt::Display for StreamOperationError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let operation = self.operation;
        let source_text = self
            .source
            .to_string();
        let rendered_source = escaped_text(&source_text);
        write!(
            formatter,
            "failed to {operation}: {rendered_source}"
        )
    }
}

impl std::error::Error for StreamOperationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Wraps one provider error without changing its stable I/O category.
pub(super) fn contextualize(
    operation: StreamOperation,
    source: io::Error,
) -> io::Error {
    let kind = source.kind();
    io::Error::new(
        kind,
        StreamOperationError {
            operation,
            source,
        },
    )
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;
    use std::io;

    use super::{StreamOperation, contextualize};

    #[test]
    fn provider_error_escapes_source_control_characters() {
        let error = contextualize(
            StreamOperation::Flush {
                accepted_bytes: 4,
            },
            io::Error::other("flush\ninjected"),
        );

        let rendered = error.to_string();

        assert!(
            !rendered
                .chars()
                .any(char::is_control),
            "diagnostic contains a control character: {rendered:?}"
        );
        assert!(rendered.contains(r"flush\ninjected"));
        assert!(
            error
                .source()
                .is_some()
        );
    }

    #[test]
    fn one_byte_progress_uses_the_singular_unit() {
        let write = StreamOperation::Write {
            accepted_bytes: 0,
            total_bytes: 1,
        };
        let flush = StreamOperation::Flush {
            accepted_bytes: 1,
        };

        assert_eq!(
            write.to_string(),
            "write standard stream after 0 of 1 byte"
        );
        assert_eq!(
            flush.to_string(),
            "flush standard stream after accepting 1 byte"
        );
    }
}
