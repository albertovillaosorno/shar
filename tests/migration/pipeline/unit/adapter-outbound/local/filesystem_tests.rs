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
//   - Filesystem tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem tests test module.
// - Description:
//   - Implements the declared test module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem tests test module.

use std::fs;

use super::collect_files;

#[cfg(windows)]
#[test]
fn non_unicode_root_error_is_reversible() -> Result<(), String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;

    let root = std::path::PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800_u16,
        u16::from(b'b'),
    ]));
    let result = collect_files(&root);
    let Some(error) = result.err() else {
        return Err("non-Unicode root unexpectedly succeeded".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains(r"a\u{D800}b") {
        return Err(format!("diagnostic lost native root: {rendered:?}"));
    }
    if rendered.contains('\u{fffd}') {
        return Err(format!("diagnostic used replacement text: {rendered:?}"));
    }
    Ok(())
}

#[test]
fn collect_files_returns_paths_in_canonical_order() -> Result<(), String> {
    let root = std::env::temp_dir()
        .join(format!("pipeline-filesystem-order-{}", std::process::id(),));
    match fs::remove_dir_all(&root) {
        Ok(()) => {},
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
        Err(error) => return Err(error.to_string()),
    }
    for directory in ["zeta", "alpha", "middle"] {
        let path = root.join(directory).join("file.bin");
        fs::create_dir_all(
            path.parent()
                .ok_or_else(|| String::from("missing parent"))?,
        )
        .map_err(|error| error.to_string())?;
        fs::write(&path, directory).map_err(|error| error.to_string())?;
    }

    let actual = collect_files(&root).map_err(|error| error.to_string())?;
    let mut expected = actual.clone();
    expected.sort();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;

    if actual != expected {
        return Err(format!(
            "filesystem traversal was not canonical: {actual:?}"
        ));
    }
    Ok(())
}
