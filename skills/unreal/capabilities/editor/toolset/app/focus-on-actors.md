# Focus on actors

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.EditorAppToolset.FocusOnActors
```

Toolset:

```text
EditorToolset.EditorAppToolset
```

## What this tool does

Repositions the level editor camera to focus on the specified actors. Cannot be
called while PIE is active.

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
Use this tool to frame one or more resolved SHAR level actors in the active
editor viewport before visual inspection, coordinate conversion, or a bounded
viewport capture.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must have an active level viewport.
- PIE must be stopped; the live tool rejects focus during PIE.
- Resolve every actor through a current scene read and capture the viewport
  camera with `GetCameraTransform` before invocation.
- Prefer actors with real component bounds, and define `SetCameraTransform`
  recovery before testing focus.
- Poll fresh camera reads because viewport framing settles asynchronously.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "actors": [
    {
      "refPath": "/Game/Untitled.Untitled:PersistentLevel.StaticMeshActor_UAID_00E04C68026767EF02_1945126057"
    }
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A disposable Engine cube was placed at `(250000, -175000, 42000)` with
scale `(4, 6, 8)`. The call returned no structured value. The first two camera
polls remained at the captured origin; the third poll moved the viewport to
approximately `(248799.719, -175000, 42000)`, independently proving that the
camera framed the target. `GetSelectedActors` stayed empty, so focusing did not
select the actor. Cleanup restored the exact captured camera, removed the cube,
and a fresh scene search returned no task actor.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The mutation is asynchronous editor state. Poll `GetCameraTransform` rather
  than reading it only once after the call.
- `returnValue: null` is not proof of focus; require a fresh camera transform or
  another independent viewport postcondition.
- `GetVisibleActors` already included the far test cube before focus in this
  session, so visibility membership alone was not a reliable success oracle.
- Actor references contain transient level-instance identifiers. Resolve a fresh
  actor reference for every session instead of reusing the example literally.
- The tool changes camera framing but does not select the target actor.
- Always restore the captured camera when focus is used only for validation.
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

### `actors`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The actors to focus the level camera on.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.FocusOnActors \
  --arguments '
{
  "actors": []
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
