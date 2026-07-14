# Capture viewport

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Review required**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.EditorAppToolset.CaptureViewport
```

Toolset:

```text
EditorToolset.EditorAppToolset
```

## What this tool does

Captures the level viewport with optional annotations.

Annotations rendering overlays a projected 3D world-space grid plus name +
position labels on visible actors. The grid is drawn at a configurable ground-
plane Z and projected through the camera, with coordinate numbers at
intersections (shown in meters). Each labeled actor gets a crosshair at its
projected screen position with a leader-line callout placed to avoid overlap.
This gives a vision-capable agent spatial awareness: it can reference grid
coordinates to direct placement and identify scene contents by label.

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
[TODO]
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
[TODO]
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
[FILL_ME]
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
[TODO]
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
[TODO]
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

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

### `annotations`

- Required: **no**
- Type: `object`
- Purpose:

Optional annotation overlay configuration. Only use this when you need the
information in order to perform spatial actions.

### `bShowUI`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If false (default), editor UI overlays such as transform gizmos and selection
outlines are hidden in the captured image. Set true to capture exactly what's
on screen, gizmos and all.

### `captureTransform`

- Required: **no**
- Type: `object`
- Purpose:

Optional pose to capture from. If unset, uses the viewport's current camera.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.CaptureViewport \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

The captured image and associated metadata.

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
