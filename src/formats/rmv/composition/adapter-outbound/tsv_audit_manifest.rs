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
//   - Tsv audit manifest outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tsv audit manifest outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tsv audit manifest outbound adapter.

use std::fmt::Write as _;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::{AuditReport, MovieKind, RmvError};
use crate::ports::AuditManifestSink;

/// Filesystem-backed canonical TSV audit manifest sink.
#[derive(Debug, Default, Clone, Copy)]
pub struct TsvAuditManifestSink;

impl AuditManifestSink for TsvAuditManifestSink {
    fn write_manifest(
        &self,
        output_root: &Path,
        report: &AuditReport,
    ) -> Result<(), RmvError> {
        write_manifest(output_root, report).map_err(|source| {
            RmvError::io(output_root.join("manifest.tsv"), source)
        })
    }
}

/// Write audit manifest.
fn write_manifest(
    output_root: &Path,
    report: &AuditReport,
) -> Result<(), std::io::Error> {
    let header_columns = [
        "sha256",
        "bytes",
        "kind",
        "current_source",
        "pre_bink_master_evidence",
        "expected_bk2",
    ];
    let mut manifest = header_columns.join("\t");
    manifest.push('\n');
    let mut rows = Vec::with_capacity(report.records.len());
    for record in &report.records {
        let kind = match record.kind {
            MovieKind::BinkV1 => "bink-v1",
            MovieKind::BinkV2 => "bink-v2",
            MovieKind::OggNamedRmv => "ogg-named-rmv",
            MovieKind::XboxXmvLike => "xbox-xmv-like",
            MovieKind::RadicalMovieHeader => "radical-movie-header",
            MovieKind::Unknown => "unknown",
        };
        let source_path = escape_manifest_path(&record.source_path);
        let provenance = escape_manifest_field(&record.provenance.summary());
        let output_path = escape_manifest_path(&record.output_path);
        let mut row = String::new();
        let _write_result = writeln!(
            row,
            "{}\t{}\t{}\t{}\t{}\t{}",
            record.hash.hex(),
            record.bytes,
            kind,
            source_path,
            provenance,
            output_path,
        );
        rows.push((source_path, output_path, row));
    }
    rows.sort();
    for row in rows {
        manifest.push_str(&row.2);
    }
    local::write_text(&output_root.join("manifest.tsv"), &manifest, true)
}

/// Escapes a filesystem path without losing Windows UTF-16 code units.
#[cfg(windows)]
fn escape_manifest_path(path: &Path) -> String {
    use std::os::windows::ffi::OsStrExt as _;

    let mut escaped = String::new();
    for decoded in char::decode_utf16(path.as_os_str().encode_wide()) {
        match decoded {
            Ok(character) => append_manifest_character(&mut escaped, character),
            Err(error) => {
                escaped.push_str(r"\u");
                let _write_result =
                    write!(escaped, "{:04X}", error.unpaired_surrogate());
            },
        }
    }
    escaped
}

/// Escapes a path through Unicode text on platforms with byte-safe callers.
#[cfg(not(windows))]
fn escape_manifest_path(path: &Path) -> String {
    escape_manifest_field(&path.to_string_lossy())
}

/// Escapes control characters that would otherwise corrupt TSV structure.
fn escape_manifest_field(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        append_manifest_character(&mut escaped, character);
    }
    escaped
}

/// Appends one manifest character with TSV-safe escaping.
fn append_manifest_character(escaped: &mut String, character: char) {
    match character {
        '\\' => escaped.push_str("\\\\"),
        '\t' => escaped.push_str("\\t"),
        '\n' => escaped.push_str("\\n"),
        '\r' => escaped.push_str("\\r"),
        control if control.is_control() => {
            escaped.push('\\');
            let _write_result = write!(escaped, "u{:04X}", u32::from(control));
        },
        other => escaped.push(other),
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/formats/rmv/unit/adapter-outbound/tsv_audit_manifest/tests.rs"]
mod tests;
