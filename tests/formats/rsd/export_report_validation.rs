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
//   - Export report validation test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Export report validation test module.
// - Description:
//   - Implements the declared test module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Export report validation test module.

use std::collections::BTreeMap;
use std::path::PathBuf;

use rsd::{ExportReport, RsdEncoding, RsdHeader, SourceRootReport};
use same_file as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn export_reports_reject_invalid_format_headers() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 0,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous_count = format_counts.insert(header, 1_usize);
    let report = ExportReport {
        source_roots: vec![SourceRootReport {
            root: PathBuf::from("source"),
            files: 1,
            source_bytes: 2,
            wav_bytes: 46,
        }],
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 46,
        format_counts,
    };

    assert!(
        report.validate().is_err(),
        "report validation must reject impossible public RSD headers"
    );
}

#[test]
fn wav_evidence_requires_minimum_riff_bytes() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous = format_counts.insert(header, 1_usize);
    let report = ExportReport {
        source_roots: vec![SourceRootReport {
            root: PathBuf::from("source"),
            files: 1,
            source_bytes: 2,
            wav_bytes: 1,
        }],
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 1,
        format_counts,
    };
    assert!(
        report.validate().is_err(),
        "aggregate evidence must reject undersized WAV output"
    );

    let mut root = SourceRootReport {
        root: PathBuf::from("source"),
        files: 0,
        source_bytes: 0,
        wav_bytes: 0,
    };
    let root_before = root.clone();
    let root_result = root.add_file(2_u64, 1_u64);
    assert!(
        root_result.is_err(),
        "source-root mutation must reject undersized WAV output"
    );
    assert_eq!(
        root, root_before,
        "invalid WAV evidence must not mutate source-root totals"
    );

    let mut aggregate = ExportReport::default();
    let aggregate_before = aggregate.clone();
    let aggregate_result = aggregate.add_file(header, 2_u64, 1_u64);
    assert!(
        aggregate_result.is_err(),
        "aggregate mutation must reject undersized WAV output"
    );
    assert_eq!(
        aggregate, aggregate_before,
        "invalid WAV evidence must not mutate aggregate totals"
    );
}

#[test]
fn wav_evidence_matches_channel_frame_size() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 2,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous = format_counts.insert(header, 1_usize);
    let report = ExportReport {
        source_roots: vec![SourceRootReport {
            root: PathBuf::from("source"),
            files: 1,
            source_bytes: 2,
            wav_bytes: 46,
        }],
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 46,
        format_counts,
    };
    assert!(
        report.validate().is_err(),
        "stereo WAV evidence must include one complete sample frame"
    );

    let mut aggregate = ExportReport::default();
    let before = aggregate.clone();
    let result = aggregate.add_file(header, 2_u64, 46_u64);
    assert!(
        result.is_err(),
        "aggregate mutation must reject undersized stereo WAV output"
    );
    assert_eq!(
        aggregate, before,
        "undersized stereo evidence must not mutate aggregate totals"
    );
}

#[test]
fn radical_adpcm_wav_evidence_requires_one_decoded_frame() {
    let header = RsdHeader {
        encoding: RsdEncoding::RadicalAdpcm,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous = format_counts.insert(header, 1_usize);
    let report = ExportReport {
        source_roots: vec![SourceRootReport {
            root: PathBuf::from("source"),
            files: 1,
            source_bytes: 2,
            wav_bytes: 46,
        }],
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 46,
        format_counts,
    };
    assert!(
        report.validate().is_err(),
        "RADP WAV evidence must contain one decoded codec frame"
    );

    let mut aggregate = ExportReport::default();
    let before = aggregate.clone();
    let result = aggregate.add_file(header, 2_u64, 46_u64);
    assert!(
        result.is_err(),
        "aggregate mutation must reject undersized RADP WAV output"
    );
    assert_eq!(
        aggregate, before,
        "undersized RADP evidence must not mutate aggregate totals"
    );
}

#[test]
fn radical_adpcm_source_evidence_requires_one_encoded_frame() {
    let header = RsdHeader {
        encoding: RsdEncoding::RadicalAdpcm,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous = format_counts.insert(header, 1_usize);
    let report = ExportReport {
        source_roots: vec![SourceRootReport {
            root: PathBuf::from("source"),
            files: 1,
            source_bytes: 2,
            wav_bytes: 108,
        }],
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 108,
        format_counts,
    };
    assert!(
        report.validate().is_err(),
        "RADP source evidence must contain one encoded codec frame"
    );

    let mut aggregate = ExportReport::default();
    let before = aggregate.clone();
    let result = aggregate.add_file(header, 2_u64, 108_u64);
    assert!(
        result.is_err(),
        "aggregate mutation must reject undersized RADP source input"
    );
    assert_eq!(
        aggregate, before,
        "undersized RADP source evidence must not mutate aggregate totals"
    );
}

#[test]
fn zero_byte_file_evidence_is_rejected() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut source_empty_root = SourceRootReport {
        root: PathBuf::from("source"),
        files: 0,
        source_bytes: 0,
        wav_bytes: 0,
    };
    let source_empty_root_result = source_empty_root.add_file(0_u64, 46_u64);
    assert!(
        source_empty_root_result.is_err(),
        "zero-byte RSD sources must fail per-root mutation"
    );

    let mut wav_empty_root = SourceRootReport {
        root: PathBuf::from("source"),
        files: 0,
        source_bytes: 0,
        wav_bytes: 0,
    };
    let wav_empty_root_result = wav_empty_root.add_file(2_u64, 0_u64);
    assert!(
        wav_empty_root_result.is_err(),
        "zero-byte WAV outputs must fail per-root mutation"
    );

    let mut source_empty_report = ExportReport::default();
    let source_empty_report_result =
        source_empty_report.add_file(header, 0_u64, 46_u64);
    assert!(
        source_empty_report_result.is_err(),
        "zero-byte RSD sources must fail aggregate mutation"
    );

    let mut wav_empty_report = ExportReport::default();
    let wav_empty_report_result =
        wav_empty_report.add_file(header, 2_u64, 0_u64);
    assert!(
        wav_empty_report_result.is_err(),
        "zero-byte WAV outputs must fail aggregate mutation"
    );
}

#[test]
fn root_file_counts_must_own_their_byte_evidence() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous = format_counts.insert(header, 1_usize);
    let report = ExportReport {
        source_roots: vec![
            SourceRootReport {
                root: PathBuf::from("empty"),
                files: 0,
                source_bytes: 2,
                wav_bytes: 46,
            },
            SourceRootReport {
                root: PathBuf::from("claimed"),
                files: 1,
                source_bytes: 0,
                wav_bytes: 0,
            },
        ],
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 46,
        format_counts,
    };

    assert!(
        report.validate().is_err(),
        "per-root file counts must own the bytes included in global totals"
    );
}

#[test]
fn duplicate_source_roots_are_rejected() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous = format_counts.insert(header, 2_usize);
    let duplicate = PathBuf::from("source");
    let report = ExportReport {
        source_roots: vec![
            SourceRootReport {
                root: duplicate.clone(),
                files: 1,
                source_bytes: 2,
                wav_bytes: 46,
            },
            SourceRootReport {
                root: duplicate,
                files: 1,
                source_bytes: 2,
                wav_bytes: 46,
            },
        ],
        total_files: 2,
        source_bytes: 4,
        wav_bytes: 92,
        format_counts,
    };

    assert!(
        report.validate().is_err(),
        "one source root must not contribute duplicate report entries"
    );
}

#[test]
fn source_roots_require_folder_identity() {
    let header = RsdHeader {
        encoding: RsdEncoding::PcmLittleEndian,
        channels: 1,
        bits_per_sample: 16,
        sample_rate: 24_000,
    };
    let mut format_counts = BTreeMap::new();
    let _previous = format_counts.insert(header, 1_usize);
    let report = ExportReport {
        source_roots: vec![SourceRootReport {
            root: PathBuf::new(),
            files: 1,
            source_bytes: 2,
            wav_bytes: 46,
        }],
        total_files: 1,
        source_bytes: 2,
        wav_bytes: 46,
        format_counts,
    };

    assert!(
        report.validate().is_err(),
        "report roots must retain the folder identity used for publication"
    );
}

#[test]
fn invalid_source_root_receivers_are_not_mutated() {
    let invalid_roots = [
        SourceRootReport {
            root: PathBuf::new(),
            files: 0,
            source_bytes: 0,
            wav_bytes: 0,
        },
        SourceRootReport {
            root: PathBuf::from("source"),
            files: 0,
            source_bytes: 2,
            wav_bytes: 46,
        },
        SourceRootReport {
            root: PathBuf::from("source"),
            files: 1,
            source_bytes: 0,
            wav_bytes: 46,
        },
        SourceRootReport {
            root: PathBuf::from("source"),
            files: 1,
            source_bytes: 2,
            wav_bytes: 0,
        },
    ];
    for mut report in invalid_roots {
        let before = report.clone();
        let result = report.add_file(2_u64, 46_u64);

        assert!(
            result.is_err(),
            "invalid source-root state must fail before mutation"
        );
        assert_eq!(
            report, before,
            "invalid source-root state must remain unchanged"
        );
    }
}
