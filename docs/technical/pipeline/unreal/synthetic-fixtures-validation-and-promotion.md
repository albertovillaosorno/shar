# Synthetic fixtures, validation, and promotion

- Status: Active
- Last reviewed: 2026-07-18

## Fixture root

Tiny repository-owned normalized inputs used to test Unreal contracts live only
under:

```text
tests/fixtures/unreal/
```

Global ignore rules continue to reject extracted and generated media.
`.gitignore` contains narrow exceptions for this exact fixture root because
these files are independently authored, minimal, redistributable, deterministic,
and required to prove the importer without private game assets.

## Initial fixture families

The first fixture set contains:

- one synthetic triangle mesh in canonical binary FBX 7.7;
- one tiny lossless base-color PNG;
- one deterministic `unreal-import-plan.json`;
- one expected native read-back JSON contract;
- no copyrighted character, vehicle, world, audio, name, logo, or texture.

The triangle is an import-transport fixture, not a valid shipping character.
Native character-definition automation tests use transient definitions and soft
paths so they can validate the C++ contract without checking generated UAssets
into Git.

Later fixture sets add a low-poly humanoid rig, a simple four-wheel vehicle, one
modular world tile, one mission, one sound, and one UI asset. Every fixture
remains intentionally small and tests a specific contract rather than visual
quality.

## Generated native test content

Editor automation imports fixture inputs into:

```text
/Game/SHAR/Tests/Generated
/Game/SHAR/Maps/Tests
```

Generated test `.uasset` and `.umap` files remain ignored. A clean editor test
deletes the generated root, reimports, reads back native state, runs Data
Validation and Automation tests, and compares logical results. This proves
fixture reproducibility without committing machine-generated test packages.
The separately governed canonical authored open world is not a test fixture and
is published through its narrow Git LFS exception.

## Validation layers

A package is promoted only after:

1. JSON schema, canonical identity, path, hash, and dependency validation;
1. source media validation for geometry, texture, audio, media, or data;
1. deterministic repeated plan generation;
1. staged Unreal import;
1. native class and property read-back;
1. Data Validation for every generated Primary and secondary asset;
1. family-specific structural tests;
1. visual or audio evidence where semantics cannot be proven structurally;
1. Asset Manager registration, bundle, and cook-rule verification;
1. clean-project repeatability;
1. rollback verification from an interrupted or invalid transaction.

## Character fixture acceptance

The initial character contract proves:

- stable `SharCharacter` and `SharCharacterPresentation` identities;
- lowercase canonical identifier validation;
- mesh, Skeleton, Physics Asset, material, texture, and animation-class soft
  references occupy the declared bundles;
- invalid identity, missing presentation, duplicate alias, missing source
  package, empty revision, or invalid schema version fails before publication;
- no validation step synchronously loads arbitrary dependencies;
- clean reimport produces the same object paths and read-back contract.

## Fixture publication policy

A new binary fixture requires:

- an English comment in `.gitignore` explaining its exact scope and why it is
  safe;
- a documented deterministic generator or independently authored provenance;
- a SHA-256 digest in its import plan;
- a maximum size appropriate for the focused contract;
- canonical validation.

Broad exceptions such as `!tests/**`, `!**/*.fbx`, or `!**/*.png` are forbidden.
A fixture containing extracted, private, branded, or third-party content is
rejected regardless of size.

## Promotion failure behavior

Any failed check keeps the transaction in staging or rolls it back. The result
names the package, artifact, target, invariant, observed value, expected value,
and corrective action. Partial success, manual repair, warning-only identity
drift, and stale read-back are not accepted.
