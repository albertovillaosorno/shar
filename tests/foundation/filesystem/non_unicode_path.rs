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
//   - Non unicode path test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Non unicode path test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Non unicode path test module.

use schoenwald_filesystem as _;

#[cfg(windows)]
mod windows {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;
    use std::path::{Path, PathBuf};

    use schoenwald_filesystem::{RootedPathError, resolve_under};

    #[test]
    fn non_unicode_component_is_rejected() -> Result<(), String> {
        let component = PathBuf::from(OsString::from_wide(&[
            u16::from(b'b'),
            0xd800_u16,
            u16::from(b'x'),
        ]));
        let result = resolve_under(Path::new("output"), &component);

        if result != Err(RootedPathError::NonUnicodeComponent) {
            return Err(format!(
                "unexpected non-Unicode resolution: {result:?}"
            ));
        }
        Ok(())
    }
}
