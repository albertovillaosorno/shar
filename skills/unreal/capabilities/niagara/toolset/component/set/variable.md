# Set variable

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_Component.SetVariable
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_Component
```

## What this tool does

Sets the value of a user variable on the component. This overrides the default
value of a user-exposed parameter on a specific component instance.

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
Use this mutation to set a typed per-component Niagara user override for a SHAR
effect instance without modifying the system asset default.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Assign the intended NiagaraSystem first.
- Discover the exact user parameter name and Niagara type from the system.
- Keep the variable type and value struct identical.
- Read `instanceParameterOverrides` after writing.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "component": {
    "refPath": (
        "/Temp/Untitled_1.Untitled_1:PersistentLevel."
        "NiagaraActor_1.NiagaraComponent0"
    )
},
    "variable": {
        "name": "User.SHARSpeed",
        "type": {
            "classStructOrEnum": {
                "refPath": "/Script/Niagara.NiagaraFloat"
            }
        },
        "value": {
            "struct": {"refPath": "/Script/Niagara.NiagaraFloat"},
            "value": {"value": 8.5},
        },
    },
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles wrote float `8.5` as bytes `[0, 0, 8, 65]`, then replaced it with
`12.5` as `[0, 0, 72, 65]` while retaining one matching override.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This mutation returns JSON null and performs no schema-membership validation.
- Unknown names are accepted and create new raw overrides.
- The same name with another Niagara type is accepted as a second override.
- A mismatched value struct is also accepted and can overwrite bytes under the
  declared type; one float override received Int32 bytes `[7, 0, 0, 0]`.
- Validate name, type, and struct before calling, then independently inspect
  override bytes.
- A StaticMesh component argument fails NiagaraComponent type validation.
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
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_Component
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `component`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara component to set the variable on

### `variable`

- Required: **yes**
- Type: `object`
- Purpose:

The variable instance containing the name and new value to set

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_Component \
  NiagaraToolsets.NiagaraToolset_Component.SetVariable \
  --arguments '
{
  "component": {},
  "variable": {}
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
