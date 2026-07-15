# Get emitter input values

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.GetEmitterInputValues
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Returns all resolved input values for every module across all four emitter
script stacks. One FNiagaraExt_ModuleInputValues entry per module, each
carrying all its resolved input values. Call this in parallel with
GetEmitterTopology to get both structure and values in two passes.

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
Use this tool to collect resolved input values for every module across all four
script stacks of one Niagara emitter in a single read.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain the exact emitter name from `GetSystemSummary`.
- Populate all StackItemReference fields; leave script, module, and input-name
  fields empty and use renderer index `-1`.
- Pair the result with `GetEmitterTopology` for stack ownership and schema.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "emitterRef": {
    "system": {"refPath": "/Niagara/VectorFields/VectorFieldVisualizationSystem.VectorFieldVisualizationSystem"},
    "emitterName": "VectorFieldParticleEmitter",
    "scriptName": "",
    "moduleName": "",
    "rendererIndex": -1,
    "inputNameStack": []
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two Particle reads returned ten stable module-value records: EmitterState,
SpawnRate, SphereLocation, one spawn assignment, SampleVectorField,
ApplyVectorField, one update assignment, UpdateAge, Color, and
SolveForcesAndVelocity. Bounding Box independently returned EmitterState,
SpawnBurst_Instantaneous, and ConstructBoundingBoxForVectorField001.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The result is one flat array spanning all four script stacks; stack ownership
  is not included in each record.
- Compare order with `GetEmitterTopology` rather than inferring a stack from
  module names.
- Values use heterogeneous instanced structs and can be linked, literal,
  dynamic, or data-interface payloads.
- Assignment-module names can contain generated GUID suffixes.
- Empty and missing emitter names fail explicitly.
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

### `emitterRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the emitter to retrieve input values from

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.GetEmitterInputValues \
  --arguments '
{
  "emitterRef": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

Array of per-module input value bundles across all four script stacks

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
