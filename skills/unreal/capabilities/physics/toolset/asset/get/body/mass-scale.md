# Get body mass scale

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PhysicsToolsets.PhysicsAssetToolset.GetBodyMassScale
```

Toolset:

```text
PhysicsToolsets.PhysicsAssetToolset
```

## What this tool does

Returns the mass-scale multiplier for the given body.

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
Use this tool to read the per-body mass multiplier during SHAR physics tuning or
parity review.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact loaded PhysicsAsset object path.
- Select a body name from `GetBodyNames`.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "physicsAsset": {"refPath": "/AnimatorKit/Meshes/PA_PhysCube.PA_PhysCube"},
  "boneName": "box"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned `1` for body `box`. A deliberately missing body name raised
`No body found for bone`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The value is a multiplier, not the final computed mass.
- Geometry, density, physical material, and component settings also affect
  runtime mass.
- Missing bodies raise a native error.
- Changing mass scale is a persistent asset mutation.
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
shar-unreal-mcp describe PhysicsToolsets.PhysicsAssetToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `boneName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the bone whose body to query.

### `physicsAsset`

- Required: **yes**
- Type: `object`
- Purpose:

The physics asset to query.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PhysicsToolsets.PhysicsAssetToolset \
  PhysicsToolsets.PhysicsAssetToolset.GetBodyMassScale \
  --arguments '
{
  "boneName": "<value>",
  "physicsAsset": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `number`
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
