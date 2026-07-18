# Import file

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.data_table.DataTableTools.import_file
```

Toolset:

```text
editor_toolset.toolsets.data_table.DataTableTools
```

## What this tool does

Imports a file from disk as a DataTable asset.

The file's columns must match the property names in schema. Use
search_row_structs to discover usable schema structs.

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
Use this tool to import a typed SHAR DataTable from a bounded CSV source whose
header matches the discovered row schema.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live DataTableTools schema.
- Discover the exact ScriptStruct with `search_row_structs` and inspect the
  resulting table with `get_schema`.
- Use an absolute source-file path visible to the Unreal Editor process and
  keep the source file disposable.
- Match the CSV header to `Name,Tag,DevComment` for `GameplayTagTableRow`.
- Define whole-folder asset cleanup before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "asset_name": "DT_MCP_Imported_43004322",
  "folder_path": "/Game/SHAR_MCP_Validation_DataTable_43004322",
  "schema": {
    "refPath": "/Script/GameplayTags.GameplayTagTableRow"
  },
  "source_file": "C:/<workspace>/temp/data-table-validation.csv"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The import returned one exact DataTable asset. `list_rows` returned
`Imported`, `get_rows` returned tag `None` and the imported developer comment,
and `get_schema` matched the source table schema.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- DataTable references and row schemas are live editor identities; rediscover
  them after deletion, import, or schema replacement.
- The tested schema was `/Script/GameplayTags.GameplayTagTableRow`, whose
  writable fields are `tag` and `devComment`.
- The source path must be absolute for the Unreal process; repository guidance
  sanitizes the private workspace prefix.
- CSV column names and value encoding must match the selected ScriptStruct
  exactly.
- This validation used one disposable content folder and removed the imported
  CSV after verification.
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
shar-unreal-mcp describe editor_toolset.toolsets.data_table.DataTableTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `asset_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the new asset.

### `folder_path`

- Required: **yes**
- Type: `string`
- Purpose:

The content-browser folder to create the asset in.

### `schema`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `source_file`

- Required: **yes**
- Type: `string`
- Purpose:

The absolute path to the source file on disk.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.data_table.DataTableTools \
  editor_toolset.toolsets.data_table.DataTableTools.import_file \
  --arguments '
{
  "asset_name": "<value>",
  "folder_path": "<value>",
  "schema": {},
  "source_file": "<value>"
}
'
```

## Expected output

The assets produced by the import (typically a single DataTable).

### `returnValue`

- Required: **yes**
- Type: `array<object>`
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
