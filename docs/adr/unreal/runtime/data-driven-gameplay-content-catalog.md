# Data-driven Unreal gameplay content catalog

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Runtime gameplay identity and authored content

## Context

The conversion pipeline already produces deterministic package identities and
native asset plans. The runtime still needs a distinct game-domain catalog for
characters, vehicles, missions, locations, rewards, costumes, dialogue events,
and bonus modes. Several domain entities have alternate names, while missions
contain ordered objective chains and conditional asset requirements. Hardcoded
C++ names, Blueprint-owned registries, or filesystem discovery would duplicate
identity policy and make imports, saves, mods, cooking, and tests disagree.

Unreal provides primary asset identities, asynchronous Asset Manager loading,
soft references, asset bundles, and typed data tables. The project needs one
fixed composition of those native mechanisms rather than multiple equivalent
patterns.

## Decision

Every canonical top-level gameplay entity is represented by one non-Blueprint
primary data asset with a stable primary asset type and name. The asset records
its canonical domain identity, deterministic source-package references,
classification tags, display identity, and soft references to secondary native
assets.

Ordered or high-cardinality child records, including mission steps, quote-event
bindings, costume offers, race checkpoints, and tuning samples, are stored in
generated data tables whose row structures are owned by C++. Primary data
assets reference those tables through soft references and declare asset bundles
for the runtime states that load them.

Alternate public names are explicit alias rows that resolve to one canonical
identity. An alias never creates a second primary asset, save key, progression
key, or mod target. Asset Manager redirects are reserved for migrations of
previously published primary asset identities; they are not the domain alias
model.

Gameplay tags classify capabilities, roles, availability, and queryable state,
but never replace canonical identity. Runtime systems resolve content only
through the catalog service and primary asset identifiers. They do not scan
content directories, infer identity from filenames, or embed package paths.

## Consequences

- One entity has one runtime identity across import, cooking, saves, missions,
  mods, UI, and tests.
- Alternate names remain searchable without duplicating assets or progression.
- Primary assets can load secondary meshes, animation, audio, UI, and world
  references asynchronously through bounded bundles.
- Generated data tables preserve ordered child records without turning a single
  primary asset into an unreviewable monolith.
- Validation rejects duplicate canonical identities, alias collisions or
  cycles, unresolved soft references, invalid tags, missing required bundles,
  unordered mission steps, and source-package references that are not present
  in the approved native plan.
- Blueprint may consume the catalog through reflected read-only contracts, but
  it does not own identity, registration, validation, or load policy.

## Rejected alternatives

- One monolithic data table for every gameplay domain.
- One Blueprint class or Blueprint registry per entity.
- Raw strings, content paths, or filenames as runtime identity.
- Gameplay tags as identity rather than classification.
- Duplicate primary assets for aliases or alternate display names.
- Runtime filesystem discovery or a custom database that bypasses Unreal asset
  registration, cooking, and auditing.
