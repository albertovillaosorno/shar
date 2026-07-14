# FBX model normalization

This crate is the Phase 4 model-normalization boundary for FBX-capable assets.
It is intentionally hexagonal and package-index driven.

## Architectural rule

The package index is an intake contract, not a geometry model.

`ports/package_index.rs` defines the inbound port that resolves one stable
Phase 3 package id into package evidence: package id, model family, model member
ids, material member ids, texture member ids, and animation member ids. Implementations
of that port may read generated package-index JSONL, but they must only return
stable evidence. They must not translate meshes, decide FBX topology, read local
asset routes, write files, or choose Blender behavior.

Model translation begins after package evidence has already been resolved.
Domain modules own the pure translators: `domain/mesh/translator.rs` converts
resolved mesh evidence into geometry, `domain/material/translator.rs` converts
resolved material evidence, and the texture, skeleton, skin, animation, and
camera domain modules do the same for their own evidence. Application code
orchestrates these domain translators through ports; it does not contain a
parallel translator tree. This keeps props, vehicles, characters, and terrain on
one export engine instead of separate exporters.

Terrain and world pieces are package families, not FBX domain primitives. If a
package represents a terrain or world piece, the package-index adapter reports
that family and provides mesh-related member ids. The domain still sees scene,
node, geometry, surface layers, material, texture, skeleton, skin, animation,
camera, transform, coordinate policy, and capability reports.

## Layers

```text
src/fbx/src/domain/domain.rs
  Root domain facade.

src/fbx/src/domain/<concept>/<concept>.rs
  Concept facade only. Lists modules and re-exports the stable public surface.

src/fbx/src/domain/<concept>/*.rs
  Real domain value objects, invariants, and pure translators.

src/fbx/src/application/
  Use cases, package-family profiles, planning, and reporting. Application code
  orchestrates domain translators through ports; it does not own translation
  modules and does not depend on adapters.

src/fbx/src/ports/
  Inbound and outbound contracts: package index reader, component source,
  artifact target/sink, scene writer, validator.

src/fbx/src/adapters/driving/
  Inbound adapters such as CLI parsing. They translate user input into
  application requests.

src/fbx/src/adapters/driven/
  Outbound adapters such as generated package-index readers, decoded component
  sources, the canonical binary FBX 7.7 writer, and optional Blender or Maya
  import/review-script generators.
```

## Conversion flow

```text
Phase 3 package id
  -> PackageIndexReader port
  -> ModelPackageEvidence
  -> ModelExportPlan
  -> ComponentSource port
  -> domain mesh/material/texture/skeleton/skin/animation/camera translators
  -> domain Scene
  -> SceneWriter port
  -> driven writer adapter
```

## Character package status

The character package lane is complete. Canonical binary FBX 7.7 output has
been imported into Blender 5.1 and Maya 2027 with geometry, materials, embedded
textures, authored mesh partitions, skeleton hierarchy, skinning, native
animation curves, source-rate key timing, and animated poses preserved. The
writer does not forcibly fuse separate source meshes; Homer therefore remains a
valid three-mesh character artifact.

The review applications do not own scene semantics and do not emit alternate
production artifacts. Package evidence, domain translation, capability reports,
and the binary FBX writer remain authoritative.

The next delivery step is the full local character catalog under
`fbx-assets/characters/`, with one self-contained FBX per package and one
deterministic manifest. Phase 4 then proceeds through props, vehicles, and world
pieces in that order.

## Non-goals

FBX is only for model-like assets. Gameplay state, vehicle physics, world
streaming, mission logic, UI logic, and other non-model data must remain in
Unreal-native Phase 5 translators or companion reports. They must not be faked as
FBX content.
