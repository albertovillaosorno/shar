# Get verbosity

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.LogsToolset.GetVerbosity
```

Toolset:

```text
EditorToolset.LogsToolset
```

## What this tool does

Returns the current verbosity level for a log category.

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
Use this tool to capture the current verbosity before interpreting missing SHAR
Toolset Registry diagnostics or before a controlled `SetVerbosity` operation.
The verified category `LogToolsetRegistry` reported the runtime level `Log`.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Obtain the exact category from `GetLogCategories`; do not infer it from a
  display label or toolset identity.
- No world, asset, or editor selection is required.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "category": "LogToolsetRegistry"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`GetLogCategories` first returned `LogToolsetRegistry` for the
`ToolsetRegistry` filter. Two consecutive calls with the validated arguments
then returned `"Log"`, proving a stable read of the current runtime setting.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An unregistered category fails with a native “category not found” error; it
  does not return `NoLogging` or an empty value.
- The result is current runtime state and can change after `SetVerbosity`.
- A valid verbosity result does not prove that the category has log entries.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe EditorToolset.LogsToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `category`

- Required: **no**
- Type: `string`
- Default: `"LogsToolset"`
- Purpose:

The log category name, e.g. "LogTemp".

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.LogsToolset \
  EditorToolset.LogsToolset.GetVerbosity \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

The verbosity level as a string: one of "NoLogging", "Fatal", "Error",
"Warning", "Display", "Log", "Verbose", or "VeryVerbose". Raises a script error
if the category is not found.

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
