# Capture editor image

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.EditorAppToolset.CaptureEditorImage
```

Toolset:

```text
EditorToolset.EditorAppToolset
```

## What this tool does

Captures an image of the entire editor application as the user sees it.

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
Use this tool to capture the complete SHAR Editor application for bounded
visual verification when a viewport-only or asset-only image would omit
required panels, dialogs, or other editor context.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must be open in the intended interactive editor.
- Arrange the editor windows and panels that must appear in the evidence image.
- Use `CaptureViewport` or `CaptureAssetImage` when the whole application is
  broader than the required evidence.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned an object containing `mimeType: "image/png"` and base64
`data`. Strict base64 decoding produced a 727,211-byte PNG with the canonical
PNG signature and a valid IHDR declaring a 1280 by 688 image. A second capture
also returned a non-empty PNG payload, proving that the result was not an empty
transport envelope.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The image covers the entire editor application, not only the level viewport.
- Dimensions and encoded size depend on the current window layout and visible
  editor state; do not assert a stable byte hash across captures.
- The returned base64 payload can be large, so summarize or decode it without
  printing the full value into terminal logs.
- A valid screenshot proves visible editor state only; use structured read tools
  for asset identities, values, counts, and persistence.
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
shar-unreal-mcp describe EditorToolset.EditorAppToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

This tool accepts no input fields.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.CaptureEditorImage \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

An image of editor windows as they appear on the users' desktop.

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
