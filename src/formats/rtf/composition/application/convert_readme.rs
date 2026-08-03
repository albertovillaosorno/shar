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
//   - Convert readme application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Convert readme application service.
// - Description:
//   - Implements the declared application service responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Convert readme application service.

use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::DiagnosticPath;

use crate::domain::{format_unix_date, rtf_to_markdown};
use crate::ports::RtfSource;

/// Returns untrusted diagnostic text without raw control characters.
fn escaped_text(value: &str) -> String {
    value.chars().flat_map(char::escape_default).collect()
}

/// Affiliation and provenance disclaimer prepended to generated documents.
const DISCLAIMER: &str = "\
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
        let source_text = self.source.to_string();
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
        let snapshot =
            source
                .load(input)
                .map_err(|read_error| ConvertReadmeError {
                    path: input.to_path_buf(),
                    source: read_error,
                })?;
        let source_date = snapshot.modified_unix_seconds.map(format_unix_date);
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
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/formats/rtf/unit/application/convert_readme/tests.rs"]
mod tests;
