# Unreal configuration and asset validation

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [C++-primary and Blueprint-compatible Unreal project](../../adr/unreal/project/cpp-primary-blueprint-compatible-project.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native art authoring, style, and asset validation contract](native-art-authoring-style-and-asset-validation-contract.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md)
- [Unreal gameplay content catalog](gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Unreal platform, quality, and optimization contract](platform-quality-and-optimization.md)

## Purpose

This specification defines how repository-owned validation inspects supported
project configuration, semantic content definitions, normalized manifests, and
native Unreal assets without parsing proprietary engine internals or treating
raw
source artifacts as product authority.

It covers:

- configuration schema and identity;
- normalized manifest validation;
- native Data Validation integration;
- asset and dependency rules;
- content-catalog integrity;
- platform and quality compatibility;
- commandlet execution;
- findings, waivers, reports, and failure behavior; and
- native read-back after import or mutation.

## Authority boundary

Repository-owned parsers read only public configuration and generated evidence
whose schema belongs to the repository. Native Unreal assets, packages,
components, worlds, physics state, rendering state, and editor internals remain
engine authority and are observed through supported APIs and read-back tools.

The validator does not:

- deserialize proprietary package internals independently;
- scan raw DCC scenes, images, P3D packages, audio, video, Office files, or
  caches
  as semantic ledger entries;
- infer gameplay identity from filenames or folder position;
- execute historical batch scripts or exporter configuration;
- replace native Data Validation; or
- repair assets silently during a validation run.

## Validation topology

Validation has four layers:

1. **Schema validation** — public configuration and manifest structure.
1. **Semantic validation** — stable identities, references, ownership, and
   domain
   invariants.
1. **Native asset validation** — Unreal asset class, metadata, dependencies, and
   project-specific native rules.
1. **Runtime acceptance validation** — read-back, platform cook, loading,
   construction, and behavior evidence.

A layer cannot waive a failure owned by another layer.

## Configuration identity

Every configuration document declares:

- schema identity and version;
- owning subsystem;
- configuration revision;
- target platform or platform-neutral scope;
- feature, world, chapter, local-player, or asset scope where applicable;
- dependency revisions;
- migration policy;
- validation ruleset revision; and
- deterministic serialization and digest policy.

Unknown schema versions, duplicate identities, conflicting ownership, ambiguous
aliases, or unversioned migrations fail closed.

## Manifest validation

Normalized manifests own counts and conversion evidence for raw assets. A
manifest declares:

- schema and generator revision;
- deterministic ordering;
- package and semantic identities;
- source and output hashes and sizes;
- normalized asset class;
- geometry, rig, animation, material, texture, collision, and dependency
  summaries where applicable;
- warnings and accepted repairs;
- completeness state; and
- terminal validation result.

The validator verifies internal counts, uniqueness, hashes, ordering, reference
closure, expected output existence, and agreement between package summaries and
entry rows.

Animation counts are deduplicated by stable animation-package identity rather
than summed once for every costume or character variant that embeds the same
animation family.

## Native Data Validation

The project extends Unreal Data Validation with C++ validators for product-owned
rules. Blueprint or Python validators may support editor workflows, but the
blocking continuous-integration path must remain deterministic and available to
the repository validation command.

Validation may run for:

- one asset;
- selected assets;
- assets and dependencies;
- one folder;
- one feature or content family;
- one platform or quality profile; or
- the complete project.

Continuous integration invokes native commandlet validation and repository-owned
checks through the canonical repository validator.

## Asset rules

Native asset validation covers at least:

- package and object naming;
- semantic identity and metadata;
- asset class;
- primary-asset type and bundle membership;
- owner feature, world, location, character, vehicle, or content family;
- hard and soft dependency policy;
- dependency cycles;
- editor-only and runtime-only boundaries;
- platform and quality availability;
- source contamination and private metadata;
- replacement, deprecation, and migration state; and
- deterministic read-back.

## Geometry and presentation rules

Geometry and visual validation covers:

- finite vertex, normal, tangent, UV, color, and transform data;
- topology and degeneracy policy;
- scale, axes, pivots, origin, hierarchy, and bounds;
- material-slot roles;
- texture dimensions, color space, compression, mip, streaming, and alpha roles;
- Skeleton, skinning, Physics Asset, sockets, animation, curves, and track
  compatibility;
- collision and physical materials;
- LOD, HLOD, Nanite, instancing, culling, and shadow policy;
- world, Data Layer, Level Instance, and streaming ownership;
- platform budgets and graphics profiles; and
- visual-style, palette, cel-shading, and fallback requirements.

Detailed authoring rules follow
<!-- markdownlint-disable-next-line MD013 -->
[Native art authoring, style, and asset validation contract](native-art-authoring-style-and-asset-validation-contract.md).

## Content-catalog rules

Catalog validation verifies:

- unique canonical identities and aliases;
- exact definition types;
- chapter, level, location, interior, world, feature, and progression ownership;
- character, vehicle, costume, reward, billboard, collector-card, gag,
  pedestrian,
  and placement references;
- mission, objective, route, dialogue, audio, animation, VFX, and UI references;
- required native asset bundles;
- lock, purchase, unlock, completion, and persistence policy;
- location and placement closure;
- production-only metadata exclusion; and
- deterministic generated output.

Approval, completion, assignment, milestone, date, and review columns from
private
production records are not runtime state.

## Platform-error and localization rules

Localized error definitions verify:

- one stable semantic error identity;
- localized text keys rather than embedded mutable strings;
- required cultures and prioritized fallback;
- placeholders and formatting arguments;
- severity, actions, retry, recovery, and terminal result;
- platform applicability;
- safe behavior when a translation is missing; and
- no dependence on source row order or platform-era numeric codes.

## Native read-back

After import or mutation, validation reads supported native state for:

- asset class and package;
- metadata and primary-asset identity;
- geometry, bounds, transforms, materials, textures, Skeleton, animation,
  collision, LODs, and dependencies;
- Data Layer, Level Instance, world, and streaming ownership;
- platform and quality settings;
- runtime-loadable bundles; and
- construction or registration results.

A successful tool call is transport evidence only. Acceptance requires native
read-back to match the complete plan within explicit tolerances.

## Findings

A validation finding records:

- stable rule identity and revision;
- affected configuration, definition, manifest, or native asset;
- severity;
- expected and observed normalized values;
- platform, quality, feature, world, and dependency scope;
- safe evidence;
- remediation guidance;
- waiver eligibility; and
- terminal disposition.

Blocking findings cannot be converted into warnings merely to make a validation
run green.

## Waivers

A waiver is allowed only when the governing rule permits one. It declares:

- exact finding and rule revision;
- bounded asset or configuration scope;
- reason;
- owner;
- compensating evidence;
- expiration or review condition; and
- platform and quality limits.

Waivers are prohibited for ambiguous identity, malformed schemas, private-data
leaks, source contamination, corrupt native state, unsafe dependencies, missing
gameplay-critical collision, or incompatible Skeleton and animation ownership.

## Determinism

The same repository revision, configuration, manifests, native asset revision,
validator rules, platform profile, and toolchain must produce the same
normalized
findings and terminal result.

Filesystem enumeration order, wall-clock time, locale, editor selection, current
Content Browser folder, or concurrent unrelated work cannot affect output.

## Failure behavior

Validation fails closed when:

- configuration or manifest structure is malformed;
- an identity is missing, duplicated, or ambiguous;
- an excluded raw artifact is reintroduced as semantic authority;
- production-only metadata enters runtime definitions;
- native assets or dependencies differ from the accepted plan;
- required platform or quality data is absent;
- read-back cannot prove the requested state;
- a blocking rule fails; or
- a waiver is invalid, expired, or out of scope.

Validation never partially publishes a new asset or catalog revision.

## Verification

Automated verification covers:

- valid, malformed, missing, duplicate, and unknown schemas;
- semantic identity and alias normalization;
- manifest counts, hashes, ordering, and reference closure;
- raw-artifact exclusion;
- art, geometry, rig, animation, texture, material, collision, LOD, and platform
  rules;
- catalog references and production-metadata exclusion;
- localized error keys, cultures, placeholders, and fallback;
- native Data Validation and commandlet invocation;
- read-back comparison;
- waiver boundaries; and
- deterministic reports.

## Invariants

- Only supported repository-owned configuration and manifests are parsed
  directly.
- Raw assets are counted from normalized manifests, not tracked as semantic
  per-file coverage.
- Native engine state remains authoritative for native objects and packages.
- Every blocking rule has one stable identity and deterministic result.
- Production administration never becomes runtime data.
- Validation does not mutate the asset under review.
- A successful command is not acceptance without native read-back.
- No lint, validation, privacy, identity, or dependency rule is weakened to make
  the gate pass.
