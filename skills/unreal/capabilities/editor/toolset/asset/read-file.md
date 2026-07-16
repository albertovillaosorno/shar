# Read file

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.asset.AssetTools.read_file
```

Toolset:

```text
editor_toolset.toolsets.asset.AssetTools
```

## What this tool does

Reads a text file from disk and returns its contents.

Only files under /Game/, an enabled plugin's Content/ directory, or the project
Saved/ directory may be read. Only plain text formats are supported.

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
Use this read to inspect a reviewed plain-text file inside the SHAR project
Content or Saved roots, or an enabled plugin Content root.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve an absolute path under an allowed root.
- Confirm the file is an approved plain-text format.
- Keep the requested file narrow and task-owned.
- Use project APIs rather than this tool for UObject assets.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "file_path": (
    "C:/workspace/shar/src/uproject/Saved/"
    "SHAR_MCP_FileProbe_1.txt"
  )
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles read back the exact Unicode text written by WriteFile, including the
final newline. Windows stored CRLF bytes, while the returned text normalized
them to ` `.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The path must be absolute. A relative `Saved/...` path resolved against the
  engine binary directory and was rejected.
- A missing file raises `File not found`.
- Reads are restricted to project Content, project Saved, and enabled plugin
  Content roots.
- Only approved plain-text formats are supported.
- Returned newline representation may differ from on-disk line endings.
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
shar-unreal-mcp describe editor_toolset.toolsets.asset.AssetTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `file_path`

- Required: **yes**
- Type: `string`
- Purpose:

The path to the file to read.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.asset.AssetTools \
  editor_toolset.toolsets.asset.AssetTools.read_file \
  --arguments '
{
  "file_path": "<value>"
}
'
```

## Expected output

The text content of the file.

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
