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

use super::super::json::JsonObject;
use super::{append_summary, is_parameter_name, is_sound_method};

#[test]
fn method_tokens_are_not_parameter_names() -> Result<(), String> {
    if !is_sound_method("SetVolume") || is_parameter_name("SetVolume") {
        return Err("SetVolume classification is inconsistent".to_owned());
    }
    let mut json = JsonObject::new();
    append_summary(&mut json, b"SetVolume\0carVolume");
    let output = json.finish();
    if output.contains("\"parameter_names\":[\"carVolume\"]")
        && !output.contains("\"parameter_names\":[\"SetVolume")
    {
        Ok(())
    } else {
        Err(format!("method/parameter output overlapped: {output}"))
    }
}
