// File:
//   - error.rs
// Path:
//   - src/rsd/src/domain/error.rs
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
//   - Pure rsd domain rules for domain error.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - A new transport layer requires adapter-specific context that cannot
//   - remain represented as a pure parsing, decoding, or export failure.
// - Merge-When:
//   - Another rsd module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Error variants for RSD parsing, decoding, and export.
// - Description:
//   - Defines the closed failure vocabulary shared by container validation,
//   - sample decoding, safe path handling, and RIFF serialization.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: The closed RSD failure taxonomy, formatter helpers, and error
//   - source chaining form one exhaustive domain contract.
//

//! Error variants for RSD parsing, decoding, and export.
//!
//! A closed taxonomy keeps corrupt input, unsupported codecs, and output-size
//! failures explicit without leaking adapter implementation details.
use std::path::PathBuf;

use super::EscapedPath;

/// Closed error taxonomy keeps parser, decoder, and filesystem failures
/// explicit.
#[derive(Debug)]
pub enum RsdError {
    /// Export request omitted every source root.
    NoInputRoots,
    /// Export request discovered no RSD audio inputs.
    NoAudioInputs,
    /// Filesystem operation failed with path context for diagnostics.
    Io {
        /// Path tied to the failed IO operation.
        path: PathBuf,
        /// Original IO failure retained for error chaining.
        source: std::io::Error,
    },
    /// Source audio conversion failed with the owning file path preserved.
    SourceAudio {
        /// RSD file whose parsing, decoding, or serialization failed.
        path: PathBuf,
        /// Typed conversion failure retained for error chaining.
        source: Box<Self>,
    },
    /// Header marker fails the container signature check.
    BadMagic,
    /// Header bytes are incomplete, so format fields cannot be trusted.
    TruncatedHeader,
    /// Payload ended before a declared audio frame could be decoded.
    TruncatedData,
    /// Reserved bytes in a padded RSD header violate the writer contract.
    InvalidHeaderPadding,
    /// Input root cannot become a safe output folder name.
    InvalidRootName(PathBuf),
    /// Input root exists but is not a directory tree.
    InvalidSourceRoot(PathBuf),
    /// Existing output root is not a directory tree.
    InvalidOutputRoot(PathBuf),
    /// Two source roots would map into the same output folder identity.
    CollidingRootName {
        /// Earlier root claiming the shared output folder name.
        first: PathBuf,
        /// Later root that conflicts with the earlier output identity.
        second: PathBuf,
    },
    /// More than one converted artifact resolves to the same destination.
    CollidingOutputPath(PathBuf),
    /// Source and output trees overlap and cannot remain isolated.
    OverlappingOutputRoot {
        /// Source root whose traversal intersects the output tree.
        source: PathBuf,
        /// Output root that intersects the source tree.
        output: PathBuf,
    },
    /// Output-relative path failed normalization.
    InvalidPath(PathBuf),
    /// ADPCM step table index left the supported codec range.
    InvalidStepIndex(i32),
    /// Decoded predictor output could not fit the target PCM sample width.
    InvalidSample(i32),
    /// Codec tag is outside the encodings this exporter understands.
    UnsupportedEncoding([u8; 4]),
    /// Bit depth cannot be emitted as target PCM WAV.
    UnsupportedBitDepth(u32),
    /// Sample rate cannot represent a playable PCM stream.
    UnsupportedSampleRate(u32),
    /// Channel count is outside safe decoder allocation bounds.
    UnsupportedChannels(u32),
    /// A parser or serializer buffer could not be reserved safely.
    AllocationFailed(usize),
    /// Export evidence violates its aggregate consistency contract.
    InvalidReport(&'static str),
    /// Export evidence cannot represent another checked aggregate update.
    ReportOverflow(&'static str),
    /// Output would exceed RIFF WAV length fields.
    WavTooLarge(usize),
    /// Payload length is not aligned to the codec frame size.
    UnalignedPayload {
        /// Encoding name identifies which frame contract failed.
        encoding: &'static str,
        /// Payload size that failed frame alignment.
        bytes: usize,
        /// Required frame size for the active encoding and channel count.
        frame_size: usize,
    },
}

/// Writes one path-scoped failure without erasing its typed source.
fn write_path_error(
    formatter: &mut core::fmt::Formatter<'_>,
    path: &std::path::Path,
    source: &dyn core::fmt::Display,
) -> core::fmt::Result {
    write!(
        formatter,
        "{}: {source}",
        EscapedPath::new(path)
    )
}

/// Writes one labeled path without dropping non-Unicode components.
fn write_labeled_path(
    formatter: &mut core::fmt::Formatter<'_>,
    label: &str,
    path: &std::path::Path,
) -> core::fmt::Result {
    write!(
        formatter,
        "{label}: {}",
        EscapedPath::new(path)
    )
}

/// Writes two paths that participate in one filesystem conflict.
fn write_path_pair(
    formatter: &mut core::fmt::Formatter<'_>,
    label: &str,
    first: &std::path::Path,
    second: &std::path::Path,
) -> core::fmt::Result {
    write!(
        formatter,
        "{label}: {} and {}",
        EscapedPath::new(first),
        EscapedPath::new(second)
    )
}

/// Renders an untrusted four-byte codec tag without terminal controls.
fn write_encoding_tag(
    formatter: &mut core::fmt::Formatter<'_>,
    tag: [u8; 4],
) -> core::fmt::Result {
    write!(
        formatter,
        "unsupported RSD encoding: "
    )?;
    for byte in tag {
        write!(
            formatter,
            "\\x{byte:02X}"
        )?;
    }
    Ok(())
}

/// Writes one RIFF frame-alignment failure.
fn write_unaligned_payload(
    formatter: &mut core::fmt::Formatter<'_>,
    encoding: &str,
    bytes: usize,
    frame_size: usize,
) -> core::fmt::Result {
    write!(
        formatter,
        "{encoding} payload has {bytes} bytes, not a multiple of          \
         frame size {frame_size}"
    )
}

/// Constructs one fixed-text rendering case without repeated match blocks.
const fn static_display(message: &'static str) -> ErrorDisplay<'static> {
    ErrorDisplay::Static(message)
}

/// Typed rendering case derived exhaustively from one RSD failure.
#[derive(Clone, Copy)]
enum ErrorDisplay<'a> {
    /// Fixed diagnostic text.
    Static(&'static str),
    /// Path plus retained typed source.
    PathError {
        /// Path tied to the source failure.
        path: &'a std::path::Path,
        /// Typed source rendered after the path.
        source: &'a dyn core::fmt::Display,
    },
    /// Labeled path diagnostic.
    LabeledPath {
        /// Stable diagnostic label.
        label: &'static str,
        /// Untrusted path rendered with control escaping.
        path: &'a std::path::Path,
    },
    /// Two paths participating in one conflict.
    PathPair {
        /// Stable conflict label.
        label: &'static str,
        /// First conflicting path.
        first: &'a std::path::Path,
        /// Second conflicting path.
        second: &'a std::path::Path,
    },
    /// Untrusted four-byte codec tag.
    Encoding([u8; 4]),
    /// Labeled signed scalar.
    SignedValue {
        /// Stable diagnostic label.
        label: &'static str,
        /// Signed diagnostic value.
        value: i32,
    },
    /// Labeled unsigned scalar.
    UnsignedValue {
        /// Stable diagnostic label.
        label: &'static str,
        /// Unsigned diagnostic value.
        value: u32,
    },
    /// Failed byte-buffer reservation.
    AllocationFailed(usize),
    /// Oversized RIFF payload.
    WavTooLarge(usize),
    /// Misaligned codec payload.
    UnalignedPayload {
        /// Codec family name.
        encoding: &'static str,
        /// Actual payload byte count.
        bytes: usize,
        /// Required codec frame size.
        frame_size: usize,
    },
}

impl RsdError {
    /// Classify one failure into its complete rendering contract.
    fn diagnostic_case(&self) -> ErrorDisplay<'_> {
        match self {
            Self::NoInputRoots => {
                static_display("no RSD source roots provided")
            }
            Self::NoAudioInputs => static_display("no RSD audio inputs found"),
            Self::Io {
                path,
                source,
            } => ErrorDisplay::PathError {
                path,
                source,
            },
            Self::SourceAudio {
                path,
                source,
            } => ErrorDisplay::PathError {
                path,
                source: source.as_ref(),
            },
            Self::BadMagic => static_display("not an RSD4 audio file"),
            Self::TruncatedHeader => static_display("RSD header is truncated"),
            Self::TruncatedData => static_display("RSD payload is truncated"),
            Self::InvalidHeaderPadding => {
                static_display("RSD padded header metadata is corrupt")
            }
            Self::InvalidRootName(path) => ErrorDisplay::LabeledPath {
                label: "input root has no safe folder name",
                path,
            },
            Self::InvalidSourceRoot(path) => ErrorDisplay::LabeledPath {
                label: "input root is not a directory",
                path,
            },
            Self::InvalidOutputRoot(path) => ErrorDisplay::LabeledPath {
                label: "output root is not a directory",
                path,
            },
            Self::CollidingRootName {
                first,
                second,
            } => ErrorDisplay::PathPair {
                label: "source roots share one output folder name",
                first,
                second,
            },
            Self::CollidingOutputPath(path) => ErrorDisplay::LabeledPath {
                label: "multiple RSD sources target one output path",
                path,
            },
            Self::OverlappingOutputRoot {
                source,
                output,
            } => ErrorDisplay::PathPair {
                label: "source and output trees overlap",
                first: source,
                second: output,
            },
            Self::InvalidPath(path) => ErrorDisplay::LabeledPath {
                label: "path is not safe for export",
                path,
            },
            Self::InvalidStepIndex(value) => ErrorDisplay::SignedValue {
                label: "invalid step index",
                value: *value,
            },
            Self::InvalidSample(value) => ErrorDisplay::SignedValue {
                label: "invalid PCM sample",
                value: *value,
            },
            Self::UnsupportedEncoding(tag) => ErrorDisplay::Encoding(*tag),
            Self::UnsupportedBitDepth(value) => ErrorDisplay::UnsignedValue {
                label: "unsupported bit depth",
                value: *value,
            },
            Self::UnsupportedSampleRate(value) => ErrorDisplay::UnsignedValue {
                label: "unsupported sample rate",
                value: *value,
            },
            Self::UnsupportedChannels(value) => ErrorDisplay::UnsignedValue {
                label: "unsupported channel count",
                value: *value,
            },
            Self::AllocationFailed(bytes) => {
                ErrorDisplay::AllocationFailed(*bytes)
            }
            Self::InvalidReport(message) | Self::ReportOverflow(message) => {
                static_display(message)
            }
            Self::WavTooLarge(bytes) => ErrorDisplay::WavTooLarge(*bytes),
            Self::UnalignedPayload {
                encoding,
                bytes,
                frame_size,
            } => ErrorDisplay::UnalignedPayload {
                encoding,
                bytes: *bytes,
                frame_size: *frame_size,
            },
        }
    }
}

impl core::fmt::Display for RsdError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self.diagnostic_case() {
            ErrorDisplay::Static(message) => formatter.write_str(message),
            ErrorDisplay::PathError {
                path,
                source,
            } => write_path_error(
                formatter, path, source,
            ),
            ErrorDisplay::LabeledPath {
                label,
                path,
            } => write_labeled_path(
                formatter, label, path,
            ),
            ErrorDisplay::PathPair {
                label,
                first,
                second,
            } => write_path_pair(
                formatter, label, first, second,
            ),
            ErrorDisplay::Encoding(tag) => write_encoding_tag(
                formatter, tag,
            ),
            ErrorDisplay::SignedValue {
                label,
                value,
            } => write!(
                formatter,
                "{label}: {value}"
            ),
            ErrorDisplay::UnsignedValue {
                label,
                value,
            } => write!(
                formatter,
                "{label}: {value}"
            ),
            ErrorDisplay::AllocationFailed(bytes) => write!(
                formatter,
                "unable to reserve {bytes} bytes for RSD conversion"
            ),
            ErrorDisplay::WavTooLarge(bytes) => write!(
                formatter,
                "WAV payload is too large: {bytes} bytes"
            ),
            ErrorDisplay::UnalignedPayload {
                encoding,
                bytes,
                frame_size,
            } => write_unaligned_payload(
                formatter, encoding, bytes, frame_size,
            ),
        }
    }
}
impl std::error::Error for RsdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io {
                source,
                ..
            } => Some(source),
            Self::SourceAudio {
                source,
                ..
            } => Some(source.as_ref()),
            _ => None,
        }
    }
}
