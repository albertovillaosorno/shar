# Screenshot

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
SlateInspectorToolset.SlateInspectorToolset.Screenshot
```

Toolset:

```text
SlateInspectorToolset.SlateInspectorToolset
```

## What this tool does

Screenshot a Slate widget or the active editor window. Prefer this over
SceneTools.take_screenshot for Editor UI; use SceneTools only for 3D viewport.

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
Use this tool to capture bounded visual evidence from a known Slate window or
widget before and after SHAR editor-UI work. Discover the exact ref through
`Windows` and `Snapshot`, then capture that ref rather than relying on active-
window inference.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Obtain a current Slate widget ref from `Snapshot` in the same editor session.
- Prefer the smallest ref that contains the evidence required for review.
- Decode the returned base64 image in memory or route temporary output outside
  tracked repository content.
- Pause window resizing and focus changes when comparing repeated captures.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "ref": "w1"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two captures of window ref `w1` returned byte-identical valid `image/png`
payloads at 1358 by 718 pixels. Refs `i1` and `i2` produced the same image,
while inset ref `i3` returned a deterministic 1354 by 714 PNG with different
bytes. The dimensions matched the corresponding `Snapshot` geometry.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty ref and a deliberately missing ref returned an empty MIME type and
  zero image bytes without a native error; verify payload length and MIME type
  explicitly.
- Refs are session-local and can change when Slate windows or widget trees are
  rebuilt.
- A parent window and image child can resolve to identical pixels when they
  cover the same geometry.
- Screenshot success proves raster capture only; it does not prove widgets are
  interactive, accessible, or semantically correct.
- Do not commit large base64 payloads or temporary PNG review artifacts.
- Repeated bytes were stable for the verified static UI state, but live
  animations, focus, tooltips, and notifications can change output.
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

Slate widget ref. Empty = active window.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SlateInspectorToolset.SlateInspectorToolset \
  SlateInspectorToolset.SlateInspectorToolset.Screenshot \
  --arguments '
{
  "ref": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

ToolsetImage

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
