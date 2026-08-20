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
//   - External contract tests for normalized SHAR mod manifests.
// - Must-Not:
//   - Execute package code, inspect private game data, or mutate user content.
// - Allows:
//   - Synthetic package manifests, members, and deterministic assertions.
// - Split-When:
//   - Another package-contract version requires independent fixtures.
// - Merge-When:
//   - Another test module owns the identical normalized manifest contract.
// - Summary:
//   - Normalized SHAR mod-package external contract tests.
// - Description:
//   - Proves deterministic identity, path safety, trust, and relationships.
// - Usage:
//   - Runs as the mod-package crate's external contract test.
// - Defaults:
//   - Invalid, ambiguous, or nondeterministic manifests fail closed.
//

//! External contract tests for normalized SHAR mod manifests.

use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_mod_package::{
    CONTRACT_VERSION, Dependency, PackageKind, PackageManifest, Provenance,
    TrustLevel, content_revision, dependency_load_order, member_from_bytes,
};
use shar_sha256 as _;
use unicode_normalization as _;

fn manifest() -> Result<PackageManifest, Box<dyn std::error::Error>> {
    let mut members = vec![
        member_from_bytes(
            "content/localization/text.jsonl",
            "application/jsonl",
            "localization/text",
            b"{\"key\":\"HELLO\"}\n",
        )?,
        member_from_bytes(
            "content/source/Léeme.rtf",
            "application/rtf",
            "localization/readme",
            b"readme",
        )?,
    ];
    members.sort_by(|left, right| left.path.cmp(&right.path));
    let revision = content_revision(&members)?;
    Ok(PackageManifest {
        contract_version: CONTRACT_VERSION.to_owned(),
        canonical_id: "shar.localization.spanish".to_owned(),
        package_revision: revision,
        package_kind: PackageKind::Content,
        priority: 100,
        dependencies: Vec::new(),
        conflicts: Vec::new(),
        supersedes: Vec::new(),
        required_capabilities: vec!["localization.overlay.v1".to_owned()],
        supported_targets: Vec::new(),
        members,
        provenance: Provenance {
            authors: vec!["original-rightsholders".to_owned()],
            source: String::from(
                "generated-from-user-supplied-lawful-original-game",
            ),
            license: "NOASSERTION".to_owned(),
        },
        trust_level: TrustLevel::ContentOnly,
    })
}

#[test]
fn content_manifest_round_trips_deterministically()
-> Result<(), Box<dyn std::error::Error>> {
    let manifest = manifest()?;
    let first = manifest.to_pretty_json()?;
    let reparsed = PackageManifest::from_json(&first)?;
    let second = reparsed.to_pretty_json()?;
    assert_eq!(
        manifest, reparsed,
        "round-tripped manifest identity changed"
    );
    assert_eq!(first, second, "canonical serialization changed after parse");
    Ok(())
}

#[test]
fn storage_location_is_not_package_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let manifest = manifest()?;
    let text = manifest.to_pretty_json()?;
    let linux_home_prefix = ["/", "home", "/"].concat();
    assert!(
        !text.contains(&linux_home_prefix),
        "manifest leaked a Linux storage path"
    );
    assert!(
        !text.contains("C:\\"),
        "manifest leaked a Windows storage path"
    );
    assert_eq!(manifest.canonical_id, "shar.localization.spanish");
    Ok(())
}

#[test]
fn rejects_unknown_fields_and_noncanonical_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let text = manifest()?.to_pretty_json()?;
    let unknown = text.replacen(
        "\"contract_version\":",
        "\"unknown\": true,\n  \"contract_version\":",
        1,
    );
    assert!(
        PackageManifest::from_json(&unknown).is_err(),
        "unknown manifest fields must fail closed"
    );

    let mut invalid = manifest()?;
    invalid.canonical_id = "SHAR.Localization.Spanish".to_owned();
    assert!(
        invalid.validate().is_err(),
        "mixed-case identity was accepted"
    );
    Ok(())
}

#[test]
fn rejects_path_aliases_collisions_and_traversal()
-> Result<(), Box<dyn std::error::Error>> {
    let mut collision = manifest()?;
    collision.members.push(member_from_bytes(
        "content/SOURCE/Léeme.rtf",
        "application/rtf",
        "localization/readme",
        b"other",
    )?);
    collision
        .members
        .sort_by(|left, right| left.path.cmp(&right.path));
    assert!(
        collision.validate().is_err(),
        "portable path collision was accepted"
    );

    assert!(
        member_from_bytes(
            "../escape.bin",
            "application/octet-stream",
            "content/data",
            b"escape",
        )
        .is_err(),
        "parent traversal member was accepted"
    );
    assert!(
        member_from_bytes(
            "content\\alias.bin",
            "application/octet-stream",
            "content/data",
            b"alias",
        )
        .is_err(),
        "backslash path alias was accepted"
    );
    Ok(())
}

#[test]
fn member_construction_and_revision_share_manifest_validation()
-> Result<(), Box<dyn std::error::Error>> {
    assert!(
        member_from_bytes(
            "mod.json",
            "application/json",
            "content/manifest",
            b"{}",
        )
        .is_err(),
        "reserved manifest member was constructed"
    );

    let mut invalid_role = manifest()?.members;
    let first = invalid_role
        .first_mut()
        .ok_or("fixture did not contain a member")?;
    first.role = "Content/Invalid".to_owned();
    assert!(
        content_revision(&invalid_role).is_err(),
        "revision hashed a noncanonical member role"
    );

    let mut oversized = manifest()?.members;
    let first = oversized
        .first_mut()
        .ok_or("fixture did not contain a member")?;
    first.bytes = 8 * 1024 * 1024 * 1024 + 1;
    assert!(
        content_revision(&oversized).is_err(),
        "revision hashed an oversized member"
    );
    Ok(())
}

#[test]
fn package_revision_must_bind_canonical_members()
-> Result<(), Box<dyn std::error::Error>> {
    let mut stale_token = manifest()?;
    stale_token.package_revision = "1".to_owned();
    assert!(
        stale_token.validate().is_err(),
        "arbitrary package revision token was accepted"
    );

    let mut stale_members = manifest()?;
    let first = stale_members
        .members
        .first_mut()
        .ok_or("fixture did not contain a member")?;
    first.role = "localization/alternate".to_owned();
    assert!(
        stale_members.validate().is_err(),
        "changed canonical members retained a stale package revision"
    );
    Ok(())
}

#[test]
fn content_revision_frames_variable_member_metadata()
-> Result<(), Box<dyn std::error::Error>> {
    let first =
        member_from_bytes("content/item.bin", "a", "bc", b"same bytes")?;
    let second =
        member_from_bytes("content/item.bin", "ab", "c", b"same bytes")?;
    let first_revision = content_revision(&[first])?;
    let second_revision = content_revision(&[second])?;
    assert_ne!(
        first_revision, second_revision,
        "metadata field boundaries collided in the content revision"
    );
    Ok(())
}

fn package_with_identity(
    canonical_id: &str,
    dependencies: Vec<Dependency>,
) -> Result<PackageManifest, Box<dyn std::error::Error>> {
    let mut package = manifest()?;
    package.canonical_id = canonical_id.to_owned();
    package.dependencies = dependencies;
    Ok(package)
}

#[test]
fn dependency_order_is_exact_and_discovery_independent()
-> Result<(), Box<dyn std::error::Error>> {
    let core = package_with_identity("shar.core", Vec::new())?;
    let core_revision = core.package_revision.clone();
    let independent = package_with_identity("shar.independent", Vec::new())?;
    let feature = package_with_identity("shar.feature", vec![Dependency {
        canonical_id: core.canonical_id.clone(),
        revision: core_revision.clone(),
    }])?;
    let expected = vec![
        "shar.core".to_owned(),
        "shar.feature".to_owned(),
        "shar.independent".to_owned(),
    ];
    assert_eq!(
        dependency_load_order(&[
            independent.clone(),
            feature.clone(),
            core.clone(),
        ])?,
        expected
    );
    assert_eq!(
        dependency_load_order(&[feature, core.clone(), independent])?,
        expected
    );

    let missing =
        package_with_identity("shar.missing-user", vec![Dependency {
            canonical_id: "shar.absent".to_owned(),
            revision: core_revision,
        }])?;
    assert!(dependency_load_order(&[missing]).is_err());

    let wrong_revision =
        package_with_identity("shar.wrong-user", vec![Dependency {
            canonical_id: "shar.core".to_owned(),
            revision: "wrong".to_owned(),
        }])?;
    assert!(dependency_load_order(&[core.clone(), wrong_revision]).is_err());
    assert!(dependency_load_order(&[core.clone(), core]).is_err());
    Ok(())
}

#[test]
fn dependency_order_rejects_cycles() -> Result<(), Box<dyn std::error::Error>> {
    let seed = manifest()?;
    let revision = seed.package_revision;
    let first = package_with_identity("shar.first", vec![Dependency {
        canonical_id: "shar.second".to_owned(),
        revision: revision.clone(),
    }])?;
    let second = package_with_identity("shar.second", vec![Dependency {
        canonical_id: "shar.first".to_owned(),
        revision,
    }])?;
    assert!(dependency_load_order(&[first, second]).is_err());
    Ok(())
}

#[test]
fn native_packages_require_explicit_targets_and_trust()
-> Result<(), Box<dyn std::error::Error>> {
    let mut native = manifest()?;
    native.package_kind = PackageKind::Native;
    assert!(
        native.validate().is_err(),
        "native package inherited content-only trust"
    );

    native.trust_level = TrustLevel::NativeExplicit;
    assert!(
        native.validate().is_err(),
        "native package without target compatibility was accepted"
    );
    native.supported_targets = vec!["linux.x86_64".to_owned()];
    native.validate()?;
    Ok(())
}

#[test]
fn rejects_self_relationships_and_nondeterministic_lists()
-> Result<(), Box<dyn std::error::Error>> {
    let mut self_dependency = manifest()?;
    self_dependency.dependencies = vec![Dependency {
        canonical_id: self_dependency.canonical_id.clone(),
        revision: "1".to_owned(),
    }];
    assert!(
        self_dependency.validate().is_err(),
        "self dependency was accepted"
    );

    let mut duplicate_dependency = manifest()?;
    duplicate_dependency.dependencies = vec![
        Dependency {
            canonical_id: "shar.core.example".to_owned(),
            revision: "1".to_owned(),
        },
        Dependency {
            canonical_id: "shar.core.example".to_owned(),
            revision: "2".to_owned(),
        },
    ];
    assert!(
        duplicate_dependency.validate().is_err(),
        "multiple exact revisions of one dependency id were accepted"
    );

    let mut conflicting_dependency = manifest()?;
    conflicting_dependency.dependencies = vec![Dependency {
        canonical_id: "shar.core.example".to_owned(),
        revision: "1".to_owned(),
    }];
    conflicting_dependency.conflicts = vec!["shar.core.example".to_owned()];
    assert!(
        conflicting_dependency.validate().is_err(),
        "dependency was also accepted as a conflict"
    );

    let mut superseded_dependency = manifest()?;
    superseded_dependency.dependencies = vec![Dependency {
        canonical_id: "shar.core.example".to_owned(),
        revision: "1".to_owned(),
    }];
    superseded_dependency.supersedes = vec!["shar.core.example".to_owned()];
    assert!(
        superseded_dependency.validate().is_err(),
        "dependency was also accepted as superseded"
    );

    let mut unsorted = manifest()?;
    unsorted.required_capabilities = vec![
        "rendering.v1".to_owned(),
        "localization.overlay.v1".to_owned(),
    ];
    assert!(
        unsorted.validate().is_err(),
        "nondeterministic capability ordering was accepted"
    );
    Ok(())
}
