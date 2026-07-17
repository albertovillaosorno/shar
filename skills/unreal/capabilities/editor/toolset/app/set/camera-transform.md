# Set camera transform

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.EditorAppToolset.SetCameraTransform
```

Toolset:

```text
EditorToolset.EditorAppToolset
```

## What this tool does

Sets the position and rotation of the level viewport camera.

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
Use this tool to restore a captured SHAR level-viewport camera after
bounded inspection, framing, or focus operations without changing any
project asset.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must be open with an active level viewport.
- Capture the current transform with `GetCameraTransform` before changing it.
- Supply every location, rotation, and scale component required by the live
  schema.
- Keep the operation outside PIE unless the requested camera belongs to PIE.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "transform": {
    "location": {
      "x": -610.426708,
      "y": 429.850689,
      "z": 208.675465
    },
    "rotation": {
      "pitch": 8.952503,
      "yaw": -61.121499,
      "roll": 0
    },
    "scale": {
      "x": 1,
      "y": 1,
      "z": 1
    }
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned `returnValue: null`. A separate
`GetCameraTransform` call returned the same location, rotation, and unit
scale, proving that the active level viewport applied the complete transform.
The verified call reused the captured pre-state, so the operator camera was
left unchanged.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The mutation is transient editor state; it does not save a camera asset.
- The response does not contain the applied transform, so a fresh
  `GetCameraTransform` call is required for verification.
- Unreal normalizes floating-point values and negative zero in the returned
  transform; compare numeric values rather than serialized text.
- Capture the pre-state before any non-no-op movement so recovery is exact.
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

### `transform`

- Required: **yes**
- Type: `object`
- Purpose:

The transform to apply to the viewport camera.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.SetCameraTransform \
  --arguments '
{
  "transform": {}
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
