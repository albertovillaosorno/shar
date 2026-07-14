// File:
//   - export_report.rs
// Path:
//   - src/rsd/src/domain/export_report.rs
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
//   - Pure batch-export evidence and aggregation rules.
// - Must-Not:
//   - Discover files, decode command arguments, or write outputs.
// - Allows:
//   - Root reports, aggregate counts, and saturating evidence updates.
// - Split-When:
//   - Split when per-root and aggregate reports evolve independently.
// - Merge-When:
//   - Another domain module owns the same export evidence aggregate.
// - Summary:
//   - RSD export report domain model.
// - Description:
//   - Captures deterministic conversion evidence without storage policy.
// - Usage:
//   - Returned by exporter ports and application commands.
// - Defaults:
//   - Empty reports contain zero counts and no roots.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Per-root and aggregate evidence mutation, validation, and
//   - overflow safety form one transactional report contract.
//

//! Pure report model for batch RSD export evidence.
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use super::{RsdEncoding, RsdError, RsdHeader, WavAudio};

/// Rejects impossible converted-file byte evidence before aggregate mutation.
const fn validate_file_evidence(
    source_bytes: u64,
    wav_bytes: u64,
) -> Result<(), RsdError> {
    if source_bytes == 0_u64 {
        return Err(
            RsdError::InvalidReport("RSD source file contains no bytes"),
        );
    }
    if wav_bytes < WavAudio::MINIMUM_FILE_BYTES {
        return Err(
            RsdError::InvalidReport(
                "RSD WAV output is smaller than a valid RIFF file",
            ),
        );
    }
    Ok(())
}

/// Conversion evidence for one source root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRootReport {
    /// Source root mirrored into output.
    pub root: PathBuf,
    /// Number of converted files.
    pub files: usize,
    /// Total source bytes.
    pub source_bytes: u64,
    /// Total generated WAV bytes.
    pub wav_bytes: u64,
}

/// Verifies one source-root report before aggregation or mutation.
fn validate_source_root_state(
    report: &SourceRootReport
) -> Result<(), RsdError> {
    if report
        .root
        .file_name()
        .is_none()
    {
        return Err(
            RsdError::InvalidReport("RSD source root has no folder identity"),
        );
    }
    if report.files == 0_usize {
        if report.source_bytes != 0_u64 || report.wav_bytes != 0_u64 {
            return Err(
                RsdError::InvalidReport(
                    "empty RSD source root contains byte evidence",
                ),
            );
        }
    } else {
        validate_file_evidence(
            report.source_bytes,
            report.wav_bytes,
        )?;
    }
    Ok(())
}

impl SourceRootReport {
    /// Adds one converted file without allowing partial evidence on overflow.
    ///
    /// # Errors
    ///
    /// Returns [`RsdError`] when byte evidence is empty or a per-root counter
    /// cannot represent the complete update.
    pub fn add_file(
        &mut self,
        source_bytes: u64,
        wav_bytes: u64,
    ) -> Result<(), RsdError> {
        validate_source_root_state(self)?;
        validate_file_evidence(
            source_bytes,
            wav_bytes,
        )?;
        let files = self
            .files
            .checked_add(1)
            .ok_or(
                RsdError::ReportOverflow("RSD source-root file count overflow"),
            )?;
        let total_source_bytes = self
            .source_bytes
            .checked_add(source_bytes)
            .ok_or(
                RsdError::ReportOverflow("RSD source-root byte count overflow"),
            )?;
        let total_wav_bytes = self
            .wav_bytes
            .checked_add(wav_bytes)
            .ok_or(
                RsdError::ReportOverflow(
                    "RSD source-root WAV byte count overflow",
                ),
            )?;

        self.files = files;
        self.source_bytes = total_source_bytes;
        self.wav_bytes = total_wav_bytes;
        Ok(())
    }
}

/// Aggregate conversion evidence for a batch.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExportReport {
    /// Per-root reports.
    pub source_roots: Vec<SourceRootReport>,
    /// Total converted files.
    pub total_files: usize,
    /// Total source bytes.
    pub source_bytes: u64,
    /// Total WAV bytes.
    pub wav_bytes: u64,
    /// Counts by validated RSD header.
    pub format_counts: BTreeMap<RsdHeader, usize>,
}

impl ExportReport {
    /// Verifies aggregate counters remain coherent during incremental assembly.
    fn validate_aggregate_state(&self) -> Result<(), RsdError> {
        if self.total_files == 0_usize {
            if self.source_bytes != 0_u64
                || self.wav_bytes != 0_u64
                || !self
                    .format_counts
                    .is_empty()
            {
                return Err(
                    RsdError::InvalidReport(
                        "empty RSD export report contains aggregate evidence",
                    ),
                );
            }
            return Ok(());
        }
        validate_file_evidence(
            self.source_bytes,
            self.wav_bytes,
        )?;
        let mut format_files = 0_usize;
        let mut minimum_radp_source_bytes = 0_u64;
        let mut minimum_wav_bytes = 0_u64;
        for (header, count) in &self.format_counts {
            header.validate()?;
            if *count == 0_usize {
                return Err(
                    RsdError::InvalidReport(
                        "RSD export report contains a zero format count",
                    ),
                );
            }
            format_files = format_files
                .checked_add(*count)
                .ok_or(
                    RsdError::ReportOverflow("RSD format count total overflow"),
                )?;
            let format_count_u64 = u64::try_from(*count).map_err(
                |_conversion_error| {
                    RsdError::ReportOverflow(
                        "RSD format count exceeds byte-evidence capacity",
                    )
                },
            )?;
            if header.encoding == RsdEncoding::RadicalAdpcm {
                let format_source_bytes = header
                    .minimum_source_file_bytes()?
                    .checked_mul(format_count_u64)
                    .ok_or(
                        RsdError::ReportOverflow(
                            "RADP minimum source byte evidence overflow",
                        ),
                    )?;
                minimum_radp_source_bytes = minimum_radp_source_bytes
                    .checked_add(format_source_bytes)
                    .ok_or(
                        RsdError::ReportOverflow(
                            "RADP minimum source byte total overflow",
                        ),
                    )?;
            }
            let format_wav_bytes = header
                .minimum_wav_file_bytes()?
                .checked_mul(format_count_u64)
                .ok_or(
                    RsdError::ReportOverflow(
                        "RSD minimum WAV byte evidence overflow",
                    ),
                )?;
            minimum_wav_bytes = minimum_wav_bytes
                .checked_add(format_wav_bytes)
                .ok_or(
                    RsdError::ReportOverflow(
                        "RSD minimum WAV byte total overflow",
                    ),
                )?;
        }
        if format_files != self.total_files {
            return Err(
                RsdError::InvalidReport(
                    "RSD format counts do not match total files",
                ),
            );
        }
        if self.source_bytes < minimum_radp_source_bytes {
            return Err(
                RsdError::InvalidReport(
                    "RADP source bytes are below the encoded-frame minimum",
                ),
            );
        }
        if self.wav_bytes < minimum_wav_bytes {
            return Err(
                RsdError::InvalidReport(
                    "RSD WAV bytes are below the format-specific minimum",
                ),
            );
        }
        Ok(())
    }

    /// Verifies root, aggregate, and format counts describe the same batch.
    ///
    /// # Errors
    ///
    /// Returns [`RsdError`] when totals overflow during verification or the
    /// report contains contradictory evidence.
    pub fn validate(&self) -> Result<(), RsdError> {
        self.validate_aggregate_state()?;
        if self.total_files == 0_usize {
            return Err(
                RsdError::InvalidReport("RSD export report contains no files"),
            );
        }
        let mut root_files = 0_usize;
        let mut root_source_bytes = 0_u64;
        let mut root_wav_bytes = 0_u64;
        let mut roots = BTreeSet::new();
        for root in &self.source_roots {
            validate_source_root_state(root)?;
            if !roots.insert(&root.root) {
                return Err(
                    RsdError::InvalidReport(
                        "RSD export report contains a duplicate source root",
                    ),
                );
            }
            root_files = root_files
                .checked_add(root.files)
                .ok_or(
                    RsdError::ReportOverflow(
                        "RSD source-root file total overflow",
                    ),
                )?;
            root_source_bytes = root_source_bytes
                .checked_add(root.source_bytes)
                .ok_or(
                    RsdError::ReportOverflow(
                        "RSD source-root byte total overflow",
                    ),
                )?;
            root_wav_bytes = root_wav_bytes
                .checked_add(root.wav_bytes)
                .ok_or(
                    RsdError::ReportOverflow(
                        "RSD source-root WAV byte total overflow",
                    ),
                )?;
        }
        if root_files != self.total_files
            || root_source_bytes != self.source_bytes
            || root_wav_bytes != self.wav_bytes
        {
            return Err(
                RsdError::InvalidReport(
                    "RSD export totals do not match source-root evidence",
                ),
            );
        }
        Ok(())
    }

    /// Adds one validated conversion to aggregate evidence.
    ///
    /// # Errors
    ///
    /// Returns [`RsdError`] when format or byte evidence is invalid or an
    /// aggregate counter cannot represent the complete update.
    pub fn add_file(
        &mut self,
        header: RsdHeader,
        source_bytes: u64,
        wav_bytes: u64,
    ) -> Result<(), RsdError> {
        self.validate_aggregate_state()?;
        header.validate()?;
        validate_file_evidence(
            source_bytes,
            wav_bytes,
        )?;
        if header.encoding == RsdEncoding::RadicalAdpcm
            && source_bytes < header.minimum_source_file_bytes()?
        {
            return Err(
                RsdError::InvalidReport(
                    "RADP source is below the encoded-frame minimum",
                ),
            );
        }
        if wav_bytes < header.minimum_wav_file_bytes()? {
            return Err(
                RsdError::InvalidReport(
                    "RSD WAV output is below the format-specific minimum",
                ),
            );
        }
        let total_files = self
            .total_files
            .checked_add(1)
            .ok_or(
                RsdError::ReportOverflow(
                    "RSD export total file count overflow",
                ),
            )?;
        let total_source_bytes = self
            .source_bytes
            .checked_add(source_bytes)
            .ok_or(
                RsdError::ReportOverflow(
                    "RSD export source byte count overflow",
                ),
            )?;
        let total_wav_bytes = self
            .wav_bytes
            .checked_add(wav_bytes)
            .ok_or(
                RsdError::ReportOverflow("RSD export WAV byte count overflow"),
            )?;
        let format_count = self
            .format_counts
            .get(&header)
            .copied()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or(
                RsdError::ReportOverflow("RSD export format count overflow"),
            )?;

        self.total_files = total_files;
        self.source_bytes = total_source_bytes;
        self.wav_bytes = total_wav_bytes;
        let _previous_format_count = self
            .format_counts
            .insert(
                header,
                format_count,
            );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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

        let result = report.add_file(
            header, 2_u64, 46_u64,
        );

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
        let _previous_valid_count = valid_counts.insert(
            valid_header,
            1_usize,
        );
        let mut zero_counts = BTreeMap::new();
        let _previous_zero_count = zero_counts.insert(
            valid_header,
            0_usize,
        );
        let mut invalid_counts = BTreeMap::new();
        let _previous_invalid_count = invalid_counts.insert(
            invalid_header,
            1_usize,
        );
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
            let result = report.add_file(
                valid_header,
                2_u64,
                46_u64,
            );

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

        let result = report.add_file(
            header, 2_u64, 46_u64,
        );

        assert!(
            result.is_err(),
            "overflowing aggregate evidence must return a typed failure"
        );
        assert_eq!(
            report, before,
            "overflowing one aggregate field must not mutate other evidence"
        );
    }
}
