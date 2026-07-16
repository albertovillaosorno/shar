# Snapshot

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
SlateInspectorToolset.SlateInspectorToolset.Snapshot
```

Toolset:

```text
SlateInspectorToolset.SlateInspectorToolset
```

## What this tool does

Capture a Slate UI accessibility snapshot. Use this to read the current widget
tree and discover refs for action tools (Click, Type, Hover, etc.). A shallow
root observer (depth 0) covers top-level windows automatically. Before
interacting with a specific window or panel, call Observe() on it to get deep
coverage, then Snapshot that subtree to see its contents. Refs discovered by a
previous Snapshot remain usable. You do NOT need to call Snapshot again before
every action.

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
Use this tool to read a bounded Slate accessibility subtree before SHAR verifies
editor UI state or selects a widget ref for a later controlled action. Keep
source locations disabled for portable review output.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Obtain the target ref from `Windows`, a prior snapshot, or an active observer.
- Register a bounded observer first when the target subtree needs continuously
  refreshed refs.
- Choose the smallest useful `maxDepth` and keep source locations disabled.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "ref": "w1",
  "maxDepth": 6,
  "bIncludeSourceLocations": false
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two observed-window snapshots returned the same four-node structure: one editor
window and three image widgets with refs `w1`, `i1`, `i2`, and `i3`. Position
and size remained stable, while the focused marker and serialized bytes changed
between calls. `WaitFor` independently found the window title.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The result is a plain-text accessibility tree, not JSON.
- Widget refs are editor-session identities and can become stale after UI
  reconstruction.
- Snapshot bytes are not deterministic because focus and live UI metadata can
  change even when structure and geometry remain stable.
- A larger `maxDepth` cannot expose semantics that the current accessibility
  tree represents only as image widgets.
- Enabling source locations can introduce engine or workstation-specific paths
  into review output and was not used.
- Snapshot does not interact with the UI or prove that a widget action will
  succeed.
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

### `bIncludeSourceLocations`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

Include [src=File:Line] tags showing where each widget was created in C++.

### `maxDepth`

- Required: **no**
- Type: `integer`
- Default: `30`
- Purpose:

Maximum depth (default 30).

### `ref`

- Required: **yes**
- Type: `string`
- Purpose:

Subtree root ref. Empty = all windows.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SlateInspectorToolset.SlateInspectorToolset \
  SlateInspectorToolset.SlateInspectorToolset.Snapshot \
  --arguments '
{
  "ref": "<value>"
}
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
