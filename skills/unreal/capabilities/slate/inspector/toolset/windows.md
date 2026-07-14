# Windows

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
SlateInspectorToolset.SlateInspectorToolset.Windows
```

Toolset:

```text
SlateInspectorToolset.SlateInspectorToolset
```

## What this tool does

List, select, or close top-level Slate editor windows.

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
Use this tool with `action: "list"` to enumerate top-level Slate windows before
SHAR captures a UI snapshot, registers an observer, or verifies which editor
surface is available. Treat select and close as separate UI mutations.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Slate must have initialized at least one top-level editor window.
- Use `action: "list"` for inspection and ignore `index` except where required
  by the live schema.
- Refresh the list immediately before using an index or discovered widget ref.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "action": "list",
  "index": -1
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned the same one-entry JSON array with index `0` and the title
"shar - Unreal Editor". A following `Snapshot` identified the corresponding
window as `w1`. An invalid action returned explanatory error text inside a
successful `returnValue` rather than raising a transport error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON string and requires a second JSON parse for `list`.
- Window indexes and focus can change when windows open, close, or reorder.
- `select` changes focus and `close` destroys a window; neither was used in the
  validated read-only example.
- Unknown actions can return an `Error: ...` string with transport success, so
  callers must inspect the semantic payload.
- Window titles are session and project state, not stable identifiers.
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

### `action`

- Required: **no**
- Type: `string`
- Default: `"list"`
- Purpose:

"list" returns JSON array, "select" brings to front, "close" destroys.

### `index`

- Required: **no**
- Type: `integer`
- Default: `-1`
- Purpose:

Window index for select/close.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SlateInspectorToolset.SlateInspectorToolset \
  SlateInspectorToolset.SlateInspectorToolset.Windows \
  --arguments '
{}
'
```

## Expected output

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
