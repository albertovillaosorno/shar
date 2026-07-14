# U enum info

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Review required**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
NiagaraToolsets.NiagaraToolset_Info.UEnum_Info
```

Toolset:

```text
NiagaraToolsets.NiagaraToolset_Info
```

## What this tool does

Returns information about a UEnum and all its values. ALWAYS call this when
working with a UEnum type to see valid values.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to enumerate the live values of a reflected Niagara enum before
SHAR constructs Niagara tool arguments, validates imported effects, or maps an
engine enum to a stable project-side contract. Query the exact enum identity
rather than relying on remembered names or numeric values.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- The Niagara module containing the enum must be loaded.
- Supply the full reflected `UEnum` path, including the `/Script/...` module
  prefix.
- Preserve the returned names and values with the engine revision that produced
  them because enum members can change between versions.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "enum": {
    "refPath": "/Script/Niagara.ENiagaraExecutionState"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned byte-identical JSON strings naming
`ENiagaraExecutionState` and seven entries with values `0` through `6`.
Independent reflected engine metadata confirmed the declared sequence from
`Active` through `Num`; the tool additionally exposed the generated
`ENiagaraExecutionState_MAX` entry.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON string and requires a second JSON parse.
- The returned array includes hidden and generated sentinel entries such as
  `Disabled`, `Num`, and `ENiagaraExecutionState_MAX`; enumeration does not mean
  every member is an appropriate user-facing or runtime input.
- Display names and descriptions are diagnostic metadata and must not replace
  canonical enum names in tool arguments.
- A class path, missing enum, or empty reference raises a native parameter error.
- Numeric enum values and membership can change with the engine or plugin
  version; query the live editor before constructing dependent arguments.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Current**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe NiagaraToolsets.NiagaraToolset_Info
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `enum`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  NiagaraToolsets.NiagaraToolset_Info \
  NiagaraToolsets.NiagaraToolset_Info.UEnum_Info \
  --arguments '
{
  "enum": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
