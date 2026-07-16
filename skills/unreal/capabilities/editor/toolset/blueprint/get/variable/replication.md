# Get variable replication

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.get_variable_replication
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Gets the replication mode of a Blueprint member variable.

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
Use this tool to verify a Blueprint member variable's replication mode before
SHAR evaluates networking or RepNotify behavior.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact Blueprint ref and an existing member variable.
- Interpret the result as one of `None`, `Replicated`, or `RepNotify`.
- Independently inspect generated OnRep functions when RepNotify is expected.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "blueprint": {"refPath": "/Game/B_SHAR_MCP_Blueprint_ReadFixture2.B_SHAR_MCP_Blueprint_ReadFixture2"},
  "variable_name": "SHARReadVariable"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Across two disposable cycles, a new Boolean variable returned `None`. After
setting `RepNotify`, two reads per cycle returned `RepNotify`; compilation
succeeded and `list_functions` independently showed implemented function
`OnRep_SHARReadVariable`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The read does not prove actor replication, authority flow, lifetime
  conditions, or network correctness.
- Setting RepNotify can create an OnRep function, but this read only reports the
  variable mode.
- Missing variables raise an explicit variable-not-found error.
- Error messages can contain session-specific object addresses.
- Recompile and inspect the generated function independently after replication
  mutations.
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
shar-unreal-mcp describe editor_toolset.toolsets.blueprint.BlueprintTools
```

1. Confirm every required input against the current schema.

## Inputs

### `blueprint`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `variable_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the member variable to query.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.get_variable_replication \
  --arguments '
{
  "blueprint": {},
  "variable_name": "<value>"
}
'
```

## Expected output

The replication mode (NONE, REPLICATED, or REP_NOTIFY).

### `returnValue`

- Required: **yes**
- Type: `string`
- Allowed values:

  - `"None"`
  - `"Replicated"`
  - `"RepNotify"`
- Purpose:

EBlueprintVariableReplication

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
