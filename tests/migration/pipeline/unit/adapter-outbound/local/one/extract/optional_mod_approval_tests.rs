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
//   - Optional package approval preflight tests.
// - Must-Not:
//   - Execute a complete extraction after approval succeeds.
// - Allows:
//   - Temporary local fixtures proving mutation order.
// - Split-When:
//   - Approval policy gains another independently tested boundary.
// - Merge-When:
//   - Another test module owns identical preflight evidence.
// - Summary:
//   - Optional package approval preflight tests.
// - Description:
//   - Proves unapproved packages fail before output cleanup or creation.
// - Usage:
//   - Included only by the extraction adapter under cfg(test).
// - Defaults:
//   - Missing approval fails closed when a supported package is present.
//

//! Optional package approval preflight tests.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::ExtractGameAssets;
use crate::domain::PipelineConfig;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn case_root(label: &str) -> PathBuf {
    let id = CASE_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "shar-optional-approval-{label}-{}-{id}",
        std::process::id()
    ))
}

#[test]
fn unapproved_packages_fail_before_output_mutation() -> Result<(), String> {
    let root = case_root("mutation-order");
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    let game_root = root.join("game");
    let mods_root = game_root.join("mods");
    let extracted_root = root.join("extracted");
    fs::create_dir_all(&mods_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;
    fs::write(mods_root.join("m.lmlm"), b"fixture")
        .map_err(|error| error.to_string())?;
    let sentinel = extracted_root.join("sentinel.txt");
    fs::write(&sentinel, b"preserved").map_err(|error| error.to_string())?;

    let clean_config = PipelineConfig {
        game_root: game_root.clone(),
        extracted_root,
        clean_extracted: true,
        approve_optional_mods: false,
    };
    let clean_error = match ExtractGameAssets::run(&clean_config) {
        Ok(_report) => {
            return Err("unapproved clean extraction succeeded".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !clean_error.contains("require explicit approval")
        || fs::read(&sentinel).map_err(|error| error.to_string())?
            != b"preserved"
    {
        return Err(
            "approval failure did not precede extraction cleanup".to_owned()
        );
    }

    let isolated_root = root.join("isolated");
    let isolated_config = PipelineConfig {
        game_root,
        extracted_root: isolated_root.clone(),
        clean_extracted: false,
        approve_optional_mods: false,
    };
    if ExtractGameAssets::export_lmlm_only(&isolated_config).is_ok()
        || isolated_root.exists()
    {
        return Err(
            "approval failure did not precede output-root creation".to_owned()
        );
    }

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(())
}
