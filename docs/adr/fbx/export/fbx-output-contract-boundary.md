# First-principles FBX output contract

- Status: Accepted
- Decision date: 2026-07-12
- Last reviewed: 2026-08-03
- Scope: Canonical model interchange output

## Context

The project needs deterministic interchange artifacts for Unreal ingestion
without manual repair or an external content-authoring application.

Past experiments added heuristic UV mirrors, map placement matrices, interior
movement matrices, and a global world-height raise. Those changes were not
owned by original source evidence and could conceal decoder or writer defects.

## Decision

Canonical model output is binary FBX 7.7 generated directly by the
repository-owned writer from validated source packages. Blender and Maya are not
used for generation, conversion, repair, validation, or acceptance.

The writer preserves source topology, positions, normals, UV coordinates,
vertex colors, materials, texture identities, pivots, rigs, animation data, and
source-authored transforms. It must not apply name-based UV mirroring, artistic
repositioning, map offsets, interior offsets, global height adjustments, or
other inferred corrections.

The only world-wide orientation operation is the declared FBX export-root basis
conversion. World FBXs use one `SHAR_Export_Root` with `ReflectX` so Unreal can
interpret the original coordinate basis consistently. Geometry remains
unchanged below that root, and import actors use identity location, rotation,
and scale.

ASCII FBX and authoring-file formats are not canonical outputs. Unsupported
source capabilities fail explicitly instead of being approximated silently.

### Editor-only structural-guide profile

The structural guide is an optional FBX 7.7 view of the same source-authored
world geometry. It combines normal-import world FBXs under the same `ReflectX`
root, preserves source positions and UVs, excludes isolated review galleries,
and adds no guide-only placement or height policy.

Its atlas is editor inspection evidence, not runtime material authority. The
guide may approximate presentation only where its manifest records the exact
limitation. It cannot become terrain, collision, gameplay, navigation, or
shipping-render authority.

## Consequences

- The repository owns serialization correctness.
- Source evidence, not manual editing, determines FBX content.
- Unreal receives the original map as FBX rather than a replacement Landscape.
- UV and spatial differences expose decoder or writer defects instead of being
  hidden by heuristic corrections.
- External content-authoring applications are not pipeline dependencies.

## Rejected alternatives

- Exporting through Blender or Maya.
- Name-based UV correction.
- Reviewed per-package map or interior movement matrices.
- A fixed global world-height adjustment.
- Rebuilding the original world as an Unreal Landscape.
