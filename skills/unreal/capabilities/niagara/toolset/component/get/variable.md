# Get variable

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_Component.GetVariable
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_Component
```

## What this tool does

Gets the current value of a specific user variable on the component. This
retrieves the current value of a user-exposed parameter, including any
component-level overrides.

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
Use this tool to read one Niagara Component user variable by exact type after
discovering its current definition through `GetUserVariables`.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain the component and variable type from `GetUserVariables`.
- Preserve `classStructOrEnum` exactly.
- Verify that the returned instanced-value struct matches the requested type.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "component": {"refPath": "/Game/B_SHAR_MCP_Niagara_Component_ReadFixture.B_SHAR_MCP_Niagara_Component_ReadFixture_C:Niagara_GEN_VARIABLE"},
  "var": {
    "name": "FieldIntensity",
    "type": {
      "classStructOrEnum": {"refPath": "/Script/Niagara.NiagaraFloat"}
    }
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two disposable cycles and two reads per cycle returned `FieldIntensity` as
NiagaraFloat value `1`. Both `FieldIntensity` and `User.FieldIntensity`
resolved. A missing name and a real name paired with NiagaraBool returned
successful variable shells whose `value` object was empty.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Name lookup accepts both prefixed and unprefixed user-variable names.
- Missing names and type mismatches do not raise errors; they return an empty
  `value` object.
- Treat an empty value as lookup failure and verify name plus type against
  `GetUserVariables` first.
- Returned values use an instanced struct whose `struct` ref determines the
  value shape.
- Component refs and component-owned data interfaces become stale after deletion
  or recompilation.
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

## Inputs

### `component`

- Required: **yes**
- Type: `object`
- Purpose:

The Niagara component to retrieve the variable from

### `var`

- Required: **yes**
- Type: `object`
- Purpose:

The variable definition (name and type) to look up

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_Component \
  NiagaraToolsets.NiagaraToolset_Component.GetVariable \
  --arguments '
{
  "component": {},
  "var": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

The variable instance containing the current value

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
