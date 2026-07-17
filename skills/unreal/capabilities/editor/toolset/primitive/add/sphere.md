# Add sphere

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.primitive.PrimitiveTools.add_sphere
```

Toolset:

```text
editor_toolset.toolsets.primitive.PrimitiveTools
```

## What this tool does

Adds a sphere-shaped StaticMeshComponent to an actor.

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
Use this tool to add one sphere-shaped SHAR scene component for bounded radius,
trigger, collision-proxy, debug, or spatial-layout validation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must have the owner actor loaded and PIE stopped.
- Resolve the exact actor and capture its complete component list.
- Choose a unique component name and define exact removal through the returned
  component reference before mutation.
- Treat the supplied transform as actor-local and verify the resulting reflected
  component properties independently.
- Do not save the level for a disposable validation component.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "actor": {
    "refPath": "/Temp/Untitled_1.Untitled_1:PersistentLevel.PlayerStart_UAID_F02F74551BF5599B01_1153002503"
  },
  "name": "SHAR_MCP_ValidationSphere",
  "radius": 45,
  "local_transform": {
    "location": {"x": -100, "y": 0, "z": 0},
    "rotation": {"pitch": 0, "yaw": 40, "roll": 0},
    "scale": {"x": 1, "y": 1, "z": 1}
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned the exact named component reference. A separate component
list contained that reference. Reflected properties returned
`/Engine/BasicShapes/Sphere.Sphere`, local location approximately
`(-100, 0, 0)`, yaw approximately `40`, and uniform relative scale `0.9`.
Exact-reference removal returned `true`, and the actor's original four-component
list was restored exactly.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool adds a `StaticMeshComponent`; it does not create a separate actor.
- Primitive dimensions are represented through relative scale on an engine
  basic-shape mesh.
- Local transform scale can multiply the dimension-derived scale and should be
  used deliberately.
- Floating-point normalization is expected in reflected transforms.
- Attachment hierarchy, collision, materials, and mobility require separate
  verification when relevant.
- Component creation changes loaded level state and can persist if the level is
  saved.
- The returned component reference is the cleanup authority.
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
shar-unreal-mcp describe editor_toolset.toolsets.primitive.PrimitiveTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `actor`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `local_transform`

- Required: **no**
- Type: `object`
- Purpose:

Represents a 3D transformation with optional location, rotation, and scale.
Unset fields mean "identity" when creating objects and "don't change" when
modifying existing ones.

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the new component.

### `radius`

- Required: **no**
- Type: `number`
- Default: `50`
- Purpose:

The radius of the sphere.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.primitive.PrimitiveTools \
  editor_toolset.toolsets.primitive.PrimitiveTools.add_sphere \
  --arguments '
{
  "actor": {},
  "name": "<value>"
}
'
```

## Expected output

The new StaticMeshComponent.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

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
