// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT

//! External contract tests for normalized SHAR mod manifests.

use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;
use unicode_normalization as _;

use shar_mod_package::{
    CONTRACT_VERSION, Dependency, PackageKind, PackageManifest, Provenance, TrustLevel,
    content_revision, member_from_bytes,
};

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
            source: "generated-from-user-supplied-lawful-original-game".to_owned(),
            license: "NOASSERTION".to_owned(),
        },
        trust_level: TrustLevel::ContentOnly,
    })
}

#[test]
fn content_manifest_round_trips_deterministically() -> Result<(), Box<dyn std::error::Error>> {
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
fn storage_location_is_not_package_identity() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = manifest()?;
    let text = manifest.to_pretty_json()?;
    assert!(
        !text.contains("/home/"),
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
fn rejects_unknown_fields_and_noncanonical_identity() -> Result<(), Box<dyn std::error::Error>> {
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
fn rejects_path_aliases_collisions_and_traversal() -> Result<(), Box<dyn std::error::Error>> {
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
fn native_packages_require_explicit_targets_and_trust() -> Result<(), Box<dyn std::error::Error>> {
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
fn rejects_self_relationships_and_nondeterministic_lists() -> Result<(), Box<dyn std::error::Error>>
{
    let mut self_dependency = manifest()?;
    self_dependency.dependencies = vec![Dependency {
        canonical_id: self_dependency.canonical_id.clone(),
        revision: "1".to_owned(),
    }];
    assert!(
        self_dependency.validate().is_err(),
        "self dependency was accepted"
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
