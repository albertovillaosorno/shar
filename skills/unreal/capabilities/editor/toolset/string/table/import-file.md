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
editor_toolset.toolsets.string_table.StringTableTools.import_file
```

Toolset:

```text
editor_toolset.toolsets.string_table.StringTableTools
```

## What this tool does

Imports a file from disk as a StringTable asset.

The file must have a header row with at least 'Key' and 'SourceString' columns.
Additional meta-data columns are imported but the namespace is not - the
StringTable's namespace is derived from its asset path.

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
Use this mutation to create a SHAR StringTable from a reviewed CSV source
containing localisation keys and source strings.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Provide an absolute local source-file path.
- Include `Key` and `SourceString` header columns.
- Choose an unused destination asset name.
- Keep import sources outside the public repository when they are temporary.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "folder_path": "/Game",
  "asset_name": "ST_SHAR_MCP_ImportFixture_1",
  "source_file": "C:/Temp/shar-string-table-import.csv"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two UTF-8 CSV imports returned one StringTable reference each. Both tables
contained `Greeting` and `Farewell`; namespace matched the asset name and table
ID matched the object path.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent asset import and returns a one-element reference list.
- Additional metadata columns are accepted, but namespace is derived from the
  asset path.
- Importing into an existing asset name raises `already exists`.
- Missing required headers produce a generic import failure.
- In both cycles AssetTools.delete returned false even though AssetTools.exists
  then returned false; verify absence rather than trusting the delete Boolean
  alone.
- Temporary source files must be removed after import.
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
shar-unreal-mcp describe editor_toolset.toolsets.string_table.StringTableTools
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

### `source_file`

- Required: **yes**
- Type: `string`
- Purpose:

The absolute path to the source file on disk.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.string_table.StringTableTools \
  editor_toolset.toolsets.string_table.StringTableTools.import_file \
  --arguments '
{
  "asset_name": "<value>",
  "folder_path": "<value>",
  "source_file": "<value>"
}
'
```

## Expected output

The assets produced by the import (a single StringTable).

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
