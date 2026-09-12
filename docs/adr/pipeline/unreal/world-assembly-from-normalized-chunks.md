# World assembly from normalized chunks

- Status: Accepted
- Decision date: 2026-07-12
- Last reviewed: 2026-08-18
- Scope: World reconstruction

## Context

World reconstruction must preserve observable structure without importing a
proprietary editor project or inventing a replacement terrain model.

## Decision

The Unreal world consumes normalized decoded geometry and source-authored
placement JSON directly through native construction plans. The pipeline
preserves source positions, transforms, pivots, UVs, materials, textures, and
package identity without inserting an interchange-format coordinate layer.
Source
owner and placement records govern independent, breakable, and interactable
roles.

Breakable classification
requires the exact decoded `srr_tree_dsg` or `srr_breakable_object` container
kind; a dynamic/static-physics name containing `tree` is not breakable
authority. Spatial proximity is not object-identity authority. Auxiliary
coordinate-reference
meshes may contribute
positions/normals only when package identity, owner kind/name, mesh identity,
and topology all match exactly. Owner-only/topology-only association is not
source authority, and ambiguous exact donors fail closed.

Narrative levels may share stable family labels for catalog, streaming, mission,
and progression purposes. Those labels are metadata only. They do not translate,
recenter, stitch, or otherwise move geometry.

World, race, road, prop, door, and interior native constructors use one typed
source-to-Unreal basis conversion. No export-root reflection or importer
front-axis
setting, actor transform, height offset, or UV correction may become a second
coordinate policy.

The original world is constructed from normalized decoded geometry. The base
port does not replace it with an Unreal Landscape or a manually sculpted terrain
surface.

Interior packages retain their source-authored coordinates. Stable interior
identity may combine equivalent package content and may separate Level 7
Halloween additions from the canonical base, but fusion cannot depend on a
reviewed movement matrix. Duplicate ownership compares source-space triangles
within a bounded decoding tolerance while retained triangles preserve original
presentation data.

Collision evidence remains separate from render-mesh construction. Doors,
mission
anchors, cameras, locators, lights, triggers, and other non-mesh records
preserve their decoded source coordinates and are not adjusted by a shared
registry.

## Consequences

- Normalized JSON is the canonical Unreal world-ingestion boundary.
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
- Per-file actor transforms that compensate for incorrect native output.
