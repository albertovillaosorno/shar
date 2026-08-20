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

use std::path::PathBuf;

use super::{texture_key, unique_vehicle_component_paths};

#[test]
fn texture_key_removes_extension_case_and_fixed_width_padding() {
    assert_eq!(texture_key("WindsheildT.bmp\0\0"), "windsheildt");
    assert_eq!(texture_key("homer_vWheel.PNG"), "homer_vwheel");
}

#[test]
fn projected_component_paths_preserve_package_order() -> Result<(), String> {
    let paths = unique_vehicle_component_paths(
        [PathBuf::from("mesh/z.json"), PathBuf::from("mesh/a.json")],
        "mesh",
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(
        paths,
        [PathBuf::from("mesh/z.json"), PathBuf::from("mesh/a.json")]
    );
    Ok(())
}

#[test]
fn projected_component_path_collision_fails_closed() -> Result<(), String> {
    let result = unique_vehicle_component_paths(
        [
            PathBuf::from("components/mesh/shared.json"),
            PathBuf::from("components/mesh/shared.json"),
        ],
        "mesh",
    );
    let Err(error) = result else {
        return Err("duplicate projected vehicle path was accepted".to_owned());
    };
    assert_eq!(
        error.to_string(),
        concat!(
            "vehicle package projects duplicate mesh path: ",
            "components/mesh/shared.json"
        )
    );
    Ok(())
}
