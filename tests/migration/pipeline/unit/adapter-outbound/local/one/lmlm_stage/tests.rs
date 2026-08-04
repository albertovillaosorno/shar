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
//   - Optional LMLM stage unit tests.
// - Must-Not:
//   - Own production behavior or require an installed language mod.
// - Allows:
//   - Isolated filesystem fixtures for optional-package behavior.
// - Split-When:
//   - Split when present-package extraction needs independent fixtures.
// - Merge-When:
//   - Merge when another module owns the identical evidence.
// - Summary:
//   - Optional LMLM stage unit tests.
// - Description:
//   - Proves an English-only game creates no synthetic LMLM output.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Missing optional packages succeed with zero output.
//

//! Optional LMLM stage unit tests.

use std::fs;

use super::extract_lmlm;

#[test]
fn missing_optional_lmlm_creates_no_output() -> Result<(), String> {
    let case = std::env::temp_dir()
        .join(format!("pipeline-lmlm-optional-{}", std::process::id()));
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    let stale_output = extracted_root.join("lmlm");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&stale_output).map_err(|error| error.to_string())?;
    fs::write(stale_output.join("manifest.json"), b"stale")
        .map_err(|error| error.to_string())?;

    let report = extract_lmlm(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;

    if report.name != "lmlm" || report.files != 0 || report.bytes != 0 {
        return Err(format!(
            "unexpected optional-stage report: name={} files={} bytes={}",
            report.name, report.files, report.bytes
        ));
    }
    if report.note != "optional LMLM package not present; no output written" {
        return Err(format!("unexpected optional-stage note: {}", report.note));
    }
    if stale_output.exists() {
        return Err(String::from(
            "missing optional LMLM package left synthetic output behind",
        ));
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}
