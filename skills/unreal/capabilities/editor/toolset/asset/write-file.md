# Write file

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.asset.AssetTools.write_file
```

Toolset:

```text
editor_toolset.toolsets.asset.AssetTools
```

## What this tool does

Writes text content to a file on disk.

Only files under /Game/, an enabled plugin's Content/ directory, or the project
Saved/ directory may be written. Only plain text formats are supported.
Overwrites the file if it already exists.

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
Use this mutation to create or replace a small reviewed plain-text file inside
the SHAR project Saved root or another explicitly allowed content root.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve an absolute path under an allowed root.
- Confirm that overwriting the destination is intended.
- Restrict content to an approved plain-text format.
- Plan explicit cleanup for disposable validation files.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "file_path": (
    "C:/workspace/shar/src/uproject/Saved/"
    "SHAR_MCP_FileProbe_1.txt"
  ),
  "content": "Hello, Springfield!
Line 2: café ☕
"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles returned JSON null, created the file, preserved Unicode, and
overwrote it with replacement content. ReadFile independently returned the exact
logical text after each write.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a filesystem mutation and returns JSON null.
- Existing files are overwritten without a separate confirmation step.
- The path must be absolute; relative paths may resolve against the engine
  installation.
- Paths outside allowed roots, traversal outside Saved, and unsupported binary
  extensions are rejected.
- Windows wrote CRLF bytes even though ReadFile returned normalized `
` text.
- Delete disposable files explicitly after validation.
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

### `content`

- Required: **yes**
- Type: `string`
- Purpose:

The text content to write.

### `file_path`

- Required: **yes**
- Type: `string`
- Purpose:

The path to the file to write.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.asset.AssetTools \
  editor_toolset.toolsets.asset.AssetTools.write_file \
  --arguments '
{
  "content": "<value>",
  "file_path": "<value>"
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
