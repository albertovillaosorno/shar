# Remove module

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.RemoveModule
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Removes a module from a script stack. Deletes the specified module and all its
inputs from the script's execution stack.

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
Use this mutation to remove an obsolete SHAR Niagara module and its input
configuration from a specific script stack after dependency review.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a live module reference with exact system, emitter, script, and module
  names.
- Confirm the target through fresh emitter topology.
- Preserve any input values needed for rollback before removal.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "moduleToRemove": {
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_ModuleProbe_3."
        "NS_SHAR_MCP_ModuleProbe_3"
    )
},
    "emitterName": "SHARExtra",
    "scriptName": "ParticleUpdateScript",
    "moduleName": "GravityForce",
    "rendererIndex": -1,
    "inputNameStack": [],
}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles removed `GravityForce` and restored the Particle Update stack to only
`ParticleState`. Each successful removal returned JSON null.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a destructive stack mutation and returns JSON null.
- Removing a module also deletes its configured inputs.
- Repeating the removal raises `Module ... not found` for the exact stack
  reference.
- Verify the final module order through GetEmitterTopology.
- Module names must be rediscovered after stack reconstruction.
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

### `moduleToRemove`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the module to remove from the stack

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.RemoveModule \
  --arguments '
{
  "moduleToRemove": {}
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
