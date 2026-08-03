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

use super::*;

#[test]
fn rejects_unsupported_major_file_info_version() -> Result<(), String> {
    let mut bytes = vec![0_u8; FILE_INFO_VERSION_MINOR_OFFSET + 1];
    let Some(major) = bytes.get_mut(FILE_INFO_VERSION_MAJOR_OFFSET) else {
        return Err("major version fixture offset is invalid".to_owned());
    };
    *major = 2;
    let Some(minor) = bytes.get_mut(FILE_INFO_VERSION_MINOR_OFFSET) else {
        return Err("minor version fixture offset is invalid".to_owned());
    };
    *minor = FORMAT_VERSION_MINOR;

    let result = validate_version(&bytes);

    let Err(ArchiveError::InvalidArchive(message)) = result else {
        return Err("unsupported major version was accepted".to_owned());
    };
    if !message.contains("unsupported RCF version 2.2") {
        return Err(format!("unexpected version error: {message}"));
    }
    Ok(())
}

#[test]
fn rejects_unsupported_minor_file_info_version() -> Result<(), String> {
    let mut bytes = vec![0_u8; FILE_INFO_VERSION_MINOR_OFFSET + 1];
    let Some(major) = bytes.get_mut(FILE_INFO_VERSION_MAJOR_OFFSET) else {
        return Err("major version fixture offset is invalid".to_owned());
    };
    *major = FORMAT_VERSION_MAJOR;
    let Some(minor) = bytes.get_mut(FILE_INFO_VERSION_MINOR_OFFSET) else {
        return Err("minor version fixture offset is invalid".to_owned());
    };
    *minor = 3;

    let result = validate_version(&bytes);

    let Err(ArchiveError::InvalidArchive(message)) = result else {
        return Err("unsupported minor version was accepted".to_owned());
    };
    if !message.contains("unsupported RCF version 1.3") {
        return Err(format!("unexpected version error: {message}"));
    }
    Ok(())
}

#[test]
fn computes_original_cement_name_hashes() {
    assert_eq!(name_hash32(r"sound\scripts\knigh_v.spt"), 0x062b_1126);
    assert_eq!(name_hash32(r"sound\scripts\ccube.spt"), 0x0726_f620);
    assert_eq!(name_hash32(r"sound\scripts\csedan.spt"), 0x0897_2da6);
}
