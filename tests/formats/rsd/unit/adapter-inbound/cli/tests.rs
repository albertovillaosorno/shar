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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::collections::BTreeMap;
use std::path::PathBuf;

use schoenwald_cli::CliProgram;

use super::{RsdExportProgram, USAGE, report_outcome};
use crate::domain::{ExportReport, RsdEncoding, RsdHeader, SourceRootReport};

#[test]
fn inconsistent_reports_fail_instead_of_printing_success() {
    let report = ExportReport {
        source_roots: Vec::new(),
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 46,
        format_counts: BTreeMap::new(),
    };

    let outcome = report_outcome(&report);

    assert_eq!(
        outcome.status(),
        schoenwald_cli::ExitStatus::Failure,
        "inconsistent export evidence must not produce a success status"
    );
}

#[test]
fn successful_reports_escape_control_characters_in_paths() -> Result<(), String>
{
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous_count = format_counts.insert(header, 1_usize);
    let report = ExportReport {
        source_roots: vec![SourceRootReport {
            root: PathBuf::from("\u{1b}[2Jbad\nroot"),
            files: 1,
            source_bytes: 2,
            wav_bytes: 46,
        }],
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 46,
        format_counts,
    };

    let outcome = report_outcome(&report);
    let Some(first) = outcome.output().first() else {
        return Err("successful RSD report emitted no root line".to_owned());
    };
    let Some(line) = first.text().strip_suffix('\n') else {
        return Err(
            "successful RSD report omitted its line terminator".to_owned()
        );
    };
    if line.contains('\u{1b}') || line.contains('\n') {
        return Err(
            "successful RSD report emitted raw control characters".to_owned()
        );
    }
    if !line.contains(r"\u{1b}[2Jbad\nroot") {
        return Err(format!(
            "successful RSD report lost escaped path evidence: {:?}",
            first.text()
        ));
    }
    Ok(())
}

#[test]
fn missing_roots_return_one_usage_diagnostic() -> Result<(), String> {
    for arguments in [Vec::new(), vec!["output".to_owned()]] {
        let outcome = RsdExportProgram.execute(&arguments);
        if !outcome.is_failure_with_stderr_line(USAGE) {
            return Err(format!(
                "invalid RSD usage outcome for arguments: \
                     {arguments:?}"
            ));
        }
    }
    Ok(())
}
