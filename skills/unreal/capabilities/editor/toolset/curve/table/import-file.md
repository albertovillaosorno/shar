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
editor_toolset.toolsets.curve_table.CurveTableTools.import_file
```

Toolset:

```text
editor_toolset.toolsets.curve_table.CurveTableTools
```

## What this tool does

Imports a file from disk as a CurveTable asset.

The file's first column is the row name; subsequent columns are sample times
and values. interp_mode controls how the imported keys are interpolated between
samples.

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
Use this mutation to create a SHAR CurveTable from a reviewed CSV matrix of row
names, sample times, and numeric values.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Provide an absolute local CSV path.
- Put sample times in the header after the first row-name column.
- Use one subsequent row per curve.
- Choose an unused destination asset name and explicit interpolation mode.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "folder_path": "/Game",
  "asset_name": "CT_SHAR_MCP_ImportProbe_101",
  "source_file": "C:/Temp/shar-curve-table-import.csv",
  "interp_mode": "RCIM_Linear"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Linear and constant imports each returned one CurveTable with Speed and Density
rows. Both rows contained times 0, 1, and 2.5 with the expected numeric values.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent asset import and returns a one-element reference list.
- Simple GetKeys readback does not expose interpolation mode.
- Importing into an existing name raises `import_asset ... already exists`.
- A malformed nonnumeric header was accepted, so validate imported rows and keys
  rather than relying on parser rejection.
- AssetTools.exists can miss a transient name collision after an earlier failed
  import; use fresh names when necessary.
- Remove temporary source files after import.
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
shar-unreal-mcp describe editor_toolset.toolsets.curve_table.CurveTableTools
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

### `interp_mode`

- Required: **yes**
- Type: `string`
- Allowed values:

  - `"RCIM_Linear"`
  - `"RCIM_Constant"`
  - `"RCIM_Cubic"`
  - `"RCIM_None"`
- Purpose:

ERichCurveInterpMode

### `source_file`

- Required: **yes**
- Type: `string`
- Purpose:

The absolute path to the source file on disk.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.curve_table.CurveTableTools \
  editor_toolset.toolsets.curve_table.CurveTableTools.import_file \
  --arguments '
{
  "asset_name": "<value>",
  "folder_path": "<value>",
  "interp_mode": "RCIM_Linear",
  "source_file": "<value>"
}
'
```

## Expected output

The assets produced by the import (typically a single CurveTable).

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
