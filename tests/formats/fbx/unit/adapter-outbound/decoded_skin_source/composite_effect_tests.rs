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
//   - Composite effect rejection unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private decoded-composite fixtures and fail-closed assertions.
// - Split-When:
//   - Split when another composite payload family gains independent behavior.
// - Merge-When:
//   - Merge when another test module owns identical effect-rejection evidence.
// - Summary:
//   - Composite effect rejection unit tests.
// - Description:
//   - Verifies unsupported decoded effect bindings cannot disappear silently.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Fixtures are local and removed after each assertion.
//

//! Composite effect rejection unit tests.

use std::fs;
use std::path::PathBuf;

use super::{SkinSourceError, composite_bindings, load_character};

fn temp_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fbx-composite-effect-{label}-{}.json",
        std::process::id()
    ))
}

#[test]
fn effect_evidence_blocks_whole_character_export() -> Result<(), String> {
    let composite_path = temp_path("composite");
    let skeleton_path = temp_path("skeleton");
    let composite_fixture = concat!(
        r#"{"schema":"composite_drawable","name":"character","#,
        r#""skeleton_name":"skeleton","num_skins":0,"skins":[],"#,
        r#""num_props":0,"props":[],"num_effects":1,"effects":[{"#,
        r#""kind":"effect","name":"spark","is_translucent":0,"#,
        r#""skeleton_joint_id":0}]}"#,
    );
    let skeleton_fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    fs::write(&composite_path, composite_fixture)
        .and_then(|()| fs::write(&skeleton_path, skeleton_fixture))
        .map_err(|error| error.to_string())?;

    let bindings = composite_bindings(&composite_path, "skeleton", &[], 1)
        .map_err(|error| format!("effect evidence failed: {error:?}"))?;
    if bindings.effect_count != 1 {
        return Err(format!(
            "decoded effect evidence changed: {}",
            bindings.effect_count
        ));
    }
    let composite_paths = [composite_path.as_path()];
    let error =
        load_character("character", &skeleton_path, &[], &[], &composite_paths)
            .err();

    fs::remove_file(&composite_path)
        .and_then(|()| fs::remove_file(&skeleton_path))
        .map_err(|error| error.to_string())?;
    let expected = Some(SkinSourceError::UnsupportedCompositeEffects {
        path: composite_path.display().to_string(),
        count: 1,
    });
    if error == expected {
        Ok(())
    } else {
        Err(format!("whole-character effect result changed: {error:?}"))
    }
}
