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
        matches!(result, Err(RsdError::InvalidReport(_))),
        "application must reject invalid exporter evidence"
    );
}
