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
//   - Public sound-script source identity regressions.
// - Must-Not:
//   - Own extraction orchestration or persistent fixtures.
// - Allows:
//   - Temporary input files and JSON assertions.
// - Split-When:
//   - Split when another sound-script schema gains independent ownership.
// - Merge-When:
//   - Merge when the SPT adapter owns the same evidence directly.
// - Summary:
//   - Public sound-script source identity regressions.
// - Description:
//   - Prevents physical SPT input paths from entering generated JSON.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Physical path leakage fails explicitly.
//

//! Public sound-script source identity regressions.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::to_json;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

#[test]
fn spt_json_uses_the_supplied_public_source_identity() -> Result<(), String> {
    let private_fragment = "private-workstation-spt";
    let root = std::env::temp_dir().join(format!(
        "{private_fragment}-{}-{}",
        std::process::id(),
        CASE_ID.fetch_add(1, Ordering::Relaxed),
    ));
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let input = root.join(PathBuf::from("sound").join("sample.spt"));
    let parent = input.parent().ok_or_else(|| "missing parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(&input, "create sound named sample\n{\nplay()\n}\n")
        .map_err(|error| error.to_string())?;
    let json = to_json(&input, "sound/sample.spt")
        .map_err(|error| error.to_string())?;
    if json.contains(private_fragment)
        || !json.contains("\"source\":\"sound/sample.spt\"")
    {
        return Err("SPT JSON did not preserve the public source".to_owned());
    }
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(())
}
