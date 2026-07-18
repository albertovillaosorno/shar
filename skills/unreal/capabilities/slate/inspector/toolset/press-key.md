# Press key

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
SlateInspectorToolset.SlateInspectorToolset.PressKey
```

Toolset:

```text
SlateInspectorToolset.SlateInspectorToolset
```

## What this tool does

Press and release a keyboard key on the currently focused Slate widget.
Supports modifier prefixes: "Ctrl+C", "Shift+1".

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
Use this tool to send one exact keyboard action to the currently focused Slate
widget.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require doctor readiness and observe the exact visible window at sufficient
  depth.
- Resolve every widget ref from a fresh Snapshot; never reuse refs after
  docking, closing, or layout reconstruction.
- Capture a separate visual or accessibility postcondition and restore
  transient focus, filters, or layout when practical.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "key": "Backspace"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
After selecting all search text, Backspace returned true and the details
snapshot restored both `Actions` and `Asset Manager` rows.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Slate refs are observer- and layout-specific transient identities.
- Boolean success is insufficient; verify visible content, window topology,
  counters, or screenshot output independently.
- Accessibility names may remain placeholders such as `Search` while the
  underlying text filter changes.
- The key is sent to the currently focused widget; establish focus explicitly
  before destructive shortcuts.
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

### `key`

- Required: **yes**
- Type: `string`
- Purpose:

Key name with optional modifiers, e.g. "Enter", "Ctrl+A", "Shift+Tab".

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SlateInspectorToolset.SlateInspectorToolset \
  SlateInspectorToolset.SlateInspectorToolset.PressKey \
  --arguments '
{
  "key": "<value>"
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
