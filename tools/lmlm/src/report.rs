//! Deterministic evidence for inspected legacy packages.

use serde::Serialize;

use crate::archive::{FileEntry, entry_bytes};

/// Stable report schema.
pub const REPORT_SCHEMA: &str = "shar.lmlm-conversion.v2";

/// Pure3D observation produced by SHAR's existing `p3d` crate.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct P3dEvidence {
    /// Whether the shared parser accepted the payload.
    pub valid: bool,
    /// Compression reported by the shared parser when valid.
    pub compression: Option<String>,
    /// Parsed chunk count when valid.
    pub chunks: Option<usize>,
    /// Parser diagnostic when invalid.
    pub diagnostic: Option<String>,
}

/// Evidence for one archive entry.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EntryEvidence {
    /// Archive-relative path.
    pub path: String,
    /// Exact payload size.
    pub size: u64,
    /// SHA-256 of the exact payload bytes.
    pub sha256: String,
    /// Shared Pure3D analysis for `.p3d` payloads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p3d: Option<P3dEvidence>,
}

/// Deterministic package report.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ConversionReport {
    /// Stable schema identifier.
    pub schema: &'static str,
    /// The converter only handles inspectable/decompilable input.
    pub decompilable_mods_only: bool,
    /// Current adaptation state.
    pub status: &'static str,
    /// Complete source archive size.
    pub source_size: usize,
    /// SHA-256 of the complete source archive.
    pub source_sha256: String,
    /// Per-entry evidence in archive order.
    pub entries: Vec<EntryEvidence>,
}

fn p3d_evidence(path: &str, payload: &[u8]) -> Option<P3dEvidence> {
    let is_p3d = path
        .rsplit('.')
        .next()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("p3d"));
    if !is_p3d {
        return None;
    }
    Some(match p3d::analyze_p3d(payload) {
        Ok(document) => P3dEvidence {
            valid: true,
            compression: Some(document.compression.to_owned()),
            chunks: Some(document.chunks.len()),
            diagnostic: None,
        },
        Err(error) => P3dEvidence {
            valid: false,
            compression: None,
            chunks: None,
            diagnostic: Some(error.to_string()),
        },
    })
}

/// Builds deterministic evidence from validated archive bytes.
#[must_use]
pub fn build_report(data: &[u8], entries: &[FileEntry]) -> ConversionReport {
    let entries = entries
        .iter()
        .filter_map(|entry| {
            entry_bytes(data, entry).map(|payload| EntryEvidence {
                path: entry.path.clone(),
                size: entry.size,
                sha256: shar_sha256::digest_hex(payload),
                p3d: p3d_evidence(&entry.path, payload),
            })
        })
        .collect();
    ConversionReport {
        schema: REPORT_SCHEMA,
        decompilable_mods_only: true,
        status: "extracted-needs-shar-package-adaptation",
        source_size: data.len(),
        source_sha256: shar_sha256::digest_hex(data),
        entries,
    }
}
