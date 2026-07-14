# Wait for

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
SlateInspectorToolset.SlateInspectorToolset.WaitFor
```

Toolset:

```text
SlateInspectorToolset.SlateInspectorToolset
```

## What this tool does

Check if text is present or absent in the Slate widget tree. Non-blocking:
checks once and returns immediately. Poll to wait.

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
Use this tool as a non-blocking UI-state predicate before SHAR continues an
editor automation sequence. It can require one text value to be present while
simultaneously requiring another value to be absent.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Obtain stable expected text from a current window list or snapshot.
- Supply both required fields; use an empty string to skip one predicate.
- Poll at a bounded cadence when the caller needs to wait for a transition.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "text": "shar - Unreal Editor",
  "textGone": "DefinitelyMissingSlateText9f3d7c"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned `true` when the editor title was present and the unique
missing text was absent. Requiring the missing text returned `false`, and
requiring the editor title to be gone also returned `false`. `Windows` and
`Snapshot` independently exposed the title.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool checks once and returns immediately; it does not block or poll.
- When both fields are nonempty, both the present and absent predicates must be
  satisfied.
- Empty text skips that predicate rather than matching every widget.
- Text can be duplicated, localized, truncated, or change with project state;
  use a sufficiently specific current value.
- A `true` result proves text-tree state only, not widget identity or action
  readiness.
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
shar-unreal-mcp describe SlateInspectorToolset.SlateInspectorToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `text`

- Required: **yes**
- Type: `string`
- Purpose:

Text that must be present (empty = skip).

### `textGone`

- Required: **yes**
- Type: `string`
- Purpose:

Text that must be absent (empty = skip).

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SlateInspectorToolset.SlateInspectorToolset \
  SlateInspectorToolset.SlateInspectorToolset.WaitFor \
  --arguments '
{
  "text": "<value>",
  "textGone": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
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
