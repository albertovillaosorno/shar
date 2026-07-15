# Get stack input topology

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.GetStackInputTopology
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Returns stack input topology: name, type, visibility, editability. No value
payload. For the resolved value call GetStackInputData. For a dynamic-input
chain call GetDynamicInputChain.

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
Use this tool to inspect one exact Niagara stack input's type, visibility,
editability, dynamic state, and static-switch state without reading its value.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Discover the exact module and input leaf name from topology.
- Populate every StackItemReference field and preserve the complete
  `inputNameStack`.
- Keep the system, emitter, script, module, and input identities paired.
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
Two reads returned `Particles.Lifetime` as visible, editable, dynamic, non-
static-switch NiagaraFloat. The result independently confirmed the input leaf
selected from the Particle Spawn Set Variables module.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Topology carries no value payload or schema metadata.
- `inputNameStack` is an ordered path and can contain multiple names for nested
  dynamic inputs.
- Leaf names can include namespace prefixes such as `Particles.`.
- Input refs become stale after module replacement or structural edits.
- Missing input leaves fail with the full stack reference.
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

Reference to the stack input to describe.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.GetStackInputTopology \
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

Input topology with structural metadata only.

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
