# Add set parameter entry

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.AddSetParameterEntry
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Adds a single parameter to an existing SetParameters module. The module
referenced by ModuleRef must be a SetParameters (UNiagaraNodeAssignment)
module. Use bIsSetParametersModule in the module topology to confirm before
calling.

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
Use this mutation to append a typed assignment to an existing SHAR Niagara Set
Parameters module without rebuilding the module.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Rediscover the transient Set Parameters module name from fresh topology.
- Choose a parameter name not already present.
- Keep variable type and default-value struct identical.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "moduleRef": {
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_SetParamsProbe_2."
        "NS_SHAR_MCP_SetParamsProbe_2"
    )
},
    "emitterName": "SHARExtra",
    "scriptName": "ParticleUpdateScript",
    "moduleName": "SetVariables_<rediscovered GUID>",
    "rendererIndex": -1,
    "inputNameStack": [],
},
    "entry": {
    "variable": {
        "name": "Particles.SHARExtra",
        "type": {
            "classStructOrEnum": {
                "refPath": "/Script/Niagara.NiagaraFloat"
            }
        },
    },
    "defaultValue": {
        "struct": {"refPath": "/Script/Niagara.NiagaraFloat"},
        "value": {"value": 2.5},
    },
},
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles appended `Particles.SHARExtra` with float value `2.5`, preserving
`Particles.SHARValue` first and returning topology with both entries.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent module mutation.
- The return is updated module topology, not only the new entry.
- Adding an existing parameter name raises `already exists` and instructs
  removal first.
- Duplicate failure preserved all prior values; it does not replace defaults.
- Module GUID names must never be persisted between rebuilt fixtures.
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

### `entry`

- Required: **yes**
- Type: `object`
- Purpose:

The parameter to add (variable name, type, and optional default value string)

### `moduleRef`

- Required: **yes**
- Type: `object`
- Purpose:

Reference to the existing SetParameters module

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.AddSetParameterEntry \
  --arguments '
{
  "entry": {},
  "moduleRef": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Updated topology of the module after the parameter is added

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
