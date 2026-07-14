# Get log entries

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.LogsToolset.GetLogEntries
```

Toolset:

```text
EditorToolset.LogsToolset
```

## What this tool does

Returns log entries from the current session's log file.

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
Use this tool to inspect bounded current-session diagnostics after SHAR editor
startup or an automated MCP operation. The verified workflow used
`LogToolsetRegistry` to confirm native toolset registration and used a global
search to confirm that the editor opened `src/uproject/shar.uproject`.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Discover the exact category with `GetLogCategories` when filtering by
  category.
- Supply a valid regular expression and a bounded positive `maxEntries` value.
- The evidence must exist in the current editor session log.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "category": "LogToolsetRegistry",
  "maxEntries": 3,
  "pattern": "Registering Toolset"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The validated call returned three chronological `LogToolsetRegistry` entries.
They exactly matched the final three entries from an earlier ten-entry query
with the same category. A separate all-category search found the current
startup entry for the canonical SHAR project.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Results come from the current editor session log, not durable project state.
- `maxEntries` selects entries from the end of the matching result; `0` removes
  the limit and should be avoided for routine inspection.
- An empty `category` searches all categories and can produce broad output.
- A registered category or valid pattern can still return an empty array.
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

If non-empty, only returns entries from this log category (e.g. "LogTemp").

### `maxEntries`

- Required: **no**
- Type: `integer`
- Default: `1000`
- Purpose:

Maximum number of entries to return, taken from the end of the log. Pass 0 for
no limit. Defaults to 1000.

### `pattern`

- Required: **yes**
- Type: `string`
- Purpose:

If non-empty, only returns entries whose text matches this regular expression.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.LogsToolset \
  EditorToolset.LogsToolset.GetLogEntries \
  --arguments '
{
  "pattern": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

A list of matching log entries in chronological order.

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
