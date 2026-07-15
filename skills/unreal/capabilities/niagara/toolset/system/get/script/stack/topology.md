# Get script stack topology

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.GetScriptStackTopology
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Returns script stack topology: all modules and their inputs in execution order.
All fields always populated.

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
Use this tool to inspect every module in one Niagara script stack in execution
order, including each module's enabled state and input topology.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain a valid script usage name from emitter topology.
- Populate every StackItemReference field; leave module and input-name fields
  empty and use renderer index `-1` for a script-level reference.
- Keep the system, emitter, and script identities paired.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "scriptRef": {
    "system": {"refPath": "/Niagara/VectorFields/VectorFieldVisualizationSystem.VectorFieldVisualizationSystem"},
    "emitterName": "VectorFieldParticleEmitter",
    "scriptName": "ParticleUpdateScript",
    "moduleName": "",
    "rendererIndex": -1,
    "inputNameStack": []
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two Particle Update reads returned six enabled modules in stable order:
SampleVectorField, ApplyVectorField, one Set Variables assignment, UpdateAge,
Color, and SolveForcesAndVelocity. Their input counts were 10, 4, 1, 5, 3, and
17. Emitter Spawn independently returned a valid empty module array.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Module order is execution-significant and should be preserved.
- Empty stacks are valid and return a script object with an empty module array.
- Topology describes inputs but carries no configured values.
- Assignment-module names can contain generated GUID suffixes.
- Empty script names and invalid script-usage names fail differently.
- Script refs become stale after structural stack edits.
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

### `scriptRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the script stack to describe.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.GetScriptStackTopology \
  --arguments '
{
  "scriptRef": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Script stack topology with all modules and inputs fully walked.

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
