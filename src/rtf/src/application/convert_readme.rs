// File:
//   - convert_readme.rs
// Path:
//   - src/rtf/src/application/convert_readme.rs
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
//   - README conversion policy and source-provenance notice composition.
// - Must-Not:
//   - Read files, write outputs, or parse command-line arguments directly.
// - Allows:
//   - Load through a source port and compose one complete Markdown document.
// - Split-When:
//   - Split when conversion and notice policy become independent use cases.
// - Merge-When:
//   - Another use case owns the same complete README conversion contract.
// - Summary:
//   - Application command for RTF README conversion.
// - Description:
//   - Produces a complete Markdown document with explicit provenance notices.
// - Usage:
//   - Invoked by driving adapters after selecting a source provider.
// - Defaults:
//   - Missing timestamp evidence produces a conservative historical notice.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for complete RTF README conversion.
//!
//! The use case owns generated-document policy while source access remains
//! replaceable behind a port.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::DiagnosticPath;

use crate::domain::{format_unix_date, rtf_to_markdown};
use crate::ports::RtfSource;

/// Returns untrusted diagnostic text without raw control characters.
fn escaped_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::escape_default)
        .collect()
}

/// Affiliation and provenance disclaimer prepended to generated documents.
const DISCLAIMER: &str =
    "\
> **Disclaimer.** This document is an automatically generated Markdown \
     conversion of the original\n> game's README. It is not affiliated with, \
     sponsored by, or endorsed by Disney, 20th Century Fox,\n> Radical \
     Entertainment, or any related rights holder. The conversion was produced \
     by original,\n> from-scratch code in this repository (the `rtf` crate); \
     no third-party libraries were used. The\n> underlying text remains the \
     property of its respective owners.\n";

/// Failure while loading an RTF source document.
#[derive(Debug)]
pub struct ConvertReadmeError {
    /// Input path whose source snapshot could not be loaded.
    path: PathBuf,
    /// Underlying source adapter failure.
    source: io::Error,
}

impl core::fmt::Display for ConvertReadmeError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let source_text = self
            .source
            .to_string();
        write!(
            formatter,
            "failed to read {}: {}",
            DiagnosticPath::new(&self.path),
            escaped_text(&source_text)
        )
    }
}

impl std::error::Error for ConvertReadmeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Stateless README conversion use case.
#[derive(Debug, Clone, Copy)]
pub struct ConvertReadme;

impl ConvertReadme {
    /// Loads and converts one RTF README into complete Markdown.
    ///
    /// # Errors
    ///
    /// Returns a contextual source-loading failure.
    pub fn execute(
        source: &impl RtfSource,
        input: &Path,
    ) -> Result<String, ConvertReadmeError> {
        let snapshot = source
            .load(input)
            .map_err(
                |read_error| ConvertReadmeError {
                    path: input.to_path_buf(),
                    source: read_error,
                },
            )?;
        let source_date = snapshot
            .modified_unix_seconds
            .map(format_unix_date);
        let mut document = header(source_date.as_deref());
        document.push_str(&rtf_to_markdown(&snapshot.bytes));
        Ok(document)
    }
}

/// Builds the generated-document notice from weak timestamp evidence.
fn header(date: Option<&str>) -> String {
    let mut header = String::from(DISCLAIMER);
    if let Some(source_date_text) = date {
        header.push_str(
            "> **Source date.** The source file's last-modified metadata \
             reads ",
        );
        header.push_str(source_date_text);
        header.push_str(
            ". This is only an\n> approximate indicator of the document's age \
             (around 2003) and cannot be asserted with\n> certainty. The \
             content is historical and must not be treated as current, \
             accurate, or\n> valid today.\n",
        );
    } else {
        header.push_str(
            "> **Source date.** This document is historical (approximately \
             2003) and must not be treated\n> as current, accurate, or valid \
             today.\n",
        );
    }
    header.push_str("\n---\n\n");
    header
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;
    #[cfg(windows)]
    use std::ffi::OsString;
    use std::io;
    #[cfg(windows)]
    use std::os::windows::ffi::OsStringExt as _;
    use std::path::Path;
    #[cfg(windows)]
    use std::path::PathBuf;

    use super::ConvertReadme;
    use crate::ports::{RtfSnapshot, RtfSource};

    struct ControlFailingSource;

    impl RtfSource for ControlFailingSource {
        fn load(
            &self,
            _path: &Path,
        ) -> io::Result<RtfSnapshot> {
            Err(io::Error::other("read\nfailure"))
        }
    }

    struct FailingSource;

    impl RtfSource for FailingSource {
        fn load(
            &self,
            _path: &Path,
        ) -> io::Result<RtfSnapshot> {
            Err(io::Error::other("read failure"))
        }
    }

    #[test]
    fn read_error_escapes_source_control_characters() {
        let result = ConvertReadme::execute(
            &ControlFailingSource,
            Path::new("readme.rtf"),
        );
        assert!(
            result.is_err(),
            "failing source unexpectedly converted"
        );
        let Err(error) = result else {
            return;
        };
        let rendered = error.to_string();

        assert!(
            !rendered
                .chars()
                .any(char::is_control),
            "diagnostic contains a control character: {rendered:?}"
        );
        assert!(rendered.contains(r"read\nfailure"));
        assert!(
            error
                .source()
                .is_some()
        );
    }

    #[cfg(windows)]
    #[test]
    fn read_error_preserves_unpaired_utf16_path_unit() {
        let path = PathBuf::from(
            OsString::from_wide(
                &[
                    u16::from(b'a'),
                    0xd800,
                    u16::from(b'b'),
                ],
            ),
        );

        let result = ConvertReadme::execute(
            &FailingSource,
            &path,
        );
        assert!(
            result.is_err(),
            "failing source unexpectedly converted"
        );
        let Err(error) = result else {
            return;
        };
        let rendered = error.to_string();

        assert!(
            rendered.contains(r"a\u{D800}b"),
            "diagnostic lost the native path unit: {rendered:?}"
        );
        assert!(!rendered.contains('\u{fffd}'));
    }
}
