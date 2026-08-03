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

use super::ExportReport;
use crate::domain::{RsdEncoding, RsdHeader};

#[test]
fn invalid_header_does_not_mutate_report() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 0,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut report = ExportReport::default();
    let before = report.clone();

    let result = report.add_file(header, 2_u64, 46_u64);

    assert!(
        result.is_err(),
        "invalid format evidence must fail at the mutation boundary"
    );
    assert_eq!(
        report, before,
        "invalid format evidence must not partially mutate the report"
    );
}

#[test]
fn invalid_aggregate_receivers_are_not_mutated() {
    let valid_header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let invalid_header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 0,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut valid_counts = BTreeMap::new();
    let _previous_valid_count = valid_counts.insert(valid_header, 1_usize);
    let mut zero_counts = BTreeMap::new();
    let _previous_zero_count = zero_counts.insert(valid_header, 0_usize);
    let mut invalid_counts = BTreeMap::new();
    let _previous_invalid_count =
        invalid_counts.insert(invalid_header, 1_usize);
    let invalid_reports = [
        ExportReport {
            source_bytes: 2,
            ..ExportReport::default()
        },
        ExportReport {
            wav_bytes: 46,
            ..ExportReport::default()
        },
        ExportReport {
            total_files: 1,
            wav_bytes: 46,
            format_counts: valid_counts.clone(),
            ..ExportReport::default()
        },
        ExportReport {
            total_files: 1,
            source_bytes: 2,
            format_counts: valid_counts.clone(),
            ..ExportReport::default()
        },
        ExportReport {
            total_files: 1,
            source_bytes: 2,
            wav_bytes: 46,
            ..ExportReport::default()
        },
        ExportReport {
            total_files: 1,
            source_bytes: 2,
            wav_bytes: 46,
            format_counts: zero_counts,
            ..ExportReport::default()
        },
        ExportReport {
            total_files: 1,
            source_bytes: 2,
            wav_bytes: 46,
            format_counts: invalid_counts,
            ..ExportReport::default()
        },
    ];
    for mut report in invalid_reports {
        let before = report.clone();
        let result = report.add_file(valid_header, 2_u64, 46_u64);

        assert!(
            result.is_err(),
            "invalid aggregate state must fail before mutation"
        );
        assert_eq!(
            report, before,
            "invalid aggregate state must remain unchanged"
        );
    }
}

#[test]
fn aggregate_overflow_does_not_partially_mutate_report() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut report = ExportReport {
        total_files: usize::MAX,
        ..ExportReport::default()
    };
    let before = report.clone();

    let result = report.add_file(header, 2_u64, 46_u64);

    assert!(
        result.is_err(),
        "overflowing aggregate evidence must return a typed failure"
    );
    assert_eq!(
        report, before,
        "overflowing one aggregate field must not mutate other evidence"
    );
}
