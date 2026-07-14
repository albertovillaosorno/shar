// File:
//   - export.rs
// Path:
//   - src/rsd/src/application/export.rs
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
//   - The batch RSD export application command.
// - Must-Not:
//   - Traverse filesystems, decode CLI arguments, or write artifacts directly.
// - Allows:
//   - Invoke an exporter port and return its complete report.
// - Split-When:
//   - Split when planning and execution become independent use cases.
// - Merge-When:
//   - Another application module owns the complete export command.
// - Summary:
//   - Application use case for RSD root export.
// - Description:
//   - Delegates one explicit batch request through an exporter port.
// - Usage:
//   - Invoked by driving adapters after selecting a driven implementation.
// - Defaults:
//   - No adapter or path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for exporting RSD roots through an outbound port.
use std::path::{Path, PathBuf};

use crate::domain::ExportReport;
use crate::ports::Exporter;

/// Stateless batch-export use case.
#[derive(Debug, Clone, Copy)]
pub struct ExportRoots;

impl ExportRoots {
    /// Executes one explicit batch export.
    ///
    /// # Errors
    ///
    /// Returns the selected exporter failure or invalid report evidence.
    pub fn execute<E: Exporter>(
        exporter: &E,
        roots: &[PathBuf],
        output_root: &Path,
    ) -> Result<ExportReport, E::Error> {
        let report = exporter.export_roots(
            roots,
            output_root,
        )?;
        report
            .validate()
            .map_err(E::Error::from)?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};

    use super::ExportRoots;
    use crate::domain::{ExportReport, RsdError};
    use crate::ports::Exporter;

    struct InvalidExporter;

    impl Exporter for InvalidExporter {
        type Error = RsdError;

        fn export_roots(
            &self,
            _roots: &[PathBuf],
            _output_root: &Path,
        ) -> Result<ExportReport, Self::Error> {
            let report = ExportReport {
                source_roots: Vec::new(),
                total_files: 1,
                source_bytes: 2,
                wav_bytes: 46,
                format_counts: BTreeMap::new(),
            };
            Ok(report)
        }
    }

    #[test]
    fn invalid_port_report_fails_at_application_boundary() {
        let root = PathBuf::from("source");
        let result = ExportRoots::execute(
            &InvalidExporter,
            std::slice::from_ref(&root),
            Path::new("output"),
        );

        assert!(
            matches!(
                result,
                Err(RsdError::InvalidReport(_))
            ),
            "application must reject invalid exporter evidence"
        );
    }
}
