# Generate convex collisions

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.static_mesh.StaticMeshTools.generate_convex_collisions
```

Toolset:

```text
editor_toolset.toolsets.static_mesh.StaticMeshTools
```

## What this tool does

Generates convex hull collision shapes for a static mesh.

Convex hulls provide accurate collision for physics simulation. More hulls
improve accuracy but increase runtime cost. Replaces any existing collision.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to generate bounded convex collision geometry for a reviewed
SHAR static mesh.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live StaticMeshTools schema.
- Use a task-owned duplicate of `/Engine/BasicShapes/Cube` and remove existing
  collision primitives before measuring generation.
- Read the exact BodySetup aggregate geometry before and after, then delete
  the complete disposable folder.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "hull_count": 2,
  "hull_precision": 10000,
  "max_hull_verts": 8,
  "mesh": {
    "refPath": "/Game/SHAR_MCP_Validation_ControlRig_Large_260718/SM_MCP_Large_260718.SM_MCP_Large_260718"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
After reviewed collision removal, reflected `BodySetup.aggGeom` contained zero
convex elements. The call returned true and the same reader reported one
convex element and zero box elements.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Static-mesh and BodySetup references become stale after asset deletion or
  whole-folder cleanup.
- Collision generation can be computationally expensive on production meshes;
  keep hull count, vertex limits, and precision bounded.
- `hull_count` is a generation request, not a guaranteed output count:
  requesting two hulls on the cube produced one convex element.
- Inspect the reflected BodySetup aggregate geometry rather than relying only
  on the Boolean return.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

<!-- markdownlint-disable-next-line MD013 -->
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `hull_count`

- Required: **no**
- Type: `integer`
- Default: `4`
- Purpose:

The number of convex hulls to generate.

### `hull_precision`

- Required: **no**
- Type: `integer`
- Default: `100000`
- Purpose:

Controls the voxel precision of the decomposition.

### `max_hull_verts`

- Required: **no**
- Type: `integer`
- Default: `16`
- Purpose:

The maximum number of vertices per hull.

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
  editor_toolset.toolsets.static_mesh.StaticMeshTools.generate_convex_collisions \
  --arguments '
{
  "mesh": {}
}
'
```

## Expected output

True if convex collision was generated successfully.

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Verify changed state through a separate read or inspection.
- Use another capability to confirm the postcondition.
- Inspect editor logs when state is not directly observable.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
