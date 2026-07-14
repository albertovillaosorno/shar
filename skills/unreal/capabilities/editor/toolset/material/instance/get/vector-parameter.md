# Get vector parameter

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.material_instance.MaterialInstanceTools.get_vector_parameter
```

Toolset:

```text
editor_toolset.toolsets.material_instance.MaterialInstanceTools
```

## What this tool does

Gets the current value of a vector parameter on a material instance.

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
Use this tool to read the effective linear-color/vector value for an exact
material-instance parameter.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact MaterialInstanceConstant object path.
- Confirm the parameter type is `Vector` through `list_parameters`.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "instance": {"refPath": "/InterchangeAssets/gltf/MaterialInstances/MI_Default_Opaque.MI_Default_Opaque"},
  "name": "BaseColorFactor"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned `{r: 1, g: 1, b: 1, a: 1}` for `BaseColorFactor`. A missing
name raised a type-specific Vector-parameter error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The result uses linear-color channel names even when the parameter is
  conceptually a vector.
- The value may be inherited rather than locally overridden.
- Color-space interpretation remains the caller's responsibility.
- Missing instances fail during parameter translation.
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
shar-unreal-mcp describe editor_toolset.toolsets.material_instance.MaterialInstanceTools
```

1. Confirm every required input against the current schema.

## Inputs

### `instance`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

The parameter name.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.material_instance.MaterialInstanceTools \
  editor_toolset.toolsets.material_instance.MaterialInstanceTools.get_vector_parameter \
  --arguments '
{
  "instance": {},
  "name": "<value>"
}
'
```

## Expected output

The effective value as a LinearColor (RGBA), inheriting from the parent if not
overridden.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

LinearColor

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
