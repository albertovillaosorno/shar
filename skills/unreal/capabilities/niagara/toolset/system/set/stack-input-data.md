# Set stack input data

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.SetStackInputData
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Sets the value of a stack module input and returns the resulting stored value.
Updates the value and configuration for a specific module input parameter.

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
Use this mutation to set a typed Niagara module input for SHAR effects,
including local values, linked parameters, enums, dynamic inputs, or supported
expressions.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a live stack-input reference with the complete input-name chain.
- Discover the input type through module topology or GetModuleInputValues.
- Use a matching instanced-value struct and payload.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "stackInputRef": {
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
    "inputNameStack": ["Gravity"],
},
    "inputData": {
        "struct": {"refPath": "/Script/CoreUObject.Vector3f"},
        "value": {"x": 1.5, "y": -2.25, "z": 3.75},
    },
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles changed `GravityForce.Gravity` from `(0, 0, -980)` to `(1.5, -2.25,
3.75)`. The return value matched the stored typed Vector3f value.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Unlike many Niagara mutations, this tool returns the resulting stored value.
- The `struct` must match the input type exactly.
- Dynamic-input chains require every nested name in `inputNameStack`.
- Re-read GetModuleInputValues to confirm persistence.
- Stale module or input names raise rather than creating a new input.
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

### `inputData`

- Required: **yes**
- Type: `object`
- Purpose:

The new value and configuration to apply to the input

### `stackInputRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the stack input to modify

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.SetStackInputData \
  --arguments '
{
  "inputData": {},
  "stackInputRef": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

The value now stored on the input after the operation

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
