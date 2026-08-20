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
//   - Read-only deep structural validation of supported source containers.
// - Must-Not:
//   - Write source inputs or disclose private source paths.
// - Allows:
//   - Traverse source strictly and invoke supported format auditors.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Deep source-audit application service.
// - Description:
//   - Read-only deep structural validation of supported source containers.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Read-only structural validation of supported source container formats.

use std::path::Path;

use p3d::analyze_p3d;
use rcf::ArchiveParser;
use rcf::domain::ArchiveError;
use rcf::ports::ArchiveByteReader;
use rmv::MovieKind;
use rmv::domain::ProvenanceEvidence;
use rsd::RsdAudio;
use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;

use crate::domain::SourceAuditError;

/// Aggregate deep-source validation evidence without private source paths.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DeepSourceAuditReport {
    /// Total supported containers inspected.
    pub files: usize,
    /// `Pure3D` package count.
    pub p3d: usize,
    /// RCF archive count.
    pub rcf: usize,
    /// RSD audio count.
    pub rsd: usize,
    /// RMV/Bink movie count.
    pub rmv: usize,
}

/// Stateless deep source validator.
#[derive(Debug, Clone, Copy)]
pub struct DeepSourceAudit;

/// Immutable byte snapshot exposed through the RCF parser's range port.
struct SnapshotReader<'a> {
    bytes: &'a [u8],
}

impl ArchiveByteReader for SnapshotReader<'_> {
    fn len(&self) -> Result<u64, ArchiveError> {
        u64::try_from(self.bytes.len()).map_err(|error| {
            ArchiveError::invalid_archive(format!(
                "RCF snapshot length does not fit u64: {error}"
            ))
        })
    }

    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ArchiveError> {
        let end = offset.checked_add(length).ok_or_else(|| {
            ArchiveError::invalid_archive("RCF snapshot range overflow")
        })?;
        let start = usize::try_from(offset).map_err(|error| {
            ArchiveError::invalid_archive(format!(
                "RCF snapshot offset does not fit usize: {error}"
            ))
        })?;
        let end = usize::try_from(end).map_err(|error| {
            ArchiveError::invalid_archive(format!(
                "RCF snapshot range end does not fit usize: {error}"
            ))
        })?;
        self.bytes
            .get(start..end)
            .map(<[u8]>::to_vec)
            .ok_or_else(|| {
                ArchiveError::invalid_archive(
                    "RCF snapshot range exceeds captured bytes",
                )
            })
    }
}

impl DeepSourceAudit {
    /// Validates every supported structured source file without writing output.
    ///
    /// # Errors
    /// Returns a format-class failure when a supported container is malformed
    /// or unreadable. A generic failure reports source-tree read errors.
    pub fn execute(
        source_root: &Path,
    ) -> Result<DeepSourceAuditReport, SourceAuditError> {
        let kind = local::path_kind(source_root).map_err(|_error| {
            SourceAuditError::new(
                "deep source validation could not inspect source directory",
            )
        })?;
        if kind != PathKind::Directory {
            let message = "source game directory not found";
            return Err(SourceAuditError::new(message));
        }
        let files =
            local::strict_regular_files(source_root).map_err(|_error| {
                SourceAuditError::new(
                    "deep source validation could not scan source directory",
                )
            })?;
        let mut report = DeepSourceAuditReport::default();
        for path in files {
            let extension = path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default()
                .to_ascii_lowercase();
            match extension.as_str() {
                "p3d" => validate_p3d(&path, &mut report)?,
                "rcf" => validate_rcf(&path, &mut report)?,
                "rsd" => validate_rsd(&path, &mut report)?,
                "rmv" => validate_rmv(&path, &mut report)?,
                _ => {},
            }
        }
        report.files = report
            .p3d
            .checked_add(report.rcf)
            .and_then(|value| value.checked_add(report.rsd))
            .and_then(|value| value.checked_add(report.rmv))
            .ok_or_else(|| {
                SourceAuditError::new("deep source count overflow")
            })?;
        Ok(report)
    }
}

fn read_source(path: &Path, kind: &str) -> Result<Vec<u8>, SourceAuditError> {
    local::read_bytes(path).map_err(|_error| {
        SourceAuditError::new(format!(
            "deep source validation could not read {kind} input"
        ))
    })
}

fn increment(value: &mut usize, kind: &str) -> Result<(), SourceAuditError> {
    *value = value.checked_add(1).ok_or_else(|| {
        SourceAuditError::new(format!("{kind} count overflow"))
    })?;
    Ok(())
}

fn validate_p3d(
    path: &Path,
    report: &mut DeepSourceAuditReport,
) -> Result<(), SourceAuditError> {
    let bytes = read_source(path, "p3d")?;
    let _document = analyze_p3d(&bytes).map_err(|_error| {
        SourceAuditError::new("deep source validation failed for p3d input")
    })?;
    increment(&mut report.p3d, "p3d")
}

fn validate_rcf(
    path: &Path,
    report: &mut DeepSourceAuditReport,
) -> Result<(), SourceAuditError> {
    let bytes = read_source(path, "rcf")?;
    let mut reader = SnapshotReader { bytes: &bytes };
    let _archive =
        ArchiveParser::from_reader(&mut reader).map_err(|_error| {
            SourceAuditError::new("deep source validation failed for rcf input")
        })?;
    increment(&mut report.rcf, "rcf")
}

fn validate_rsd(
    path: &Path,
    report: &mut DeepSourceAuditReport,
) -> Result<(), SourceAuditError> {
    let bytes = read_source(path, "rsd")?;
    let _audio = RsdAudio::parse(&bytes).map_err(|_error| {
        SourceAuditError::new("deep source validation failed for rsd input")
    })?;
    increment(&mut report.rsd, "rsd")
}

fn validate_rmv(
    path: &Path,
    report: &mut DeepSourceAuditReport,
) -> Result<(), SourceAuditError> {
    let bytes = read_source(path, "rmv")?;
    if MovieKind::from_bytes(&bytes) == MovieKind::Unknown {
        return Err(SourceAuditError::new(
            "deep source validation failed for rmv input",
        ));
    }
    let _provenance = ProvenanceEvidence::from_bytes(&bytes);
    increment(&mut report.rmv, "rmv")
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/migration/source-audit/unit/application/audit/tests.rs"]
mod tests;
