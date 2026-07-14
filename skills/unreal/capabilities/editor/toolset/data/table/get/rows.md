# Get rows

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.data_table.DataTableTools.get_rows
```

Toolset:

```text
editor_toolset.toolsets.data_table.DataTableTools
```

## What this tool does

Returns the column values for one or more rows as a JSON string.

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
Use this tool to read a bounded set of DataTable rows as structured JSON during
SHAR content or import review.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact loaded `UDataTable` object path.
- Discover row names with `list_rows`.
- Parse the returned JSON string once more before using values.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "data_table": {"refPath": "/DatasmithContent/Datasmith/AreaLightsTable.AreaLightsTable"},
  "row_names": ["EDatasmithAreaLightActorShape::Rectangle"]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two single-row reads returned the rectangle mesh reference to
`/DatasmithContent/Meshes/square.square`. A two-row read also returned the None
row with `mesh: "None"`. An empty row list returned the JSON string `{}`, while
an unknown row raised a rows-do-not-exist error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The return value is JSON text and requires a second parse.
- Object references are exported as Unreal text paths, not `refPath` objects.
- Null-like object values can serialize as the string `"None"`.
- Unknown rows raise an error instead of being omitted.
- Missing table refs fail during parameter translation.
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

### `data_table`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `row_names`

- Required: **yes**
- Type: `array<string>`
- Purpose:

The names of the rows to retrieve.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.data_table.DataTableTools \
  editor_toolset.toolsets.data_table.DataTableTools.get_rows \
  --arguments '
{
  "data_table": {},
  "row_names": []
}
'
```

## Expected output

A JSON object mapping each row name to an object of property names and values.

### `returnValue`

- Required: **yes**
- Type: `string`
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
