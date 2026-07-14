# Search row structs

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.data_table.DataTableTools.search_row_structs
```

Toolset:

```text
editor_toolset.toolsets.data_table.DataTableTools
```

## What this tool does

Finds structs that can be used as a DataTable schema.

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
Use this tool to discover reflected `TableRowBase` structs before SHAR creates
or imports a DataTable with a selected schema.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply an explicit wildcard pattern.
- Use the full returned ScriptStruct ref for later schema arguments.
- Treat results as dependent on loaded modules and plugins.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "struct_name": "*GameplayTag*"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two wildcard calls returned `GameplayTagTableRow` and
`RestrictedGameplayTagTableRow`. `GameplayTag*` returned only the first because
matching is against the struct name. A global `*` search returned 17 structs,
and an unmatched pattern returned an empty array.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Matching is wildcard-based rather than semantic search.
- Leading and trailing wildcards change prefix and substring behavior.
- Results vary with loaded modules and plugins.
- Discovery does not prove a struct is appropriate for SHAR content semantics.
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
shar-unreal-mcp describe editor_toolset.toolsets.data_table.DataTableTools
```

1. Confirm every required input against the current schema.

## Inputs

### `struct_name`

- Required: **no**
- Type: `string`
- Default: `"*"`
- Purpose:

If set, will filter structs by name using wildcard match.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.data_table.DataTableTools \
  editor_toolset.toolsets.data_table.DataTableTools.search_row_structs \
  --arguments '
{}
'
```

## Expected output

A list of structs derived from TableRowBase that match the criteria.

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
