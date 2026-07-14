# Get lod thresholds

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.static_mesh.StaticMeshTools.get_lod_thresholds
```

Toolset:

```text
editor_toolset.toolsets.static_mesh.StaticMeshTools
```

## What this tool does

Returns the screen-size thresholds at which each LOD becomes active.

Screen size is a ratio of the mesh's screen height to the viewport height. A
value of 1.0 means the mesh fills the full viewport height; values above 1.0
are valid and mean the mesh must appear larger than the viewport before the
next LOD activates. Each LOD activates when the mesh appears smaller than its
threshold.

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
Use this tool to inspect the current screen-size transition values before SHAR
compares or changes static-mesh LOD policy.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact loaded `UStaticMesh` object path.
- Read `get_lod_count` in the same editor state.
- Preserve returned order by LOD index.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "mesh": {
    "refPath": "/Engine/BasicShapes/Cube.Cube"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned `[2]`. The single value matched the independently verified
one-LOD inventory for the engine cube.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Return values are the live engine screen-size settings; do not assume they are
  restricted to zero through one.
- Ordering is significant and corresponds to LOD indices.
- Auto-compute or later LOD edits can change the values.
- Thresholds do not report geometry complexity.
- Missing mesh refs raise a native parameter error.
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
  editor_toolset.toolsets.static_mesh.StaticMeshTools.get_lod_thresholds \
  --arguments '
{
  "mesh": {}
}
'
```

## Expected output

A list of screen-size threshold values, one per LOD.

### `returnValue`

- Required: **yes**
- Type: `array<number>`
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
