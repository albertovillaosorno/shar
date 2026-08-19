//! Read-only structural validation of supported source container formats.

use std::path::Path;

use p3d::analyze_p3d;
use rcf::ArchiveParser;
use rcf::adapters::FileArchiveSource;
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

impl DeepSourceAudit {
    /// Validates every supported structured source file without writing output.
    ///
    /// # Errors
    /// Returns a format-class failure when a supported container is malformed
    /// or unreadable, and a generic failure when the source tree cannot be read.
    pub fn execute(source_root: &Path) -> Result<DeepSourceAuditReport, SourceAuditError> {
        let kind = local::path_kind(source_root).map_err(|_error| {
            SourceAuditError::new("deep source validation could not inspect source directory")
        })?;
        if kind != PathKind::Directory {
            return Err(SourceAuditError::new("source game directory not found"));
        }
        let files = local::strict_regular_files(source_root).map_err(|_error| {
            SourceAuditError::new("deep source validation could not scan source directory")
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
                _ => {}
            }
        }
        report.files = report
            .p3d
            .checked_add(report.rcf)
            .and_then(|value| value.checked_add(report.rsd))
            .and_then(|value| value.checked_add(report.rmv))
            .ok_or_else(|| SourceAuditError::new("deep source count overflow"))?;
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
    *value = value
        .checked_add(1)
        .ok_or_else(|| SourceAuditError::new(format!("{kind} count overflow")))?;
    Ok(())
}

fn validate_p3d(path: &Path, report: &mut DeepSourceAuditReport) -> Result<(), SourceAuditError> {
    let bytes = read_source(path, "p3d")?;
    let _document = analyze_p3d(&bytes)
        .map_err(|_error| SourceAuditError::new("deep source validation failed for p3d input"))?;
    increment(&mut report.p3d, "p3d")
}

fn validate_rcf(path: &Path, report: &mut DeepSourceAuditReport) -> Result<(), SourceAuditError> {
    let source = FileArchiveSource::new(path);
    let _archive = ArchiveParser::execute(&source)
        .map_err(|_error| SourceAuditError::new("deep source validation failed for rcf input"))?;
    increment(&mut report.rcf, "rcf")
}

fn validate_rsd(path: &Path, report: &mut DeepSourceAuditReport) -> Result<(), SourceAuditError> {
    let bytes = read_source(path, "rsd")?;
    let _audio = RsdAudio::parse(&bytes)
        .map_err(|_error| SourceAuditError::new("deep source validation failed for rsd input"))?;
    increment(&mut report.rsd, "rsd")
}

fn validate_rmv(path: &Path, report: &mut DeepSourceAuditReport) -> Result<(), SourceAuditError> {
    let bytes = read_source(path, "rmv")?;
    if MovieKind::from_bytes(&bytes) == MovieKind::Unknown {
        return Err(SourceAuditError::new(
            "deep source validation failed for rmv input",
        ));
    }
    let _provenance = ProvenanceEvidence::from_bytes(&bytes);
    increment(&mut report.rmv, "rmv")
}
