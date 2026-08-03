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

#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::fs;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt as _;

use schoenwald_cli::CliProgram;

use super::{RtfConversionProgram, USAGE, run};

#[test]
fn excess_arguments_return_one_usage_diagnostic() -> Result<(), String> {
    let outcome = RtfConversionProgram.execute(&[
        "input".to_owned(),
        "output".to_owned(),
        "extra".to_owned(),
    ]);
    if outcome.status() != schoenwald_cli::ExitStatus::Failure {
        return Err("excess RTF arguments must fail".to_owned());
    }
    let [chunk] = outcome.output() else {
        return Err("RTF usage must emit one diagnostic".to_owned());
    };
    if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
        return Err("RTF usage must be written to stderr".to_owned());
    }
    if chunk.text() != format!("{USAGE}\n") {
        return Err(format!("unexpected RTF usage: {:?}", chunk.text()));
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn write_error_preserves_unpaired_utf16_destination_unit() -> Result<(), String>
{
    let root = std::env::temp_dir()
        .join(format!("schoenwald-rtf-diagnostic-{}", std::process::id()));
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let input = root.join("input.rtf");
    fs::write(&input, br"{\rtf1\ansi hello}")
        .map_err(|error| error.to_string())?;
    let output = root.join(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800,
        u16::from(b'b'),
    ]));

    let result = run(&input, Some(&output));

    fs::remove_file(&input).map_err(|error| error.to_string())?;
    fs::remove_dir(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("invalid destination unexpectedly succeeded".to_owned());
    };
    if !error.contains(r"a\u{D800}b") {
        return Err(format!("diagnostic lost native path unit: {error:?}"));
    }
    if error.contains('\u{fffd}') {
        return Err(format!("diagnostic used lossy replacement: {error:?}"));
    }
    Ok(())
}
