# Get log categories

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.LogsToolset.GetLogCategories
```

Toolset:

```text
EditorToolset.LogsToolset
```

## What this tool does

Returns a sorted list of registered log categories.

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
Use this tool to discover the exact current log category before reading SHAR
startup, Toolset Registry, or MCP diagnostics. In the verified editor session,
filtering for `ToolsetRegistry` resolved `LogToolsetRegistry`, which was then
used by `GetLogEntries` and `GetVerbosity`.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must be open, and `doctor` must report a ready,
  non-empty Toolset Registry.
- No world or asset selection is required.
- Discover category names in the current editor session before passing them to
  another log tool.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "filter": "ToolsetRegistry"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The validated call returned exactly `["LogToolsetRegistry"]`, and an immediate
repeat returned the same array. A broader `Toolset` filter returned category
names in sorted order, while an unmatched filter returned an empty array.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Substring matching was case-insensitive in the verified editor session.
- An unmatched filter returns an empty array rather than an error.
- A registered category can still have no matching current-session entries;
  confirm content separately with `GetLogEntries`.
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
shar-unreal-mcp describe EditorToolset.LogsToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `filter`

- Required: **yes**
- Type: `string`
- Purpose:

If non-empty, only returns categories whose name contains this substring.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.LogsToolset \
  EditorToolset.LogsToolset.GetLogCategories \
  --arguments '
{
  "filter": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

A sorted list of log category names (e.g. ["LogBlueprint", "LogTemp"]).

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
