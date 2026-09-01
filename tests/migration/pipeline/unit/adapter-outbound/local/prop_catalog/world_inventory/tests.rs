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
//   - World inventory unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - World inventory unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! World inventory unit tests.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{
    DeferredRenderAuthority, LedgerRow, decoded_mesh_names,
    deferred_controller_binding, deferred_render_bindings, effect_particle_pair,
    WorldPrimaryRelationships, primary_world_owner_classification,
    primary_world_source_binding, source_ordered_mesh_ids,
};
use crate::adapters::driven::local::prop_catalog::inventory_common::{
    CompositeEffectEvidence, CompositeEvidence, CompositePropEvidence,
};
use crate::adapters::driven::local::prop_catalog::model::WorldPrimaryMeshOrder;
use crate::adapters::driven::local::prop_catalog::texture_authority;
use crate::domain::package::{PackageRole, PhaseThreePackageMember};

fn source_member(
    id: &str,
    role: PackageRole,
    path: &str,
    kind: &str,
    source_kind: &str,
    ordinal: usize,
) -> PhaseThreePackageMember {
    PhaseThreePackageMember {
        id: id.to_owned(),
        role,
        path: path.to_owned(),
        unit_type: "source".to_owned(),
        kind: kind.to_owned(),
        source_chunk_kind: source_kind.to_owned(),
        source_chunk_ordinal: Some(ordinal),
    }
}

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn fixture_root(label: &str) -> Result<PathBuf, String> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "shar-world-inventory-map-{label}-{}-{sequence}",
        std::process::id()
    ));
    for family in [
        "mesh",
        "quad_group",
        "shader",
        "frame_controller",
        "animation",
        "particle_system_factory",
        "particle_system",
    ] {
        fs::create_dir_all(root.join("components").join(family))
            .map_err(|error| error.to_string())?;
    }
    Ok(root)
}

#[test]
fn decoded_mesh_names_reject_duplicate_source_identity() -> Result<(), String> {
    let root = fixture_root("duplicate-name")?;
    fs::write(root.join("components/mesh/001.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    fs::write(root.join("components/mesh/002.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    // jig-ignore-next-line: literal
    let result = decoded_mesh_names(&root, &["001".to_owned(), "002".to_owned()]);
    drop(fs::remove_dir_all(&root));
    let Err(error) = result else {
        return Err("duplicate decoded mesh identity was accepted".to_owned());
    };
    if !error
        .to_string()
        .contains("world prop repeats mesh identity shared")
    {
        // jig-ignore-next-line: literal
        return Err(format!("unexpected duplicate mesh identity error: {error}"));
    }
    Ok(())
}

#[test]
fn source_mesh_ids_follow_component_ordinals() -> Result<(), String> {
    let rows = vec![
        LedgerRow {
            ordinal: 20,
            depth: 2,
            container_ordinal: 1,
            name: "lexically-first".to_owned(),
            path: "mesh/a.json".to_owned(),
            kind: "mesh".to_owned(),
        },
        LedgerRow {
            ordinal: 10,
            depth: 2,
            container_ordinal: 1,
            name: "source-first".to_owned(),
            path: "mesh/z.json".to_owned(),
            kind: "mesh".to_owned(),
        },
    ];
    let actual = source_ordered_mesh_ids(&rows)
        .map_err(|error| error.to_string())?;
    assert_eq!(actual, ["z", "a"]);
    Ok(())
}

#[test]
fn primary_world_owner_classification_matches_package_taxonomy()
-> Result<(), String> {
    for (source_kind, expected_role, expected_kind) in [
        ("srr_dyna_phys_dsg", PackageRole::Physics, "p3d-physics"),
        (
            "srr_insta_anim_dyna_phys_dsg",
            PackageRole::Physics,
            "p3d-physics",
        ),
        (
            "srr_breakable_object",
            PackageRole::World,
            "p3d-world-dsg",
        ),
        ("srr_anim_dsg", PackageRole::World, "p3d-world-dsg"),
        ("srr_anim_coll_dsg", PackageRole::World, "p3d-world-dsg"),
        ("state_prop", PackageRole::World, "p3d-animated-prop"),
        (
            "animated_object_factory",
            PackageRole::World,
            "p3d-animated-prop",
        ),
    ] {
        let actual = primary_world_owner_classification(source_kind)
            .map_err(|error| error.to_string())?;
        assert_eq!(actual, (expected_role, expected_kind));
    }
    assert!(primary_world_owner_classification("mesh").is_err());
    Ok(())
}

#[test]
fn primary_world_source_keeps_composite_order_and_static_skeleton()
-> Result<(), String> {
    let root = fixture_root("primary-particle-pair")?;
    let owner = LedgerRow {
        ordinal: 5,
        depth: 1,
        container_ordinal: 5,
        name: "owner".to_owned(),
        path: "srr_insta_anim_dyna_phys_dsg/owner.json".to_owned(),
        kind: "srr_insta_anim_dyna_phys_dsg".to_owned(),
    };
    let mesh_a = LedgerRow {
        ordinal: 10,
        depth: 2,
        container_ordinal: 5,
        name: "mesh-a".to_owned(),
        path: "mesh/mesh-a.json".to_owned(),
        kind: "mesh".to_owned(),
    };
    let mesh_b = LedgerRow {
        ordinal: 20,
        depth: 2,
        container_ordinal: 5,
        name: "mesh-b".to_owned(),
        path: "mesh/mesh-b.json".to_owned(),
        kind: "mesh".to_owned(),
    };
    let composite = LedgerRow {
        ordinal: 30,
        depth: 2,
        container_ordinal: 5,
        name: "owner".to_owned(),
        path: "composite_drawable/owner-composite.json".to_owned(),
        kind: "composite_drawable".to_owned(),
    };
    let skeleton = LedgerRow {
        ordinal: 40,
        depth: 2,
        container_ordinal: 5,
        name: "owner-rig".to_owned(),
        path: "skeleton/owner-skeleton.json".to_owned(),
        kind: "skeleton".to_owned(),
    };
    let particle_factory = LedgerRow {
        ordinal: 50,
        depth: 1,
        container_ordinal: 50,
        name: "spark".to_owned(),
        path: "particle_system_factory/spark.json".to_owned(),
        kind: "particle_system_factory".to_owned(),
    };
    let particle_system = LedgerRow {
        ordinal: 60,
        depth: 1,
        container_ordinal: 60,
        name: "spark".to_owned(),
        path: "particle_system/spark.json".to_owned(),
        kind: "particle_system".to_owned(),
    };
    fs::write(
        root.join("components/particle_system_factory/spark.json"),
        r#"{"schema":"particle_system_factory","name":"spark"}"#,
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components/particle_system/spark.json"),
        r#"{"schema":"particle_system","name":"spark","factory_name":"spark"}"#,
    )
    .map_err(|error| error.to_string())?;
    let package_particle_rows = vec![
        particle_factory.clone(),
        particle_system.clone(),
    ];
    let rows = vec![
        mesh_a.clone(),
        mesh_b.clone(),
        composite.clone(),
        skeleton.clone(),
    ];
    let source_members = BTreeMap::from([
        (
            (owner.path.clone(), owner.ordinal),
            source_member(
                "owner-member-5",
                PackageRole::Physics,
                &owner.path,
                "p3d-physics",
                &owner.kind,
                owner.ordinal,
            ),
        ),
        (
            (mesh_a.path.clone(), mesh_a.ordinal),
            source_member(
                "mesh-member-10",
                PackageRole::Model,
                &mesh_a.path,
                "p3d-mesh",
                "mesh",
                mesh_a.ordinal,
            ),
        ),
        (
            (mesh_b.path.clone(), mesh_b.ordinal),
            source_member(
                "mesh-member-20",
                PackageRole::Model,
                &mesh_b.path,
                "p3d-mesh",
                "mesh",
                mesh_b.ordinal,
            ),
        ),
        (
            (composite.path.clone(), composite.ordinal),
            source_member(
                "composite-member-30",
                PackageRole::Model,
                &composite.path,
                "p3d-composite-drawable",
                "composite_drawable",
                composite.ordinal,
            ),
        ),
        (
            (skeleton.path.clone(), skeleton.ordinal),
            source_member(
                "skeleton-member-40",
                PackageRole::Animation,
                &skeleton.path,
                "p3d-skeleton",
                "skeleton",
                skeleton.ordinal,
            ),
        ),
        (
            (particle_factory.path.clone(), particle_factory.ordinal),
            source_member(
                "particle-factory-member-50",
                PackageRole::Particle,
                &particle_factory.path,
                "p3d-particle",
                "particle_system_factory",
                particle_factory.ordinal,
            ),
        ),
        (
            (particle_system.path.clone(), particle_system.ordinal),
            source_member(
                "particle-system-member-60",
                PackageRole::Particle,
                &particle_system.path,
                "p3d-particle",
                "particle_system",
                particle_system.ordinal,
            ),
        ),
    ]);
    let selected = ["mesh-b".to_owned(), "mesh-a".to_owned()];
    let composite_source = CompositeEvidence {
        member_id: "owner-composite".to_owned(),
        name: "owner".to_owned(),
        skeleton_name: "owner-rig".to_owned(),
        prop_names: selected.to_vec(),
        prop_bindings: vec![
            CompositePropEvidence {
                name: "mesh-b".to_owned(),
                skeleton_joint_id: 9,
                is_translucent: true,
                sort_order_bits: Some(0.4_f32.to_bits()),
            },
            CompositePropEvidence {
                name: "mesh-a".to_owned(),
                skeleton_joint_id: 3,
                is_translucent: false,
                sort_order_bits: Some(0.5_f32.to_bits()),
            },
        ],
        effect_bindings: vec![CompositeEffectEvidence {
            name: "spark".to_owned(),
            skeleton_joint_id: 7,
            is_translucent: true,
            sort_order_bits: Some(0.1_f32.to_bits()),
        }],
    };
    let mesh_names = BTreeMap::from([
        ("mesh-a".to_owned(), "mesh-a".to_owned()),
        ("mesh-b".to_owned(), "mesh-b".to_owned()),
    ]);
    let binding = primary_world_source_binding(
        &owner,
        &rows,
        &selected,
        WorldPrimaryRelationships {
            mesh_order: WorldPrimaryMeshOrder::CompositeProp,
            matched_composite: Some(&composite),
            composite_evidence: Some(&composite_source),
            mesh_names: Some(&mesh_names),
            referenced_skeleton: Some(&skeleton),
            exported_ptrn_animation: None,
            particle_root: Some(&root),
            particle_rows: Some(&package_particle_rows),
        },
        &source_members,
    )
    .map_err(|error| error.to_string())?;
    let selected_ids = binding
        .selected_meshes
        .iter()
        .map(|selected| selected.member.package_member_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(binding.owner.package_member_id, "owner-member-5");
    let [effect] = binding.composite_effects.as_slice() else {
        return Err("primary binding lost composite effect evidence".to_owned());
    };
    assert_eq!(effect.composite_effect_index, 0);
    assert_eq!(effect.source_identity, "spark");
    assert_eq!(effect.skeleton_joint_id, 7);
    assert!(effect.is_translucent);
    assert_eq!(effect.sort_order_bits, Some(0.1_f32.to_bits()));
    let pair = effect
        .package_particle_pair
        .as_ref()
        .ok_or_else(|| "primary effect lost exact particle pair".to_owned())?;
    assert_eq!(
        pair.factory.package_member_id,
        "particle-factory-member-50"
    );
    assert_eq!(pair.factory.source_ordinal, 50);
    assert_eq!(pair.system.package_member_id, "particle-system-member-60");
    assert_eq!(pair.system.source_ordinal, 60);
    assert_eq!(selected_ids, ["mesh-member-20", "mesh-member-10"]);
    let first_selected = binding
        .selected_meshes
        .first()
        .ok_or_else(|| "primary binding lost first selected mesh".to_owned())?;
    assert_eq!(first_selected.composite_prop_index, Some(0));
    assert_eq!(first_selected.skeleton_joint_id, Some(9));
    assert_eq!(first_selected.is_translucent, Some(true));
    assert_eq!(first_selected.sort_order_bits, Some(0.4_f32.to_bits()));
    assert_eq!(
        binding
            .matched_composite
            .as_ref()
            .map(|member| member.package_member_id.as_str()),
        Some("composite-member-30")
    );
    assert_eq!(
        binding
            .referenced_skeleton
            .as_ref()
            .map(|member| member.package_member_id.as_str()),
        Some("skeleton-member-40")
    );
    assert!(binding.exported_ptrn_animation.is_none());
    drop(fs::remove_dir_all(&root));
    Ok(())
}

#[test]
fn effect_particle_pair_stays_unresolved_when_factory_repeats()
-> Result<(), String> {
    let rows = vec![
        LedgerRow {
            ordinal: 1,
            depth: 1,
            container_ordinal: 1,
            name: "spark".to_owned(),
            path: "particle_system_factory/spark-a.json".to_owned(),
            kind: "particle_system_factory".to_owned(),
        },
        LedgerRow {
            ordinal: 2,
            depth: 1,
            container_ordinal: 2,
            name: "spark".to_owned(),
            path: "particle_system_factory/spark-b.json".to_owned(),
            kind: "particle_system_factory".to_owned(),
        },
        LedgerRow {
            ordinal: 3,
            depth: 1,
            container_ordinal: 3,
            name: "spark".to_owned(),
            path: "particle_system/spark.json".to_owned(),
            kind: "particle_system".to_owned(),
        },
    ];
    let root = fixture_root("duplicate-particle-factory")?;
    let result = effect_particle_pair(&root, &rows, &BTreeMap::new(), "spark")
        .map_err(|error| error.to_string())?;
    drop(fs::remove_dir_all(&root));
    assert!(result.is_none());
    Ok(())
}

#[test]
fn effect_particle_pair_rejects_factory_name_mismatch() -> Result<(), String> {
    let root = fixture_root("particle-factory-name-mismatch")?;
    fs::write(
        root.join("components/particle_system_factory/spark.json"),
        r#"{"schema":"particle_system_factory","name":"spark"}"#,
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components/particle_system/spark.json"),
        r#"{"schema":"particle_system","name":"spark","factory_name":"smoke"}"#,
    )
    .map_err(|error| error.to_string())?;
    let rows = vec![
        LedgerRow {
            ordinal: 1,
            depth: 1,
            container_ordinal: 1,
            name: "spark".to_owned(),
            path: "particle_system_factory/spark.json".to_owned(),
            kind: "particle_system_factory".to_owned(),
        },
        LedgerRow {
            ordinal: 2,
            depth: 1,
            container_ordinal: 2,
            name: "spark".to_owned(),
            path: "particle_system/spark.json".to_owned(),
            kind: "particle_system".to_owned(),
        },
    ];
    let result = effect_particle_pair(&root, &rows, &BTreeMap::new(), "spark");
    drop(fs::remove_dir_all(&root));
    let Err(error) = result else {
        return Err("mismatched particle factory name was accepted".to_owned());
    };
    if !error
        .to_string()
        .contains("particle pair disagrees with identity spark")
    {
        return Err(format!("unexpected particle mismatch error: {error}"));
    }
    Ok(())
}

#[test]
fn deferred_bindings_resolve_unique_package_quad_group_occurrence()
-> Result<(), String> {
    let composite = CompositeEvidence {
        member_id: "composite".to_owned(),
        name: "owner".to_owned(),
        skeleton_name: "rig".to_owned(),
        prop_names: vec![
            "body".to_owned(),
            "beam".to_owned(),
            "logical-only".to_owned(),
        ],
        prop_bindings: vec![
            CompositePropEvidence {
                name: "body".to_owned(),
                skeleton_joint_id: 0,
                is_translucent: false,
                sort_order_bits: Some(0.5_f32.to_bits()),
            },
            CompositePropEvidence {
                name: "beam".to_owned(),
                skeleton_joint_id: 3,
                is_translucent: true,
                sort_order_bits: Some(0.49_f32.to_bits()),
            },
            CompositePropEvidence {
                name: "logical-only".to_owned(),
                skeleton_joint_id: 4,
                is_translucent: false,
                sort_order_bits: None,
            },
        ],
        effect_bindings: Vec::new(),
    };
    let rows = Vec::new();
    let package_relationships = vec![
        LedgerRow {
            ordinal: 7,
            depth: 1,
            container_ordinal: 7,
            name: "glow_m\x00".to_owned(),
            path: "shader/glow_m__ordinal_7.json".to_owned(),
            kind: "shader".to_owned(),
        },
        LedgerRow {
            ordinal: 22,
            depth: 1,
            container_ordinal: 22,
            name: "beam\x00".to_owned(),
            path: "quad_group/beam__ordinal_22.json".to_owned(),
            kind: "quad_group".to_owned(),
        },
        LedgerRow {
            ordinal: 3,
            depth: 1,
            container_ordinal: 3,
            name: "glow_m\x00".to_owned(),
            path: "shader/glow_m__ordinal_3.json".to_owned(),
            kind: "shader".to_owned(),
        },
    ];
    let meshes = BTreeMap::from([(
        "body".to_owned(),
        "body__ordinal_10".to_owned(),
    )]);

    let root = fixture_root("deferred-package-quad")?;
    fs::write(
        root.join("components/shader/glow_m__ordinal_7.json"),
        concat!(
            r#"{"schema":"shader","name":"glow_m\u0000","version":0,"#,
            r#""pddi_shader_name":"simple\u0000","has_translucency":1,"#,
            r#""vertex_needs":33,"vertex_mask":7,"num_params":2,"#,
            r#""params":[{"kind":"texture","param":"TEX","#,
            r#""value":"glow.bmp\u0000"},{"kind":"int","#,
            r#""param":"BLMD","value":3}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components/shader/glow_m__ordinal_3.json"),
        concat!(
            r#"{"schema":"shader","name":"glow_m\u0000","version":0,"#,
            r#""pddi_shader_name":"simple\u0000","has_translucency":1,"#,
            r#""vertex_needs":33,"vertex_mask":3,"num_params":2,"#,
            r#""params":[{"kind":"texture","param":"TEX","#,
            r#""value":"glow.bmp\u0000"},{"kind":"int","#,
            r#""param":"BLMD","value":2}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components/quad_group/beam__ordinal_22.json"),
        concat!(
            r#"{"schema":"quad_group","version":0,"name":"beam","#,
            r#""shader":"glow_m","z_test":1,"z_write":0,"fog":0,"#,
            r#""num_quads":1,"quads":[{"name":"beam-child","#,
            r#""version":2,"billboard_mode":"YAX","#,
            r#""translation":[2.5,-3,4],"colour":305419896,"#,
            r#""uvs":[[0.1,0.2],[0.9,0.2],[0.9,0.8],[0.1,0.8]],"#,
            r#""width":2.25,"height":4.5,"distance":-0.35,"#,
            r#""uv_offset":[0.25,-0.5],"rotation_wxyz":[1,0,0,0],"#,
            r#""cutoff_mode":"DBL","uv_offset_range":[0.5,0.75],"#,
            r#""source_range":1.25,"edge_range":0.625,"#,
            r#""perspective":false}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let authority =
        texture_authority::SharedTextureAuthority::from_occurrences_for_tests(&[
            texture_authority::TextureOccurrenceFixture {
                logical: "glow.bmp",
                package_id: "level-three-terrain",
                subcategory: "terrain-world/level-03/terrain-mesh",
                package_member_id: "texture-member-182",
                member_id: "glow",
                source_ordinal: 182,
                path: "glow.png",
                sha256: "one",
            },
            texture_authority::TextureOccurrenceFixture {
                logical: "glow.bmp",
                package_id: "level-three-terrain",
                subcategory: "terrain-world/level-03/terrain-mesh",
                package_member_id: "texture-member-10591",
                member_id: "glow__ordinal_10591",
                source_ordinal: 10_591,
                path: "glow__ordinal_10591.png",
                sha256: "two",
            },
        ]);
    let source_members = BTreeMap::from([
        (
            ("shader/glow_m__ordinal_3.json".to_owned(), 3),
            source_member(
                "shader-member-3",
                PackageRole::Material,
                "shader/glow_m__ordinal_3.json",
                "p3d-shader",
                "shader",
                3,
            ),
        ),
        (
            ("shader/glow_m__ordinal_7.json".to_owned(), 7),
            source_member(
                "shader-member-7",
                PackageRole::Material,
                "shader/glow_m__ordinal_7.json",
                "p3d-shader",
                "shader",
                7,
            ),
        ),
        (
            ("quad_group/beam__ordinal_22.json".to_owned(), 22),
            source_member(
                "quad-member-22",
                PackageRole::Model,
                "quad_group/beam__ordinal_22.json",
                "p3d-mesh",
                "quad_group",
                22,
            ),
        ),
    ]);
    let deferred_authority = DeferredRenderAuthority {
        source_members: &source_members,
        texture_authority: &authority,
        source_subcategory: "terrain-world/level-03/regions/l3r1",
    };
    let actual = deferred_render_bindings(
        &root,
        &rows,
        &package_relationships,
        &composite,
        &meshes,
        &deferred_authority,
    )
        .map_err(|error| error.to_string())?;
    drop(fs::remove_dir_all(&root));
    if actual.len() != 2 {
        return Err(format!("unexpected deferred binding count: {actual:?}"));
    }
    let beam = actual
        .first()
        .ok_or_else(|| "resolved deferred binding is missing".to_owned())?;
    if beam.composite_prop_index != 1
        || beam.source_identity != "beam"
        || beam.skeleton_joint_id != 3
        || !beam.is_translucent
        || beam.sort_order_bits != Some(0.49_f32.to_bits())
        || beam.component_kind.as_deref() != Some("quad_group")
        || beam.component_package_member_id.as_deref() != Some("quad-member-22")
        || beam.component_member_id.as_deref() != Some("beam__ordinal_22")
        || beam.source_ordinal != Some(22)
        || beam.controller.is_some()
    {
        return Err(format!("resolved deferred binding changed: {beam:?}"));
    }
    let billboard = beam
        .billboard
        .as_ref()
        .ok_or_else(|| "resolved billboard evidence is missing".to_owned())?;
    let quad = billboard
        .quads
        .first()
        .ok_or_else(|| "resolved billboard child is missing".to_owned())?;
    let shader_ordinals = billboard
        .shader_occurrences
        .iter()
        .map(|shader| shader.source_ordinal)
        .collect::<Vec<_>>();
    let shader_package_members = billboard
        .shader_occurrences
        .iter()
        .map(|shader| shader.package_member_id.as_str())
        .collect::<Vec<_>>();
    let shader_members = billboard
        .shader_occurrences
        .iter()
        .map(|shader| shader.member_id.as_str())
        .collect::<Vec<_>>();
    let shader_textures = billboard
        .shader_occurrences
        .iter()
        .map(|shader| shader.texture_reference.as_deref())
        .collect::<Vec<_>>();
    let texture_reference = billboard
        .texture_references
        .first()
        .ok_or_else(|| "deferred texture reference is missing".to_owned())?;
    let texture_package_members = texture_reference
        .occurrences
        .iter()
        .map(|occurrence| occurrence.package_member_id.as_str())
        .collect::<Vec<_>>();
    let texture_members = texture_reference
        .occurrences
        .iter()
        .map(|occurrence| occurrence.member_id.as_str())
        .collect::<Vec<_>>();
    let texture_ordinals = texture_reference
        .occurrences
        .iter()
        .map(|occurrence| occurrence.source_ordinal)
        .collect::<Vec<_>>();
    let texture_digests = texture_reference
        .occurrences
        .iter()
        .map(|occurrence| occurrence.sha256.as_str())
        .collect::<Vec<_>>();
    let blend_modes = billboard
        .shader_occurrences
        .iter()
        .map(|shader| {
            shader
                .params
                .get(1)
                .and_then(|parameter| parameter.value.as_u64())
        })
        .collect::<Vec<_>>();
    if billboard.version != 0
        || billboard.shader_identity != "glow_m"
        || shader_ordinals != [3, 7]
        || shader_package_members != ["shader-member-3", "shader-member-7"]
        || shader_members != ["glow_m__ordinal_3", "glow_m__ordinal_7"]
        || shader_textures != [Some("glow.bmp"), Some("glow.bmp")]
        || billboard.texture_references.len() != 1
        || texture_reference.identity != "glow.bmp"
        || texture_package_members
            != ["texture-member-182", "texture-member-10591"]
        || texture_members != ["glow", "glow__ordinal_10591"]
        || texture_ordinals != [182, 10_591]
        || texture_digests != ["one", "two"]
        || blend_modes != [Some(2), Some(3)]
        || billboard.z_test != 1
        || billboard.z_write != 0
        || billboard.fog != 0
        || billboard.quads.len() != 1
        || quad.identity != "beam-child"
        || quad.version != 2
        || quad.billboard_mode != "YAX"
        || quad.translation_bits != [2.5_f32, -3., 4.].map(f32::to_bits)
        || quad.colour != 305_419_896
        || quad.width_bits != 2.25_f32.to_bits()
        || quad.height_bits != 4.5_f32.to_bits()
        || quad.distance_bits != (-0.35_f32).to_bits()
        || quad.cutoff_mode != "DBL"
        || quad.source_range_bits != 1.25_f32.to_bits()
        || quad.edge_range_bits != 0.625_f32.to_bits()
        || quad.perspective
    {
        return Err(format!("billboard evidence changed: {billboard:?}"));
    }
    let logical = actual
        .get(1)
        .ok_or_else(|| "logical deferred binding is missing".to_owned())?;
    if logical.composite_prop_index != 2
        || logical.source_identity != "logical-only"
        || logical.skeleton_joint_id != 4
        || logical.is_translucent
        || logical.sort_order_bits.is_some()
        || logical.component_kind.is_some()
        || logical.component_package_member_id.is_some()
        || logical.component_member_id.is_some()
        || logical.source_ordinal.is_some()
        || logical.billboard.is_some()
    {
        return Err(format!("logical deferred binding changed: {logical:?}"));
    }
    Ok(())
}

#[test]
fn deferred_controller_retains_exact_animation_relationship()
-> Result<(), String> {
    let root = fixture_root("controller-animation")?;
    fs::write(
        root.join("components/frame_controller/BQG_beam.json"),
        concat!(
            r#"{"schema":"frame_controller","name":"BQG_beam","#,
            r#""version":0,"type":"BQG","frame_offset":1.25,"#,
            r#""hierarchy_name":"beam","animation_name":"BQG_beam"}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let animation = serde_json::json!({
        "schema": "animation",
        "name": "BQG_beam",
        "version": 0,
        "type": "BQG_",
        "frames": 25,
        "frame_rate": 30,
        "cyclic": 0,
        "sizes": [{
            "version": 1,
            "pc": 152,
            "ps2": 160,
            "xbox": 152,
            "gc": 152
        }],
        "group_lists": [{
            "version": 0,
            "num_groups": 1,
            "groups": [{
                "version": 0,
                "name": "beam",
                "group_id": 0,
                "num_channels": 3,
                "channels": [{
                    "kind": "float1",
                    "version": 0,
                    "param": "\x57\x44\x54\x5f",
                    "num_frames": 1,
                    "frames": [0],
                    "values": [[2.25]],
                    "channel_metadata": [{
                        "kind": "interpolation_mode",
                        "version": 0,
                        "mode": 1
                    }]
                }, {
                    "kind": "float1",
                    "version": 0,
                    "param": "HGT_",
                    "num_frames": 1,
                    "frames": [0],
                    "values": [[4.5]],
                    "channel_metadata": [{
                        "kind": "interpolation_mode",
                        "version": 0,
                        "mode": 1
                    }]
                }, {
                    "kind": "bool",
                    "version": 0,
                    "param": "VIS_",
                    "start_state": 1,
                    "num_frames": 2,
                    "values": [3, 8],
                    "channel_metadata": []
                }]
            }]
        }],
        "loose_channels": [],
        "legacy_animation_extras": []
    });
    fs::write(
        root.join("components/animation/animation_0001.json"),
        animation.to_string(),
    )
    .map_err(|error| error.to_string())?;
    let rows = vec![
        LedgerRow {
            ordinal: 43,
            depth: 2,
            container_ordinal: 1,
            name: "BQG_beam".to_owned(),
            path: "frame_controller/BQG_beam.json".to_owned(),
            kind: "frame_controller".to_owned(),
        },
        LedgerRow {
            ordinal: 41,
            depth: 2,
            container_ordinal: 1,
            name: "BQG_beam".to_owned(),
            path: "animation/animation_0001.json".to_owned(),
            kind: "animation".to_owned(),
        },
    ];
    let source_members = BTreeMap::from([
        (
            ("frame_controller/BQG_beam.json".to_owned(), 43),
            source_member(
                "controller-member-43",
                PackageRole::Controller,
                "frame_controller/BQG_beam.json",
                "p3d-controller",
                "frame_controller",
                43,
            ),
        ),
        (
            ("animation/animation_0001.json".to_owned(), 41),
            source_member(
                "animation-member-41",
                PackageRole::Animation,
                "animation/animation_0001.json",
                "p3d-animation",
                "animation",
                41,
            ),
        ),
    ]);
    let result = deferred_controller_binding(
        &root,
        &rows,
        "beam",
        Some(&["beam"]),
        &source_members,
    )
    .map_err(|error| error.to_string());
    let mismatch = deferred_controller_binding(
        &root,
        &rows,
        "beam",
        Some(&["other-child"]),
        &source_members,
    );
    drop(fs::remove_dir_all(&root));
    if mismatch.is_ok() {
        return Err("mismatched BQG child relationship was accepted".to_owned());
    }
    let binding = result?
        .ok_or_else(|| "controller relationship was not retained".to_owned())?;
    if binding.controller_identity != "BQG_beam"
        || binding.controller_kind != "frame_controller"
        || binding.controller_package_member_id != "controller-member-43"
        || binding.controller_member_id != "BQG_beam"
        || binding.controller_source_ordinal != 43
        || binding.controller_version != 0
        || binding.controller_type != "BQG"
        || f32::from_bits(binding.frame_offset_bits) != 1.25
        || binding.animation_identity != "BQG_beam"
        || binding.animation_package_member_id.as_deref()
            != Some("animation-member-41")
        || binding.animation_member_id.as_deref() != Some("animation_0001")
        || binding.animation_source_ordinal != Some(41)
        || binding.animation_version != Some(0)
        || binding.animation_type.as_deref() != Some("BQG_")
        || binding
            .animation_source
            .as_ref()
            .and_then(|source| source.get("frame_count"))
            != Some(&25.0.into())
        || binding
            .animation_source
            .as_ref()
            .and_then(|source| source.get("group_lists"))
            .and_then(serde_json::Value::as_array)
            .map(Vec::len)
            != Some(1)
        || binding
            .animation_source
            .as_ref()
            .and_then(|source| source.get("group_lists"))
            .and_then(|lists| lists.get(0))
            .and_then(|list| list.get("groups"))
            .and_then(|groups| groups.get(0))
            .and_then(|group| group.get("channels"))
            .and_then(|channels| channels.get(2))
            .and_then(|channel| channel.get("raw_values"))
            != Some(&serde_json::json!([3, 8]))
    {
        return Err(format!("controller relationship changed: {binding:?}"));
    }
    Ok(())
}
