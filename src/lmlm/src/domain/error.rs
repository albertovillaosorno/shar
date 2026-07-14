// File:
//   - error.rs
// Path:
//   - src/lmlm/src/domain/error.rs
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
//   - Typed public failures for LMLM parsing and structural validation.
// - Must-Not:
//   - Parse archive bytes, traverse records, or write extracted files.
// - Allows:
//   - Stable diagnostic data and human-readable formatting.
// - Split-When:
//   - One error family becomes independently versioned or externally mapped.
// - Merge-When:
//   - Another LMLM error module duplicates the same public failure contract.
// - Summary:
//   - Defines fail-closed parser diagnostics.
// - Description:
//   - Preserves malformed-input evidence without panics or silent coercion.
// - Usage:
//   - Returned by the public parser and consumed by adapters and tests.
// - Defaults:
//   - Errors retain paths, offsets, sizes, and unexpected values when known.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: The public error union and deterministic Display implementation
//   - form one exhaustive versioned API contract and must change together.
//

//! Typed failures for the LMLM parser boundary.
//!
//! Each variant preserves deterministic evidence for one malformed container,
//! record, path, payload, or package-identity condition.

use crate::diagnostic::EscapedText;

/// Errors that can occur while reading an `.lmlm` archive.
#[derive(Debug)]
pub enum LmlmError {
    /// The archive does not start with the `LSPA` magic.
    BadMagic {
        /// Four bytes observed at the archive start.
        observed: [u8; 4],
    },
    /// The archive ended earlier than the structure required.
    Truncated,
    /// The LSPA container version is not supported by this parser.
    UnsupportedVersion {
        /// Archive-relative version-field offset.
        offset: usize,
        /// Unsupported version value.
        value: u32,
    },
    /// The LSPA container flags do not describe the supported layout.
    UnsupportedHeaderFlags {
        /// Archive-relative flags-field offset.
        offset: usize,
        /// Unsupported flags value.
        value: u32,
    },
    /// A reserved header byte is nonzero.
    NonZeroReservedHeader {
        /// Archive-relative byte offset.
        offset: usize,
        /// Unexpected byte value.
        value: u8,
    },
    /// The reserved container block contains data.
    NonZeroReservedContainerBlock {
        /// Archive-relative byte offset.
        offset: usize,
        /// Unexpected byte value.
        value: u8,
    },
    /// The root block contains data outside its count field.
    NonZeroReservedRootBlock {
        /// Archive-relative byte offset.
        offset: usize,
        /// Unexpected byte value.
        value: u8,
    },
    /// The archive contains nonzero bytes outside declared payloads.
    NonZeroUnclaimedData {
        /// Archive-relative byte offset.
        offset: usize,
        /// Unexpected byte value.
        value: u8,
    },
    /// The archive contains nonzero bytes after its final payload.
    NonZeroTrailingData {
        /// Archive-relative byte offset.
        offset: usize,
        /// Unexpected byte value.
        value: u8,
    },
    /// Directory nesting exceeds the supported parser depth.
    ExcessiveDirectoryDepth {
        /// Archive-relative directory path.
        path: String,
        /// Requested directory depth.
        depth: usize,
    },
    /// An entry name block declares an unsupported kind word.
    InvalidEntryKind {
        /// Archive-relative entry-block offset.
        offset: usize,
        /// Unexpected kind word.
        value: u16,
    },
    /// An entry name block has no UTF-16 NUL terminator.
    UnterminatedName {
        /// Archive-relative entry-block offset.
        offset: usize,
    },
    /// An entry name block contains data after its terminator.
    NonZeroNamePadding {
        /// Archive-relative byte offset.
        offset: usize,
        /// Unexpected byte value.
        value: u8,
    },
    /// An entry metadata block contains data in its reserved tail.
    NonZeroMetadataPadding {
        /// Archive-relative byte offset.
        offset: usize,
        /// Unexpected byte value.
        value: u8,
    },
    /// A directory record declares an unsupported child-kind control.
    UnsupportedDirectoryRecordControl {
        /// Archive-relative control-byte offset.
        offset: usize,
        /// Unexpected control value.
        value: u8,
    },
    /// A directory child-kind control disagrees with its immediate children.
    DirectoryRecordControlMismatch {
        /// Archive-relative directory path.
        path: String,
        /// Archive-relative control-byte offset.
        offset: usize,
        /// Declared control value.
        declared: u8,
        /// Value derived from the immediate child kinds.
        expected: u8,
    },
    /// A file record declares an unsupported transition-control value.
    UnsupportedFileRecordControl {
        /// Archive-relative control-byte offset.
        offset: usize,
        /// Unexpected control value.
        value: u8,
    },
    /// A file record's transition block contains data after its control byte.
    NonZeroFileRecordPadding {
        /// Archive-relative byte offset.
        offset: usize,
        /// Unexpected byte value.
        value: u8,
    },
    /// An entry name is not valid UTF-16.
    InvalidNameEncoding {
        /// Archive-relative entry-block offset.
        offset: usize,
        /// Decoder failure retained for diagnosis.
        message: String,
    },
    /// An entry payload begins inside the parsed entry table.
    EntryPayloadOverlapsTable {
        /// Archive-relative entry path.
        path: String,
        /// Declared payload offset.
        offset: u64,
        /// First byte after all parsed name and metadata blocks.
        table_end: usize,
    },
    /// An entry payload offset is not aligned to a structural block.
    UnalignedEntryOffset {
        /// Archive-relative entry path.
        path: String,
        /// Declared payload offset.
        offset: u64,
    },
    /// An entry payload range lies outside the archive.
    InvalidEntryRange {
        /// Archive-relative entry path.
        path: String,
        /// Declared payload offset.
        offset: u64,
        /// Declared payload size.
        size: u64,
    },
    /// Two entries claim at least one common payload byte.
    OverlappingEntryRanges {
        /// First path in archive-offset order.
        first_path: String,
        /// First declared payload offset.
        first_offset: u64,
        /// First declared payload size.
        first_size: u64,
        /// Later path whose payload begins before the first payload ends.
        second_path: String,
        /// Later declared payload offset.
        second_offset: u64,
        /// Later declared payload size.
        second_size: u64,
    },
    /// Two archive paths target the same portable output identity.
    PathCollision {
        /// First archive path that claimed the normalized identity.
        first_path: String,
        /// Later archive path that collides with the first path.
        second_path: String,
    },
    /// An entry name contained a path component that escapes the archive root.
    UnsafePath(String),
    /// The archive is not the supported Jebano Latino package.
    UnsupportedPackage,
}

impl LmlmError {
    /// Formats errors owned by the fixed container and package boundary.
    fn fmt_container(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> Option<core::fmt::Result> {
        let result = match self {
            Self::BadMagic {
                observed,
            } => {
                let observed_value = u32::from_be_bytes(*observed);
                write!(
                    formatter,
                    "not an LSPA (.lmlm) archive; observed magic: \
                     {observed_value:#010x}"
                )
            }
            Self::Truncated => write!(
                formatter,
                "archive is truncated or malformed"
            ),
            Self::UnsupportedVersion {
                offset,
                value,
            } => write!(
                formatter,
                "unsupported LSPA archive version at {offset:#x}: {value}"
            ),
            Self::UnsupportedHeaderFlags {
                offset,
                value,
            } => write!(
                formatter,
                "unsupported LSPA archive header flags at {offset:#x}: \
                 {value:#010x}"
            ),
            Self::NonZeroReservedHeader {
                offset,
                value,
            } => write!(
                formatter,
                "reserved LSPA header byte at {offset:#x} is nonzero: \
                 {value:#04x}"
            ),
            Self::NonZeroReservedContainerBlock {
                offset,
                value,
            } => {
                write!(
                    formatter,
                    "reserved LSPA container byte at {offset:#x} is nonzero: \
                     {value:#04x}"
                )
            }
            Self::NonZeroReservedRootBlock {
                offset,
                value,
            } => write!(
                formatter,
                "reserved LSPA root byte at {offset:#x} is nonzero: \
                 {value:#04x}"
            ),
            Self::NonZeroUnclaimedData {
                offset,
                value,
            } => write!(
                formatter,
                "unclaimed LSPA byte at {offset:#x} is nonzero: {value:#04x}"
            ),
            Self::NonZeroTrailingData {
                offset,
                value,
            } => write!(
                formatter,
                "trailing LSPA byte at {offset:#x} is nonzero: {value:#04x}"
            ),
            Self::UnsupportedPackage => write!(
                formatter,
                "archive is not the supported Jebano Latino package"
            ),
            _ => return None,
        };
        Some(result)
    }

    /// Formats structural errors from one entry record or name block.
    fn fmt_entry_structure(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> Option<core::fmt::Result> {
        let result = match self {
            Self::ExcessiveDirectoryDepth {
                path,
                depth,
            } => {
                write!(
                    formatter,
                    "archive directory nesting is too deep: {} at depth \
                     {depth}",
                    EscapedText::new(path)
                )
            }
            Self::InvalidEntryKind {
                offset,
                value,
            } => write!(
                formatter,
                "unsupported LSPA entry kind at {offset:#x}: {value}"
            ),
            Self::UnterminatedName {
                offset,
            } => write!(
                formatter,
                "LSPA entry name at {offset:#x} has no UTF-16 terminator"
            ),
            Self::NonZeroNamePadding {
                offset,
                value,
            } => write!(
                formatter,
                "LSPA entry name padding at {offset:#x} is nonzero: \
                 {value:#04x}"
            ),
            Self::NonZeroMetadataPadding {
                offset,
                value,
            } => {
                write!(
                    formatter,
                    "LSPA entry metadata padding at {offset:#x} is nonzero: \
                     {value:#04x}"
                )
            }
            Self::UnsupportedDirectoryRecordControl {
                offset,
                value,
            } => write!(
                formatter,
                "unsupported LSPA directory child-kind control at \
                 {offset:#x}: {value:#04x}"
            ),
            Self::DirectoryRecordControlMismatch {
                path,
                offset,
                declared,
                expected,
            } => write!(
                formatter,
                "LSPA directory child-kind control mismatch for {} at \
                 {offset:#x}: declared {declared}, expected {expected}",
                EscapedText::new(path)
            ),
            Self::UnsupportedFileRecordControl {
                offset,
                value,
            } => {
                write!(
                    formatter,
                    "unsupported LSPA file transition control at {offset:#x}: \
                     {value:#04x}"
                )
            }
            Self::NonZeroFileRecordPadding {
                offset,
                value,
            } => {
                write!(
                    formatter,
                    "LSPA file transition padding at {offset:#x} is nonzero: \
                     {value:#04x}"
                )
            }
            Self::InvalidNameEncoding {
                offset,
                message,
            } => {
                write!(
                    formatter,
                    "archive entry name at {offset:#x} is not valid UTF-16: {}",
                    EscapedText::new(message)
                )
            }
            _ => return None,
        };
        Some(result)
    }

    /// Formats errors for validated payload ranges and aliases.
    fn fmt_payload(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> Option<core::fmt::Result> {
        let result = match self {
            Self::EntryPayloadOverlapsTable {
                path,
                offset,
                table_end,
            } => {
                write!(
                    formatter,
                    "archive entry payload overlaps the table: {} at \
                     {offset}, table ends at {table_end}",
                    EscapedText::new(path)
                )
            }
            Self::UnalignedEntryOffset {
                path,
                offset,
            } => {
                write!(
                    formatter,
                    "archive entry payload is not block aligned: {} at \
                     {offset}",
                    EscapedText::new(path)
                )
            }
            Self::InvalidEntryRange {
                path,
                offset,
                size,
            } => {
                write!(
                    formatter,
                    "archive entry payload is out of bounds: {} at {offset} \
                     for {size} bytes",
                    EscapedText::new(path)
                )
            }
            Self::OverlappingEntryRanges {
                first_path,
                first_offset,
                first_size,
                second_path,
                second_offset,
                second_size,
            } => {
                write!(
                    formatter,
                    "archive entry payloads overlap: {} at {first_offset} for \
                     {first_size} bytes and {} at {second_offset} for \
                     {second_size} bytes",
                    EscapedText::new(first_path),
                    EscapedText::new(second_path)
                )
            }
            _ => return None,
        };
        Some(result)
    }

    /// Formats portable destination identity and containment errors.
    fn fmt_path(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> Option<core::fmt::Result> {
        let result = match self {
            Self::PathCollision {
                first_path,
                second_path,
            } => {
                write!(
                    formatter,
                    "archive paths collide on a portable filesystem: {} and {}",
                    EscapedText::new(first_path),
                    EscapedText::new(second_path)
                )
            }
            Self::UnsafePath(path) => {
                write!(
                    formatter,
                    "unsafe path in archive: {}",
                    EscapedText::new(path)
                )
            }
            _ => return None,
        };
        Some(result)
    }
}

impl core::fmt::Display for LmlmError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        if let Some(result) = self.fmt_container(formatter) {
            return result;
        }
        if let Some(result) = self.fmt_entry_structure(formatter) {
            return result;
        }
        if let Some(result) = self.fmt_payload(formatter) {
            return result;
        }
        if let Some(result) = self.fmt_path(formatter) {
            return result;
        }
        formatter.write_str("unclassified LMLM error")
    }
}

impl std::error::Error for LmlmError {}

#[cfg(test)]
mod tests {
    use super::LmlmError;

    #[test]
    fn public_diagnostics_are_single_line() {
        let errors = [
            LmlmError::NonZeroReservedContainerBlock {
                offset: 0x200,
                value: 1,
            },
            LmlmError::NonZeroMetadataPadding {
                offset: 0x800,
                value: 2,
            },
            LmlmError::EntryPayloadOverlapsTable {
                path: "entry.bin".to_owned(),
                offset: 0x600,
                table_end: 0xa00,
            },
            LmlmError::InvalidEntryRange {
                path: "entry.bin".to_owned(),
                offset: 0x1000,
                size: 7,
            },
            LmlmError::PathCollision {
                first_path: "Entry.bin".to_owned(),
                second_path: "entry.bin".to_owned(),
            },
        ];

        for error in errors {
            let diagnostic = error.to_string();
            assert!(
                !diagnostic.contains(
                    [
                        '\n', '\r'
                    ]
                ),
                "public diagnostic must remain single-line: {diagnostic:?}"
            );
        }
    }

    #[test]
    fn public_diagnostics_escape_untrusted_control_characters() {
        let errors = [
            LmlmError::UnsafePath("bad\u{1b}[2J.bin".to_owned()),
            LmlmError::InvalidNameEncoding {
                offset: 0x600,
                message: "bad\nencoding".to_owned(),
            },
            LmlmError::PathCollision {
                first_path: "first\rpath".to_owned(),
                second_path: "second\u{7}path".to_owned(),
            },
        ];

        for error in errors {
            let diagnostic = error.to_string();
            assert!(
                diagnostic
                    .chars()
                    .all(|character| !character.is_control()),
                "public diagnostic exposed a control character: {diagnostic:?}"
            );
        }
    }
}
