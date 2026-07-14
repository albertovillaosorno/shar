# Get vertex count

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.static_mesh.StaticMeshTools.get_vertex_count
```

Toolset:

```text
editor_toolset.toolsets.static_mesh.StaticMeshTools
```

## What this tool does

Returns the number of vertices in a specific LOD of a static mesh.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Use the returned structured evidence directly, but still confirm the live
schema because names do not prove side effects.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to measure render-vertex complexity for one validated static-mesh
LOD during SHAR import, optimization, or parity review.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact loaded `UStaticMesh` object path.
- Read `get_lod_count` first.
- Pass an index from zero through `count - 1`.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "mesh": {"refPath": "/Engine/BasicShapes/Cube.Cube"},
  "lod_index": 0
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two LOD 0 calls returned `54`. Indices `-1`, `1`, and `999` each raised an out-
of-range error stating that the mesh has one LOD.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Render vertices can exceed unique geometric corners because UV, normal,
  tangent, or material seams split vertices.
- The value is LOD-specific and does not describe collision geometry.
- Reimport or mesh-build settings can change the count.
- Missing mesh refs fail during parameter translation.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Current**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe editor_toolset.toolsets.static_mesh.StaticMeshTools
```

1. Confirm every required input against the current schema.

## Inputs

### `lod_index`

- Required: **no**
- Type: `integer`
- Default: `0`
- Purpose:

The LOD index to query. Defaults to 0 (highest quality).

### `mesh`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.static_mesh.StaticMeshTools \
  editor_toolset.toolsets.static_mesh.StaticMeshTools.get_vertex_count \
  --arguments '
{
  "mesh": {}
}
'
```

## Expected output

The number of vertices in the specified LOD.

### `returnValue`

- Required: **yes**
- Type: `integer`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Confirm the response belongs to the open editor project.
- Reject evidence derived from stale discovery state.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
