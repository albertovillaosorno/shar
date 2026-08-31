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
//   - Decoded billboard animation source test module.
// - Must-Not:
//   - Own production behavior or broaden accepted source semantics.
// - Allows:
//   - Synthetic fixtures and assertions for exact BQG source evidence.
// - Split-When:
//   - Another animation family gains independent evidence ownership.
// - Merge-When:
//   - The billboard animation adapter no longer owns this contract.
// - Summary:
//   - Decoded billboard animation source tests.
// - Description:
//   - Locks exact BQG timing, group, and channel evidence preservation.
// - Usage:
//   - Runs as the dedicated FBX integration test target.
// - Defaults:
//   - Contradictory decoded evidence fails closed.
//

//! Decoded billboard animation source test module.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_billboard_animation_source::{
    BillboardAnimationChannelEvidence, read_billboard_animation_source_evidence,
};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn fixture_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fbx-decoded-billboard-animation-{label}-{}.json",
        std::process::id()
    ))
}

fn animation_fixture(group_count: usize) -> String {
    serde_json::json!({
        "schema": "animation",
        "name": "BQG_beam\0",
        "version": 0,
        "type": "BQG_",
        "frames": 10,
        "frame_rate": 30,
        "cyclic": 0,
        "sizes": [{
            "version": 1,
            "pc": 100,
            "ps2": 108,
            "xbox": 100,
            "gc": 100
        }],
        "group_lists": [{
            "version": 0,
            "num_groups": group_count,
            "groups": [{
                "version": 0,
                "name": "beam\0",
                "group_id": 0,
                "num_channels": 6,
                "channels": [{
                    "kind": "float1",
                    "version": 0,
                    "param": "\x57\x44\x54\x5f",
                    "num_frames": 2,
                    "frames": [0, 9],
                    "values": [[2], [3]],
                    "channel_metadata": [{
                        "kind": "interpolation_mode",
                        "version": 0,
                        "mode": 1
                    }]
                }, {
                    "kind": "float2",
                    "version": 0,
                    "param": "OFF_",
                    "num_frames": 1,
                    "frames": [0],
                    "values": [[0.25, -0.5]],
                    "channel_metadata": []
                }, {
                    "kind": "vector3",
                    "version": 0,
                    "param": "TRAN",
                    "num_frames": 1,
                    "frames": [0],
                    "values": [[1, 2, 3]],
                    "channel_metadata": []
                }, {
                    "kind": "quaternion",
                    "version": 0,
                    "param": "ROT_",
                    "num_frames": 1,
                    "frames": [0],
                    "values": [[1, 0, 0, 0]],
                    "channel_metadata": []
                }, {
                    "kind": "bool",
                    "version": 0,
                    "param": "VIS_",
                    "start_state": 0,
                    "num_frames": 2,
                    "values": [0, 1],
                    "channel_metadata": []
                }, {
                    "kind": "colour",
                    "version": 0,
                    "param": "\x43\x4c\x52\x5f",
                    "num_frames": 2,
                    "frames": [0, 9],
                    "values": [305_419_896_u32, u32::MAX],
                    "channel_metadata": []
                }]
            }]
        }],
        "loose_channels": [],
        "legacy_animation_extras": []
    })
    .to_string()
}

#[test]
fn retains_exact_billboard_group_animation_evidence() -> Result<(), String> {
    let path = fixture_path("source-evidence");
    fs::write(&path, animation_fixture(1))
        .map_err(|error| error.to_string())?;
    let result = read_billboard_animation_source_evidence(&path, "BQG_beam");
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let evidence =
        result.map_err(|error| format!("BQG evidence failed: {error:?}"))?;
    if evidence.identity != "BQG_beam"
        || evidence.version != 0
        || evidence.frame_count != 10.
        || evidence.frame_rate != 30.
        || evidence.cyclic
        || evidence.group_lists.len() != 1
    {
        return Err(format!("BQG animation evidence changed: {evidence:?}"));
    }
    let group = evidence
        .group_lists
        .first()
        .and_then(|list| list.groups.first())
        .ok_or_else(|| "BQG group evidence is missing".to_owned())?;
    if group.identity != "beam"
        || group.group_id != 0
        || group.channels.len() != 6
    {
        return Err(format!("BQG group evidence changed: {group:?}"));
    }
    if !matches!(
        group.channels.first(),
        Some(BillboardAnimationChannelEvidence::Float1 { .. })
    ) || !matches!(
        group.channels.get(4),
        Some(BillboardAnimationChannelEvidence::Bool { .. })
    ) || !matches!(
        group.channels.get(5),
        Some(BillboardAnimationChannelEvidence::Colour { .. })
    ) {
        return Err(format!("BQG channel order changed: {:?}", group.channels));
    }
    Ok(())
}

#[test]
fn rejects_contradictory_billboard_group_count() -> Result<(), String> {
    let path = fixture_path("group-count");
    fs::write(&path, animation_fixture(2))
        .map_err(|error| error.to_string())?;
    let result = read_billboard_animation_source_evidence(&path, "BQG_beam");
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    if result.is_ok() {
        return Err("contradictory BQG group count was accepted".to_owned());
    }
    Ok(())
}
