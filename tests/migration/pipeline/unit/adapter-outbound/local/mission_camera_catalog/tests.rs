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
//   - Mission camera decoded-component intake unit regressions.
// - Must-Not:
//   - Depend on installed game state or filename-derived identity.
// - Allows:
//   - Synthetic camera/multi-controller JSON and indexed path metadata.
// - Split-When:
//   - Camera and controller intake policies diverge independently.
// - Merge-When:
//   - Mission camera intake loses adapter-specific behavior.
// - Summary:
//   - Mission camera catalog adapter tests.
// - Description:
//   - Locks embedded-name decoding, member classification, and path safety.
// - Usage:
//   - Included only by the local mission camera catalog adapter.
// - Defaults:
//   - Interior control data and schema drift fail closed.
//

//! Mission camera catalog adapter tests.

use super::*;

#[test]
fn camera_name_is_read_from_embedded_json() -> Result<(), String> {
    let name = parse_component_name(
        r#"{"schema":"camera","name":"mission2camShape"}"#,
        MissionCameraComponentKind::Camera,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(name, "mission2camShape");
    Ok(())
}

#[test]
fn multicontroller_trims_only_trailing_nul_padding() -> Result<(), String> {
    let name = parse_component_name(
        r#"{"schema":"multi_controller","name":"mission2cam\u0000"}"#,
        MissionCameraComponentKind::MultiController,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(name, "mission2cam");
    Ok(())
}

#[test]
fn camera_name_rejects_surrounding_whitespace() -> Result<(), String> {
    let result = parse_component_name(
        r#"{"schema":"camera","name":" mission2camShape "}"#,
        MissionCameraComponentKind::Camera,
    );
    if result.is_ok() {
        return Err("surrounding camera whitespace was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn component_schema_must_match_indexed_kind() -> Result<(), String> {
    let Err(error) = parse_component_name(
        r#"{"schema":"camera","name":"mission2cam"}"#,
        MissionCameraComponentKind::MultiController,
    ) else {
        return Err("camera schema satisfied multicontroller member".to_owned());
    };
    assert!(error.to_string().contains("does not match member kind"));
    Ok(())
}

#[test]
fn observed_member_classifications_are_exact() -> Result<(), String> {
    assert_eq!(
        camera_member_kind(
            PackageRole::Camera,
            "camera",
            "p3d-camera",
            "camera",
        )
        .map_err(|error| error.to_string())?,
        Some(MissionCameraComponentKind::Camera)
    );
    assert_eq!(
        camera_member_kind(
            PackageRole::Controller,
            "controller",
            "p3d-controller",
            "multi_controller",
        )
        .map_err(|error| error.to_string())?,
        Some(MissionCameraComponentKind::MultiController)
    );
    assert_eq!(
        camera_member_kind(
            PackageRole::Controller,
            "controller",
            "p3d-controller",
            "frame_controller",
        )
        .map_err(|error| error.to_string())?,
        None
    );
    Ok(())
}

#[test]
fn member_path_stays_below_extracted_root() -> Result<(), String> {
    let root = Path::new("C:/work/extracted");
    let path = resolve_member_path(
        root,
        "extracted/art/missions/level01/mission2cam/components/camera/cam.json",
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(
        path,
        root.join(
            "art/missions/level01/mission2cam/components/camera/cam.json"
        )
    );
    assert!(
        resolve_member_path(root, "extracted/art/../outside.json").is_err()
    );
    Ok(())
}
