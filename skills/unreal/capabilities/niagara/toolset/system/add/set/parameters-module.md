# Add set parameters module

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.AddSetParametersModule
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Adds a SetParameters module to a script stack. Unlike AddModule which requires
a script asset, a SetParameters module dynamically assigns values to named
parameters and generates its own internal script. Use this when you need to set
one or more particle/emitter/system parameters directly in the stack.

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
Use this mutation to create a Set Parameters assignment module in a SHAR Niagara
stack when named particle, emitter, or system parameters must be authored
directly.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a full mutable emitter with a real script-stack name.
- Choose unique parameter names and matching Niagara types.
- Supply at least one initial typed parameter entry.
- Use a disposable system for validation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "moduleLocationRef": {
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_SetParamsProbe_2."
        "NS_SHAR_MCP_SetParamsProbe_2"
    )
},
    "emitterName": "SHARExtra",
    "scriptName": "ParticleUpdateScript",
    "moduleName": "",
    "rendererIndex": -1,
    "inputNameStack": [],
},
    "parameters": [
        {
            "variable": {
                "name": "Particles.SHARValue",
                "type": {
                    "classStructOrEnum": {
                        "refPath": "/Script/Niagara.NiagaraFloat"
                    }
                },
            },
            "defaultValue": {
                "struct": {
                    "refPath": "/Script/Niagara.NiagaraFloat"
                },
                "value": {"value": 1.25},
            },
        }
    ],
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles appended one enabled Set Parameters module after `ParticleState`. Its
first input was `Particles.SHARValue` with float default `1.25`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent stack mutation.
- The generated module name follows `SetVariables_<32-hex GUID>` and changes
  when rebuilt.
- Rediscover the exact module name before later entry operations.
- The return is parsed module topology and marks `bIsSetParametersModule` true.
- Lightweight inherited emitters with script name `None` are unsuitable.
- A StaticMesh system fails NiagaraSystem type validation.
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

### `moduleLocationRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference specifying where to add the module

### `parameters`

- Required: **yes**
- Type: `array<object>`
- Purpose:

Array of parameter entries, each with a variable (name + type) and an optional
default value string

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.AddSetParametersModule \
  --arguments '
{
  "moduleLocationRef": {},
  "parameters": []
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Topology of the newly added module, including all its inputs

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
