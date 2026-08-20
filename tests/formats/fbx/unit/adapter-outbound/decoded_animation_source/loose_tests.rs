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
//   - Decoded animation source loose unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Decoded animation source loose unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Assertions fail explicitly.
//

//! Decoded animation source loose unit tests.

use super::*;

#[test]
fn rejects_first_unrepresentable_frame_count() {
    let exclusive_max = usize::MAX as f64 + 1f64;

    assert_eq!(
        frame_count(exclusive_max),
        Err(DecodedAnimationError::InvalidFrameCount)
    );
}

#[test]
fn rejects_epsilon_fractional_frame_count() {
    assert_eq!(
        frame_count(1f64 + f64::EPSILON),
        Err(DecodedAnimationError::InvalidFrameCount)
    );
}

#[test]
fn rejects_declared_group_count_mismatch() {
    let error = decoded_groups(vec![DecodedGroupList {
        group_count: 1,
        groups: Vec::new(),
    }])
    .err();

    assert_eq!(
        error,
        Some(DecodedAnimationError::InvalidGroupCount {
            declared: 1,
            actual: 0,
        })
    );
}

#[test]
fn rejects_declared_channel_count_mismatch() {
    let error = decoded_groups(vec![DecodedGroupList {
        group_count: 1,
        groups: vec![DecodedGroup {
            name: "Root".to_owned(),
            channel_count: 1,
            channels: Vec::new(),
        }],
    }])
    .err();

    assert_eq!(
        error,
        Some(DecodedAnimationError::InvalidChannelCount {
            group: "Root".to_owned(),
            declared: 1,
            actual: 0,
        })
    );
}

#[test]
fn rejects_declared_key_count_mismatch() {
    let error = decoded_groups(vec![DecodedGroupList {
        group_count: 1,
        groups: vec![DecodedGroup {
            name: "Root".to_owned(),
            channel_count: 1,
            channels: vec![DecodedChannel {
                kind: "vector1".to_owned(),
                param: "TRAN".to_owned(),
                mapping: None,
                constants: None,
                key_count: 1,
                frames: Vec::new(),
                values: Vec::new(),
                compressed_values: Vec::new(),
                channel_metadata: Vec::new(),
            }],
        }],
    }])
    .err();

    assert_eq!(
        error,
        Some(DecodedAnimationError::InvalidKeyCount {
            group: "Root".to_owned(),
            parameter: "TRAN".to_owned(),
            declared: 1,
            actual: 0,
        })
    );
}

#[test]
fn rejects_embedded_identity_padding() {
    assert_eq!(
        trim_identity("Root\0Alias"),
        Err(DecodedAnimationError::InvalidIdentityPadding)
    );
}

#[test]
fn rejects_identity_control_characters() {
    assert_eq!(
        trim_identity("Root\nAlias"),
        Err(DecodedAnimationError::InvalidIdentityCharacter)
    );
}

#[test]
fn rejects_unsupported_bound_channel_parameter() {
    let bone = Bone {
        id: "Root".to_owned(),
        parent_id: None,
        rest_matrix: [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ],
    };
    let group = DecodedGroup {
        name: "Root".to_owned(),
        channel_count: 1,
        channels: vec![DecodedChannel {
            kind: "vector3".to_owned(),
            param: "FAIL".to_owned(),
            mapping: None,
            constants: None,
            key_count: 0,
            frames: Vec::new(),
            values: Vec::new(),
            compressed_values: Vec::new(),
            channel_metadata: Vec::new(),
        }],
    };
    let rest = LocalTransformSample {
        translation: [0.; 3],
        rotation_wxyz: [1., 0., 0., 0.],
    };

    assert_eq!(
        sample_track(&bone, &group, rest, 1),
        Err(DecodedAnimationError::UnsupportedChannelParameter {
            group: "Root".to_owned(),
            parameter: "FAIL".to_owned(),
        })
    );
}

#[test]
fn rest_rotation_ignores_nonuniform_scale() -> Result<(), String> {
    let bone = Bone {
        id: "Root".to_owned(),
        parent_id: None,
        rest_matrix: [
            0., 2., 0., 0., -1., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ],
    };
    let transforms = rest_transforms(&[bone])
        .map_err(|error| format!("rest transform failed: {error:?}"))?;
    let rotation = transforms
        .get("Root")
        .ok_or("root rest transform is missing")?
        .rotation_wxyz;
    let expected = std::f64::consts::FRAC_1_SQRT_2;
    let expected_rotation = [expected, 0., 0., expected];
    if rotation
        .iter()
        .zip(expected_rotation)
        .any(|(actual, expected)| (*actual - expected).abs() > 1e-6)
    {
        return Err(format!(
            "rest rotation inherited nonuniform scale: {rotation:?}"
        ));
    }
    Ok(())
}

#[test]
fn rejects_bound_keys_outside_clip_frame_count() {
    let bone = Bone {
        id: "Root".to_owned(),
        parent_id: None,
        rest_matrix: [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ],
    };
    let group = DecodedGroup {
        name: "Root".to_owned(),
        channel_count: 1,
        channels: vec![DecodedChannel {
            kind: "vector3".to_owned(),
            param: "TRAN".to_owned(),
            mapping: None,
            constants: None,
            key_count: 2,
            frames: vec![0, 2],
            values: vec![vec![0., 0., 0.], vec![2., 0., 0.]],
            compressed_values: Vec::new(),
            channel_metadata: Vec::new(),
        }],
    };
    let rest = LocalTransformSample {
        translation: [0.; 3],
        rotation_wxyz: [1., 0., 0., 0.],
    };

    assert_eq!(
        sample_track(&bone, &group, rest, 2),
        Err(DecodedAnimationError::InvalidKeySeries)
    );
}
