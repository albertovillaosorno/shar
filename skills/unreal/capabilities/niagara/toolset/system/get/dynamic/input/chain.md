# Get dynamic input chain

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.GetDynamicInputChain
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Returns the full recursive chain for a dynamic input: topology metadata and
resolved values at every level. The starting input must have value mode
Dynamic; an error is surfaced otherwise. The schema expands one full level and
emits a typed recursion stub at deeper levels; the wire format recurses to
arbitrary depth matching the underlying chain.

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
Use this tool to expand one dynamic Niagara stack input recursively, combining
topology and resolved values for every child in the chain.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Discover the exact dynamic stack input from stack-input data.
- Populate every StackItemReference field and preserve the complete
  `inputNameStack`.
- Keep the system, emitter, script, module, and input identities paired.
- Confirm that the starting value mode is Dynamic.
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
Two reads expanded UniformRangedFloat for `Particles.Lifetime`. Minimum was
literal float `0.5`, Maximum was `1`, Randomness Mode was Simulation Defaults,
and hidden Random Seed and Fixed Random Seed values were `0`. The root remained
visible, editable, and NiagaraFloat.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The wire format recurses through `NiagaraExt_DynamicInputChain` instanced
  structs.
- Child visibility and editability must be respected independently.
- Hidden conditional children remain present in the returned chain.
- A non-dynamic starting input raises an explicit value-mode error.
- Static-switch conditional inputs are flattened into module topology rather
  than represented as a dynamic chain.
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

Reference to the dynamic input to traverse

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.GetDynamicInputChain \
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

Recursive chain of entries, each carrying topology metadata and a resolved
Value payload

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
