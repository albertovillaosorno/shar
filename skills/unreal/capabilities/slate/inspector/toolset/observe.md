# Observe

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
SlateInspectorToolset.SlateInspectorToolset.Observe
```

Toolset:

```text
SlateInspectorToolset.SlateInspectorToolset
```

## What this tool does

Register an observer on a widget subtree so its refs are continuously kept up
to date (~100ms tick). Call this on the window or panel you are about to work
with. It ensures new widgets appearing in that subtree are assigned refs
automatically. Unobserve when you are done. A shallow root observer (depth 0)
already covers top-level windows.

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
Use this tool to keep refs in one bounded Slate window or panel current while
SHAR performs a short UI inspection sequence. Always pair the registration with
`Unobserve` in guaranteed cleanup.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Discover a current widget ref through `Windows` or `Snapshot`.
- Choose the smallest useful `maxDepth` for the target subtree.
- Establish `Unobserve` cleanup before registering the observer.
- Avoid overlapping observers for the same subtree unless explicitly required.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "ref": "w1",
  "maxDepth": 6
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two registrations returned `observer_2` and `observer_3`. `ListObservers` showed
each temporary observer beside the built-in root observer with root ref `w1` and
depth `6`. After a short refresh interval, `Snapshot` used the observed ref
successfully. Each identifier was then removed with `Unobserve`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a transient editor-state mutation and must be paired with cleanup.
- Observer identifiers are session-local and not reusable stable names.
- The observer refreshes on a short tick, so an immediate snapshot can precede
  cache refresh.
- Increasing depth does not create accessibility information that the widget
  tree does not expose.
- Duplicate or overlapping observers consume extra refresh work and complicate
  ref ownership.
- The built-in root observer already covers top-level windows at depth `0`.
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

### `maxDepth`

- Required: **no**
- Type: `integer`
- Default: `30`
- Purpose:

Maximum depth to walk from the root.

### `ref`

- Required: **yes**
- Type: `string`
- Purpose:

Root widget ref to observe. Empty = all visible windows.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SlateInspectorToolset.SlateInspectorToolset \
  SlateInspectorToolset.SlateInspectorToolset.Observe \
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
