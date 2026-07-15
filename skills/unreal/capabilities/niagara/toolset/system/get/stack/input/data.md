# Get stack input data

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.GetStackInputData
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Returns the value of a stack module input. Retrieves the current value and
configuration for a specific module input parameter.

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
Use this tool to read the current resolved value mode and payload for one exact
Niagara stack input.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Discover the exact module and input leaf name from topology.
- Populate every StackItemReference field and preserve the complete
  `inputNameStack`.
- Keep the system, emitter, script, module, and input identities paired.
- Inspect the returned `struct` ref before parsing its value payload.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "stackInputRef": {
  "system": {"refPath": "/Niagara/VectorFields/VectorFieldVisualizationSystem.VectorFieldVisualizationSystem"},
  "emitterName": "VectorFieldParticleEmitter",
  "scriptName": "ParticleSpawnScript",
  "moduleName": "SetVariables_D1F5C3144B416D5266BDFEBA95F1C835",
  "rendererIndex": -1,
  "inputNameStack": ["Particles.Lifetime"]
}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two reads returned `NiagaraExt_StackInputData_DynamicInput` with dynamic asset
`/Niagara/DynamicInputs/UniformRange/UniformRangedFloat.UniformRangedFloat` for
`Particles.Lifetime`. This matched the aggregate module and emitter input-value
reads independently.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The result is an instanced value with heterogeneous payload shapes.
- Dynamic, linked, literal, enum, data-interface, and unsupported modes use
  different structs.
- A dynamic value identifies the asset but does not expand its child values.
- Use `GetDynamicInputChain` for recursive values.
- Missing input leaves fail explicitly.
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

### `stackInputRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the stack input to retrieve data from

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.GetStackInputData \
  --arguments '
{
  "stackInputRef": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

The current value and configuration of the input

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
