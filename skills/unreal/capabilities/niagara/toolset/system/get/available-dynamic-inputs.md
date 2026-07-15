# Get available dynamic inputs

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.GetAvailableDynamicInputs
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Returns all available Dynamic Input Module assets compatible with the given
type. Dynamic inputs provide procedural value generation for module parameters.

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
Use this tool to discover indexed Niagara Dynamic Input assets compatible with
an exact value type before SHAR chooses procedural input generation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain the exact Niagara type definition from module or input schema.
- Preserve the `classStructOrEnum` ref exactly.
- Treat returned assets as candidates requiring separate schema inspection.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "type": {
    "classStructOrEnum": {
      "refPath": "/Script/Niagara.NiagaraFloat"
    }
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls per type returned stable inventories: 54 NiagaraFloat dynamic inputs,
43 Vector3f inputs, and 14 NiagaraBool inputs. Results included curves, random
ranges, arithmetic, masks, camera and position helpers, conversions,
interpolation, comparison, and scalability assets.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Compatibility is type-based and does not prove suitability for a specific
  module input or script usage.
- Results are asset refs without schemas or default values.
- Ordering reflects the live indexed catalog and should not be treated as
  semantic ranking.
- A valid but unsupported type such as StaticMesh returned `[]` rather than an
  error.
- Missing type-definition refs fail during parameter translation.
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
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_System
```

1. Confirm every required input against the current schema.

## Inputs

### `type`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara type definition to find compatible dynamic inputs for

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.GetAvailableDynamicInputs \
  --arguments '
{
  "type": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

Array of dynamic input script assets that can be used with the specified type

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
