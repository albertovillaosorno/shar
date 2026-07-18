# Set socket transform

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.set_socket_transform
```

Toolset:

```text
editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools
```

## What this tool does

Sets the local transform of a socket relative to its parent bone.

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
Use this tool to set a task-owned skeletal-mesh socket transform after exact
socket creation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live SkeletalMeshTools schema.
- Use a task-owned duplicate of `/Engine/EngineMeshes/SkeletalCube`; confirm
  bones `Bone01` and `Bone02`, slot `MaterialSlot`, and the current socket
  inventory before mutation.
- Capture the matching socket, material, or bone reader and define
  whole-folder deletion before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "mesh": {
    "refPath": "/Game/SHAR_MCP_Validation_ControlRig_Large_260718/SK_MCP_Large_260718.SK_MCP_Large_260718"
  },
  "socket_name": "MCP_Socket",
  "transform": {
    "location": {
      "x": 1.0,
      "y": 2.0,
      "z": 3.0
    },
    "rotation": {
      "pitch": 10.0,
      "roll": 30.0,
      "yaw": 20.0
    },
    "scale": {
      "x": 1.1,
      "y": 1.2,
      "z": 1.3
    }
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call completed without error and `get_socket_transform` returned location
1/2/3, rotation 10/20/30, and scale 1.1/1.2/1.3.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Skeletal mesh, socket, and nested socket references become stale after asset
  deletion or whole-folder cleanup.
- Bone names and material-slot names are exact asset contracts; discover them
  from the target mesh before mutation.
- This validation used a disposable duplicate of the engine SkeletalCube and
  does not authorize changes to engine-owned assets.
- Rotation values showed normal floating-point noise; compare with tolerance.
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

### `socket_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the socket to modify.

### `transform`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a 3D transformation with optional location, rotation, and scale.
Unset fields mean "identity" when creating objects and "don't change" when
modifying existing ones.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools \
  editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.set_socket_transform \
  --arguments '
{
  "mesh": {},
  "socket_name": "<value>",
  "transform": {}
}
'
```

## Expected output

The live interface does not declare a structured output schema.

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
