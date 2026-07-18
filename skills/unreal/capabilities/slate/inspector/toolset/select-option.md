# Select option

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
SlateInspectorToolset.SlateInspectorToolset.SelectOption
```

Toolset:

```text
SlateInspectorToolset.SlateInspectorToolset
```

## What this tool does

Select an option in a Slate combobox by its text label. Opens the dropdown,
finds the matching text, and clicks it.

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
Use this tool to choose one exact Unreal editor combobox option for a bounded
SHAR workflow, such as entering Landscape mode and returning to Selection mode
after inspection.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Capture a fresh Slate snapshot and resolve the exact visible combobox ref.
- Record the current option text and define the exact restoration option before
  changing editor mode or workflow state.
- Use the option label exactly as presented by the live editor.
- Expect refs to become stale after the selection changes the surrounding UI;
  take a new snapshot before restoration or any dependent action.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "ref": "co132",
  "value": "Landscape"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The initial snapshot showed `Selection Mode` on `co132`. Selecting
`Landscape` returned `true`; a fresh deep snapshot independently showed
`Landscape Mode`, `Manage`, `Sculpt`, `Paint`, and `Landscape Editor`. The
updated combobox ref was `co164`. Selecting `Selection` on that fresh ref
returned `true`, and the final snapshot showed `Selection Mode` with no
Landscape mode or Landscape editor controls.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Slate refs are session- and tree-state-specific. The validated combobox
  changed from `co132` to `co164` after the first selection.
- A `true` result requires a fresh snapshot that proves the visible option and
  dependent UI actually changed.
- The option text must match the dropdown label; unsupported or nonstandard
  comboboxes may return `false`.
- Selecting an editor mode can replace large widget subtrees and invalidate
  unrelated refs captured before the change.
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
shar-unreal-mcp describe SlateInspectorToolset.SlateInspectorToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `ref`

- Required: **yes**
- Type: `string`
- Purpose:

Slate combobox ref.

### `value`

- Required: **yes**
- Type: `string`
- Purpose:

Exact option text to select.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SlateInspectorToolset.SlateInspectorToolset \
  SlateInspectorToolset.SlateInspectorToolset.SelectOption \
  --arguments '
{
  "ref": "<value>",
  "value": "<value>"
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
