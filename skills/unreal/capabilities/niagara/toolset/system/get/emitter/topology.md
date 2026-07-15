# Get emitter topology

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.GetEmitterTopology
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Returns full emitter topology: four script stacks with all modules and inputs,
renderer references. All fields always populated. The returned topology carries
no input values; call GetEmitterInputValues in parallel. Note: mutation
endpoints (AddEmitter, AddModule) also return topology structs — input values
require a separate data call.

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
Use this tool to inspect one Niagara emitter's four script stacks, ordered
modules and input descriptors, simulation target, and renderer-instance
references.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain the exact emitter name from `GetSystemSummary`.
- Populate every StackItemReference field; use empty script/module/input values
  and renderer index `-1` for an emitter-level reference.
- Keep the system ref and emitter name paired.
- Call input-value tools separately when actual configured values are required.
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
Two Particle calls returned empty Emitter Spawn, two Emitter Update modules, two
Particle Spawn modules, six Particle Update modules, and one sprite renderer at
index `0`. Bounding Box returned two Emitter Update modules, one Particle Update
module, and one mesh renderer. Script and module ordering was stable.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Topology includes input descriptors but intentionally carries no input values.
- All four script-stack objects are present even when their module arrays are
  empty.
- Assignment-module names can contain generated GUID suffixes.
- Renderer refs use an emitter-local index.
- Module names and generated refs can change after graph edits or duplication.
- Missing emitter names fail explicitly.
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

Reference to the emitter to describe.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.GetEmitterTopology \
  --arguments '
{
  "emitterRef": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Emitter topology with all four script stacks and renderer references fully
walked.

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
