# Set capsule

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PhysicsToolsets.PhysicsAssetToolset.SetCapsule
```

Toolset:

```text
PhysicsToolsets.PhysicsAssetToolset
```

## What this tool does

Adds or replaces a capsule collision primitive on a body. If any shape with the
given name already exists on the body it is removed first. The capsule's long
axis is its local Z after applying Rotation.

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
Use this tool to create or update one named capsule shape on an existing
physics body.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  exact live toolset schema before mutation.
- Use only task-owned disposable assets and define whole-folder deletion
  before invocation.
- Duplicate the source skeletal mesh into the disposable folder and use body,
  shape, constraint, mass, and mode readers after mutation.
- Use a unique shape name and read the complete body-shape inventory.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "boneName": "Bone02",
  "center": {
    "x": 7,
    "y": 8,
    "z": 9
  },
  "length": 18,
  "physicsAsset": {
    "refPath": "/Game/SHAR_MCP_Validation_ControlRig/SK_MCP_Physics_PhysicsAsset.SK_MCP_Physics_PhysicsAsset"
  },
  "radius": 6,
  "rotation": {
    "pitch": 0,
    "roll": 0,
    "yaw": 45
  },
  "shapeName": "MCP_Capsule"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`GetBodyShapes` added named capsule `MCP_Capsule` with radius `6`, cylindrical
length `18`, and yaw `45`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Physics Asset mutations are persistent and destructive; use a duplicated
  skeletal mesh and exact body, shape, and constraint inventories.
- The duplicated mesh, generated Physics Asset, Control Rigs, and complete
  validation folder were deleted afterward.
- The setters use upsert semantics: a missing unique shape name creates that
  primitive, while an existing matching shape is updated.
- Capsule `length` is only the cylindrical section; total height is length
  plus twice the radius.
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
shar-unreal-mcp describe PhysicsToolsets.PhysicsAssetToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `boneName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the bone whose body to modify.

### `center`

- Required: **yes**
- Type: `object`
- Purpose:

Center of the capsule in bone-local space (cm).

### `length`

- Required: **yes**
- Type: `number`
- Purpose:

Length of the cylindrical section (cm). Must be non-negative. Total capsule
height = Length + 2 * Radius.

### `physicsAsset`

- Required: **yes**
- Type: `object`
- Purpose:

The physics asset to modify.

### `radius`

- Required: **yes**
- Type: `number`
- Purpose:

Radius of the capsule end-caps (cm). Must be greater than zero.

### `rotation`

- Required: **yes**
- Type: `object`
- Purpose:

Orientation of the capsule in bone-local space.

### `shapeName`

- Required: **yes**
- Type: `string`
- Purpose:

A name that uniquely identifies this shape on the body.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PhysicsToolsets.PhysicsAssetToolset \
  PhysicsToolsets.PhysicsAssetToolset.SetCapsule \
  --arguments '
{
  "boneName": "<value>",
  "center": {},
  "length": 0.0,
  "physicsAsset": {},
  "radius": 0.0,
  "rotation": {},
  "shapeName": "<value>"
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
