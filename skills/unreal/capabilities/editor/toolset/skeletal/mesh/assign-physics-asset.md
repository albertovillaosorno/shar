# Assign physics asset

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.assign_physics_asset
```

Toolset:

```text
editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools
```

## What this tool does

Assigns a physics asset to a skeletal mesh.

The physics asset must be compatible with the mesh's skeleton. Use this to swap
physics assets or assign one to a mesh that has none.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Capture pre-state, bound the target set, and verify the resulting editor or
asset state through an independent read.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to assign a matching Physics Asset to one disposable
skeletal-mesh copy.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a task-owned skeletal-mesh copy and verify skeleton or Physics Asset
  compatibility before mutation.
- Capture the matching asset, class, existence, or assignment reader and
  define whole-folder cleanup.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "mesh": {
    "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/SK_MCP_Round50_Physics.SK_MCP_Round50_Physics"
  },
  "physics_asset": {
    "refPath": "/Engine/Tutorial/SubEditors/TutorialAssets/Character/TutorialTPP_PhysicsAsset.TutorialTPP_PhysicsAsset"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned true and `get_physics_asset` returned the exact TutorialTPP
Physics Asset on the disposable matching mesh.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Skeletal import and Physics Asset assignment are compatibility-sensitive; a
  true return alone is insufficient.
- Skeletal mesh, skeleton, Physics Asset, and imported UObject references
  become stale after cleanup.
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
shar-unreal-mcp describe editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `mesh`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `physics_asset`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools \
  editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.assign_physics_asset \
  --arguments '
{
  "mesh": {},
  "physics_asset": {}
}
'
```

## Expected output

True if the physics asset was assigned successfully.

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
