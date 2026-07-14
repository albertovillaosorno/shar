# Validated game-feature mod overlays

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Native projection of accepted local mod packages

## Context

Local packages already have deterministic identity, validation, dependency,
conflict, trust, preview, and atomic active-set contracts. Unreal still needs one
native projection for accepted content without reverting to loose file
replacement, editor-directory discovery, or package-specific gameplay branches.

Portable structured data, target-cooked Unreal assets, and native executable code
have different compatibility and trust boundaries. Treating them as one load
path would either reject useful portable content or overstate the safety and
portability of executable extensions.

## Decision

Accepted local packages project into one repository-owned overlay model.

Portable data overlays register semantic rows and primary-asset metadata through
repository-owned catalog and registry adapters. They never replace base files or
mutate base assets.

Target-cooked asset overlays use mounted, revision-bound Game Feature content.
The overlay declares the exact Unreal version, target, architecture, cook
revision, primary assets, data-registry sources, World Partition content, and
Game Feature actions it needs. Activation remains subordinate to the validated
candidate active-set transaction.

Native executable packages are not loaded by this decision. They remain inactive
unless a separate accepted native-extension trust, ABI, signing, loading, and
rollback implementation exists for the exact target.

Legacy archives are transport inputs only. They are normalized and validated
into the canonical package model before any Unreal projection is considered.

## Consequences

- The canonical package declaration remains authority over identity and load
  order.
- Game Feature state is a native execution detail, not package identity.
- Portable data overlays may work across targets when every semantic capability
  resolves.
- Cooked asset overlays are target- and engine-build-specific.
- Runtime activation never scans arbitrary content folders or editor paths.
- Base assets remain immutable; overrides are deterministic overlay rows and
  references.
- Game Feature activation, catalog registration, Data Registry sources, and
  world content commit or roll back as one candidate revision.
- Removing a package restores the prior accepted overlay graph without deleting
  saved canonical identities.
- Native code never inherits trust or portability from a content-only package.

## Rejected alternatives

- Activating loose replacement files directly from a `Mods` directory.
- Treating archive order or mount order as semantic priority.
- Loading uncooked editor assets in a packaged runtime.
- Registering package content before dependency and conflict validation.
- Allowing Game Feature activation to bypass the package active-set transaction.
- Loading arbitrary native binaries because their metadata parsed successfully.
