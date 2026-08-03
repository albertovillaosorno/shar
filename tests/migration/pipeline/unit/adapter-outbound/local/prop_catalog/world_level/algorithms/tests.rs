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

use super::model::{FbxFingerprint, FbxRepairAlgorithm};
use super::{
    apply_registered_algorithm, ensure_source_compatible, select_algorithm,
    similarity,
};
use crate::domain::PipelineError;

const EXACT: FbxFingerprint = FbxFingerprint {
    meshes: 10,
    groups: 20,
    positions: 1_000,
    triangles: 500,
    uvs: 1_000,
    normals: 1_000,
    colors: 0,
};

fn verify_context(
    relative_path: &str,
    meshes: &mut [fbx::domain::mesh::MeshAsset],
) -> Result<(), PipelineError> {
    if relative_path.is_empty() {
        return Err(PipelineError::new("FBX repair test path is empty"));
    }
    let _ = FbxFingerprint::from_meshes(meshes)?;
    Ok(())
}

const FIRST: FbxRepairAlgorithm = FbxRepairAlgorithm {
    relative_path: "level-01-zones-l1z1.fbx",
    file_stem: "level-01-zones-l1z1",
    file_prefix: "level-01-zones",
    source_fingerprint: EXACT,
    apply: verify_context,
};

const SECOND: FbxRepairAlgorithm = FbxRepairAlgorithm {
    relative_path: "level-02-zones-l2z1.fbx",
    file_stem: "level-02-zones-l2z1",
    file_prefix: "level-02-zones",
    source_fingerprint: FbxFingerprint {
        meshes: 8,
        groups: 18,
        positions: 800,
        triangles: 400,
        uvs: 800,
        normals: 800,
        colors: 0,
    },
    apply: verify_context,
};

#[test]
fn exact_path_precedes_similarity() -> Result<(), String> {
    let registry = [FIRST, SECOND];
    let selected = select_algorithm(
        "LEVEL-01-ZONES-L1Z1.FBX",
        SECOND.source_fingerprint,
        &registry,
    )
    .map_err(|error| error.to_string())?
    .ok_or_else(|| String::from("exact path did not select an algorithm"))?;
    if selected.relative_path != FIRST.relative_path {
        return Err(String::from("exact path lost selection priority"));
    }
    Ok(())
}

#[test]
fn empty_registry_is_a_no_op() -> Result<(), String> {
    let mut meshes = [];
    apply_registered_algorithm("unregistered.fbx", &mut meshes)
        .map_err(|error| error.to_string())
}

#[test]
fn exact_identity_rejects_incompatible_source_structure() -> Result<(), String>
{
    if ensure_source_compatible(
        FIRST.relative_path,
        SECOND.source_fingerprint,
        &FIRST,
    )
    .is_ok()
    {
        return Err(String::from("incompatible exact source was accepted"));
    }
    Ok(())
}

#[test]
fn structural_match_requires_ninety_nine_percent_per_dimension()
-> Result<(), String> {
    let close = FbxFingerprint {
        positions: 995,
        uvs: 995,
        normals: 995,
        ..EXACT
    };
    let score = similarity(EXACT, close).map_err(|error| error.to_string())?;
    if score != 9_950 {
        return Err(format!("near-identical fingerprint scored {score}"));
    }
    let weak = FbxFingerprint { positions: 989, ..EXACT };
    let weak_score =
        similarity(EXACT, weak).map_err(|error| error.to_string())?;
    if weak_score >= 9_900 {
        return Err(format!("weak fingerprint scored {weak_score}"));
    }
    Ok(())
}

#[test]
fn unique_structural_match_selects_the_closest_algorithm() -> Result<(), String>
{
    let close = FbxFingerprint {
        positions: 995,
        uvs: 995,
        normals: 995,
        ..EXACT
    };
    let registry = [FIRST, SECOND];
    let selected = select_algorithm("unknown.fbx", close, &registry)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| String::from("structural match was not selected"))?;
    if selected.relative_path != FIRST.relative_path {
        return Err(String::from(
            "structural match selected the wrong algorithm",
        ));
    }
    Ok(())
}

#[test]
fn ambiguous_structural_match_fails_closed() -> Result<(), String> {
    let duplicate = FbxRepairAlgorithm {
        relative_path: "alternate/same-structure.fbx",
        file_stem: "same-structure",
        file_prefix: "alternate-structure",
        ..FIRST
    };
    let registry = [FIRST, duplicate];
    if select_algorithm("unknown.fbx", EXACT, &registry).is_ok() {
        return Err(String::from("ambiguous structural match was accepted"));
    }
    Ok(())
}

#[test]
fn ambiguous_prefix_fails_closed() -> Result<(), String> {
    let duplicate = FbxRepairAlgorithm {
        relative_path: "alternate/level-01-zones-l1z2.fbx",
        file_stem: "level-01-zones-l1z2",
        ..FIRST
    };
    let registry = [FIRST, duplicate];
    let result =
        select_algorithm("unknown/level-01-zones-new.fbx", EXACT, &registry);
    if result.is_ok() {
        return Err(String::from("ambiguous prefix was accepted"));
    }
    Ok(())
}
