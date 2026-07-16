# Set renderer data

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.SetRendererData
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Sets property values on a Niagara Renderer. Applies new values to renderer
properties based on the provided data structure. Payload shape varies with the
concrete renderer class; call GetRendererSchema for the renderer's class to
inspect valid properties before writing.

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
Use this mutation to configure schema-backed Niagara renderer properties such as
enablement, shadow behavior, sorting, or camera-facing behavior for SHAR
effects.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a complete renderer stack reference, including system, emitter, and
  renderer index.
- Read the concrete renderer schema before choosing property names.
- Encode `rendererData.propertyValues` as JSON text.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "renderer": {
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_RendererProbe_3."
        "NS_SHAR_MCP_RendererProbe_3"
    )
},
    "emitterName": "SHARExtra",
    "scriptName": "",
    "moduleName": "",
    "rendererIndex": 1,
    "inputNameStack": [],
},
    "rendererData": {
        "propertyValues": '{"bIsEnabled":false,"bCastShadows":false}'
    },
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
In two cycles, the added sprite renderer changed `bIsEnabled` and `bCastShadows`
from true to false. GetRendererData confirmed both values after the mutation.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This mutation returns JSON null.
- `propertyValues` is nested JSON text.
- The partial object returned by AddRenderer lacks system and emitter identity.
- An unset optional `sortOrderHint` remained null even when supplied, so re-read
  every field instead of assuming persistence.
- Renderer indexes can change after structural edits.
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
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_System
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `renderer`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the renderer to modify

### `rendererData`

- Required: **yes**
- Type: `object`
- Purpose:

Data structure containing the property values to set

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.SetRendererData \
  --arguments '
{
  "renderer": {},
  "rendererData": {}
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
