# Set module enabled

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.SetModuleEnabled
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Sets whether a module is enabled. Disabled modules remain in the stack but
don't execute. Current state is visible in module topology.

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
Use this mutation to disable or re-enable a SHAR Niagara module without removing
its position, inputs, or authored configuration from the stack.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a live module reference with exact system, emitter, script, and module
  names.
- Confirm the module currently exists in emitter topology.
- Re-read topology after the mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "moduleRef": {
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
},
    "bEnabled": False,
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
In two cycles, `GravityForce` changed from enabled to disabled and back to
enabled. Both successful mutation calls returned JSON null, while topology
reflected each state.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This mutation returns JSON null.
- Disabled modules remain in the stack and retain their inputs.
- Transport success is insufficient; verify the module `enabled` field in fresh
  topology.
- Script and module names are exact and case-sensitive identities.
- A removed or stale module reference raises explicitly.
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

### `bEnabled`

- Required: **yes**
- Type: `boolean`
- Purpose:

True to enable the module, false to disable it

### `moduleRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the module to enable or disable

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.SetModuleEnabled \
  --arguments '
{
  "bEnabled": false,
  "moduleRef": {}
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
