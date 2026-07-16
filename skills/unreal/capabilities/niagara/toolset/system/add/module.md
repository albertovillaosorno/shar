# Add module

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.AddModule
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Adds a module to a script stack. The module will be inserted into the specified
script's execution stack. Returns the module topology with all inputs walked
(no input values — call GetModuleInputValues for values).

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
Use this mutation to insert a Niagara module into a specific SHAR emitter script
stack before configuring forces, initialization, spawning, color, or utility
behavior.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a mutable system containing a full Niagara emitter.
- Use the exact script name returned by emitter topology.
- Select a NiagaraScript module compatible with that stack.
- Use a disposable system for validation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "moduleLocationRef": {
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_ModuleProbe_3."
        "NS_SHAR_MCP_ModuleProbe_3"
    )
},
    "emitterName": "SHARExtra",
    "scriptName": "ParticleUpdateScript",
    "moduleName": "",
    "rendererIndex": -1,
    "inputNameStack": [],
},
    "moduleAsset": {
        "refPath": (
            "/Niagara/Modules/Update/Forces/"
            "GravityForce.GravityForce"
        )
    },
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles appended `GravityForce` after `ParticleState` in
`SHARExtra.ParticleUpdateScript`. The returned topology was enabled and exposed
`Gravity` and `Coordinate Space` inputs.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent stack mutation and returns parsed module topology without
  input values.
- Use GetModuleInputValues for stored values.
- Lightweight inherited emitters can expose script names as `None` and reject
  normal module insertion.
- Wrong system and module asset types fail parameter translation.
- Rediscover module names after stack reconstruction.
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

### `moduleAsset`

- Required: **yes**
- Type: `object`
- Purpose:

The module script asset to add to the stack

### `moduleLocationRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference specifying where to add the module

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.AddModule \
  --arguments '
{
  "moduleAsset": {},
  "moduleLocationRef": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Topology of the newly added module with all inputs populated

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
