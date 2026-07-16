# Set verbosity

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.LogsToolset.SetVerbosity
```

Toolset:

```text
EditorToolset.LogsToolset
```

## What this tool does

Sets the verbosity level for a log category.

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
Use this tool for a bounded diagnostic change when a specific SHAR editor log
category is too noisy or not detailed enough for the current investigation.
The verified workflow temporarily changed `LogToolsetRegistry` from `Log` to
`Display`, confirmed the new runtime state, and restored the original level.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Resolve the exact category with `GetLogCategories` and capture its current
  level with `GetVerbosity` before changing it.
- Use one verbosity value declared by the live schema and define the restoration
  value before invocation.
- Avoid overlapping diagnostics that depend on the category while its level is
  temporarily changed.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "category": "LogToolsetRegistry",
  "verbosity": "Display"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`GetVerbosity` established the pre-state as `Log`. The validated mutation
returned `null`, and an independent `GetVerbosity` call returned `Display`.
A second mutation restored `Log`, which another independent read confirmed.
An invalid verbosity value failed and left the restored `Log` state unchanged.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool declares no structured output schema; the observed `null` return does
  not prove the change, so verify with `GetVerbosity`.
- The verified effect was current editor runtime state. Persistence across an
  editor restart was not tested.
- Changing verbosity can suppress evidence or greatly increase log volume;
  capture pre-state and restore it after the bounded investigation.
- Unsupported verbosity names fail with a native validation error and do not
  change the category's current level.
- Omitting `category` targets the schema default `LogsToolset`; use an explicit
  discovered category for SHAR diagnostics.
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `category`

- Required: **no**
- Type: `string`
- Default: `"LogsToolset"`
- Purpose:

The log category name, e.g. "LogTemp".

### `verbosity`

- Required: **yes**
- Type: `string`
- Purpose:

The verbosity level: one of "NoLogging", "Fatal", "Error", "Warning",
"Display", "Log", "Verbose", or "VeryVerbose".

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.LogsToolset \
  EditorToolset.LogsToolset.SetVerbosity \
  --arguments '
{
  "verbosity": "<value>"
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
