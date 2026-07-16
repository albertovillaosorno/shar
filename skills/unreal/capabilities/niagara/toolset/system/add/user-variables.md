# Add user variables

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_System.AddUserVariables
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_System
```

## What this tool does

Adds or updates user variables on a system. If a variable with the same name
already exists, it will be replaced with the new definition.

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
Use this mutation to add or replace typed Niagara user parameters needed by SHAR
effects, gameplay bindings, or component overrides.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a disposable or intentionally mutable NiagaraSystem.
- Use matching Niagara type references in `type` and `defaultValue.struct`.
- Include the `User.` namespace in the variable name.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "system": {
    "refPath": (
        "/Game/NS_SHAR_MCP_MutationProbe_2."
        "NS_SHAR_MCP_MutationProbe_2"
    )
},
    "variablesToAdd": [
        {
            "name": "User.SHARSpeed",
            "type": {
                "classStructOrEnum": {
                    "refPath": "/Script/Niagara.NiagaraFloat"
                }
            },
            "defaultValue": {
                "struct": {
                    "refPath": "/Script/Niagara.NiagaraFloat"
                },
                "value": {"value": 12.5},
            },
        }
    ],
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two systems added `User.SHARSpeed` with default `12.5`. Re-adding the same name
and type with `24.75` replaced the default and retained exactly one variable.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This mutation returns JSON null.
- Re-adding the same name and type replaces the definition rather than
  appending.
- Type and default-value struct references must agree.
- Verify through `GetSystemSummary`; the mutation result contains no details.
- A StaticMesh system argument fails type validation.
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

### `system`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara System to add variables to

### `variablesToAdd`

- Required: **yes**
- Type: `array<object>`
- Purpose:

Array of user variables to add or update

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_System \
  NiagaraToolsets.NiagaraToolset_System.AddUserVariables \
  --arguments '
{
  "system": {},
  "variablesToAdd": []
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
