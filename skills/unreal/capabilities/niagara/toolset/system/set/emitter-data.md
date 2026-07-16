# Set emitter data

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.SetEmitterData
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Sets property values on a Niagara Emitter. Applies new values to emitter-level
properties based on the provided data structure.

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
Use this mutation to update schema-backed emitter properties such as local-space
simulation or deterministic random configuration for SHAR effects.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a live emitter reference from the target NiagaraSystem.
- Read current emitter data and schema-compatible field names first.
- Encode `emitterData.propertyValues` as JSON text.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "emitter": {
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_EmitterProbe_3."
        "NS_SHAR_MCP_EmitterProbe_3"
    )
},
    "emitterName": "SHARExtra",
    "scriptName": "",
    "moduleName": "",
    "rendererIndex": -1,
    "inputNameStack": [],
},
    "emitterData": {
        "propertyValues": '{"bLocalSpace":true,"RandomSeed":2468}'
    },
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
In two cycles, `SHARExtra` changed from local-space false and random seed 0 to
local-space true and random seed 2468 while remaining CPUSim and non-
deterministic.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This mutation returns JSON null.
- Property names follow emitter C++ casing, including `bLocalSpace` and
  `RandomSeed`.
- `propertyValues` is nested JSON text.
- Re-read `GetEmitterData` and parse its property-values blob to prove
  persistence.
- Rediscover emitter names after structural mutations.
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

### `emitter`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the emitter to modify

### `emitterData`

- Required: **yes**
- Type: `object`
- Purpose:

Data structure containing the property values to set

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.SetEmitterData \
  --arguments '
{
  "emitter": {},
  "emitterData": {}
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
