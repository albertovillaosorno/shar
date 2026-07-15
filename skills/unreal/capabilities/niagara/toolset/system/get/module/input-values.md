# Get module input values

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.GetModuleInputValues
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Returns resolved input values for a single module. Use when you need values for
one specific module without walking the whole emitter.

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
Use this tool to read every resolved value for one Niagara module instance after
topology and schema have established its exact input identities and types.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain the exact script and module names from `GetEmitterTopology`.
- Populate every StackItemReference field; use renderer index `-1` and an empty
  input-name stack for a top-level module.
- Keep system, emitter, script, and module identities paired.
- Interpret each instanced value through its `struct` ref before reading the
  value payload.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "moduleRef": {
  "system": {"refPath": "/Niagara/VectorFields/VectorFieldVisualizationSystem.VectorFieldVisualizationSystem"},
  "emitterName": "VectorFieldParticleEmitter",
  "scriptName": "ParticleUpdateScript",
  "moduleName": "SampleVectorField",
  "rendererIndex": -1,
  "inputNameStack": []
}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two reads returned ten stable values. VectorField and seven transform/falloff
inputs linked to User parameters; SamplePoint linked to Particles.Position;
Sampled Vector Scale was a literal NiagaraFloat value `1`. Linked values
retained their variable names and reflected Niagara types.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Returned items contain `name` and an instanced `value`; the type is encoded by
  `value.struct` rather than a sibling type field.
- Linked inputs use `NiagaraExt_StackInputData_Linked` and require reading the
  nested linked variable.
- Literal and linked values have different payload shapes.
- This read does not include schema metadata or visibility.
- Missing module refs fail explicitly.
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

### `moduleRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the module

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.GetModuleInputValues \
  --arguments '
{
  "moduleRef": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Input value bundle for the module

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
