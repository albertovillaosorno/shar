// File:
//   - non_unicode_path.rs
// Path:
//   - src/filesystem/tests/non_unicode_path.rs
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
//   - Regression coverage for non-Unicode Windows path components.
// - Must-Not:
//   - Perform filesystem IO or depend on locale-specific rendering.
// - Allows:
//   - Construct an unpaired UTF-16 component and assert portable rejection.
// - Split-When:
//   - Split when another native encoding has independent fixture policy.
// - Merge-When:
//   - Another test target owns the same non-Unicode path contract.
// - Summary:
//   - Non-Unicode path regression coverage.
// - Description:
//   - Prevents native path encodings from bypassing portable checks.
// - Usage:
//   - Runs on Windows through the filesystem crate test target.
// - Defaults:
//   - Unpaired UTF-16 units are rejected before path resolution.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for non-Unicode Windows path components.
//!
//! Ill-formed native text must not bypass portable identity validation.
#[cfg(windows)]
mod windows {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;
    use std::path::{Path, PathBuf};

    use schoenwald_filesystem::{RootedPathError, resolve_under};

    #[test]
    fn non_unicode_component_is_rejected() -> Result<(), String> {
        let component = PathBuf::from(
            OsString::from_wide(
                &[
                    u16::from(b'b'),
                    0xd800_u16,
                    u16::from(b'x'),
                ],
            ),
        );
        let result = resolve_under(
            Path::new("output"),
            &component,
        );

        if result != Err(RootedPathError::NonUnicodeComponent) {
            return Err(
                format!("unexpected non-Unicode resolution: {result:?}"),
            );
        }
        Ok(())
    }
}
