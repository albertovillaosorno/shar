# World assembly from normalized chunks

- Status: Accepted
- Decision date: 2026-07-12
- Last reviewed: 2026-08-18
- Scope: World reconstruction

## Context

World reconstruction must preserve observable structure without importing a
proprietary editor project or inventing a replacement terrain model.

## Decision

The Unreal world consumes deterministic FBX packages generated from decoded
original geometry and source-authored placement evidence. The pipeline preserves
source positions, transforms, pivots, UVs, materials, textures, and package
identity. One decoded source mesh remains one FBX mesh regardless of the spatial
distance between disconnected components. Source owner and placement records
govern independent, breakable, and interactable roles; spatial proximity is not
object-identity authority. Auxiliary coordinate-reference meshes may contribute
positions/normals only when package identity, owner kind/name, mesh identity,
and topology all match exactly. Owner-only/topology-only association is not
source authority, and ambiguous exact donors fail closed.

Narrative levels may share stable family labels for catalog, streaming, mission,
and progression purposes. Those labels are metadata only. They do not translate,
recenter, stitch, or otherwise move geometry.

World, race, road, prop, door, and interior FBXs use one source-to-FBX
`ReflectX` export root. The root performs the explicit coordinate-basis
conversion exactly once. Package geometry receives no additional location,
rotation, scale, height, or UV correction.

The original world is imported as FBX geometry. The base port does not replace
it with an Unreal Landscape or a manually sculpted terrain surface.

Interior packages retain their source-authored coordinates. Stable interior
identity may combine equivalent package content and may separate Level 7
Halloween additions from the canonical base, but fusion cannot depend on a
reviewed movement matrix. Duplicate ownership compares source-space triangles
within a bounded decoding tolerance while retained triangles preserve original
presentation data.

Collision evidence remains separate from render FBX output. Doors, mission
anchors, cameras, locators, lights, triggers, and other non-mesh records
preserve their decoded source coordinates and are not adjusted by a shared
registry.

## Consequences

- Generated FBX is the canonical Unreal world-ingestion boundary.
- Source-authored coordinates remain inspectable and reproducible.
- No map offsets, interior movement matrices, fixed height raise, or UV mirror
  can hide conversion errors.
- Narrative grouping and World Partition streaming do not alter map geometry.
- Missing or contradictory source evidence fails before publication.

## Rejected alternatives

- Importing or copying a proprietary editor project.
- Rebuilding the map as an Unreal Landscape.
- Undocumented manual placement.
- Operator-reviewed affine corrections as production authority.
- Per-file actor transforms that compensate for incorrect FBX output.
