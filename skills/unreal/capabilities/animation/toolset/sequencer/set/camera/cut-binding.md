# Set camera cut binding

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.sequencer.SequencerTools.set_camera_cut_binding
```

Toolset:

```text
animation_toolset.toolsets.sequencer.SequencerTools
```

## What this tool does

Set which camera a camera cut section uses.

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
Use this tool to assign a camera binding to a camera-cut section only after the
installed GUID construction is compatible with UE 5.8.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve a real MovieSceneCameraCutSection and camera binding from the same
  sequence.
- Capture the current camera binding ID before mutation.
- Use a disposable section until the UE 5.8 GUID-construction defect is fixed.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "section": {
    "refPath": "/Game/Cinematics/LS_Example.LS_Example:MovieScene_0.MovieSceneCameraCutSection_0"
  },
  "camera_binding_id": "11111111-2222-3333-4444-555555555555"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two disposable camera-cut fixtures reached this wrapper. UE 5.8 reported `call()
takes at most 0 arguments (1 given)` when the wrapper constructed
`unreal.Guid(camera_binding_id)`, so no binding change was accepted.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The installed wrapper uses a GUID constructor form rejected by UE 5.8.
- The section and camera binding must belong to the same sequence hierarchy.
- Verify the section camera-binding ID independently after any future successful
  call.
- Restore the original binding when validating on a disposable section.
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
shar-unreal-mcp describe animation_toolset.toolsets.sequencer.SequencerTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `camera_binding_id`

- Required: **yes**
- Type: `string`
- Purpose:

The binding ID of the camera (from get_bindings).

### `section`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.sequencer.SequencerTools \
  animation_toolset.toolsets.sequencer.SequencerTools.set_camera_cut_binding \
  --arguments '
{
  "camera_binding_id": "<value>",
  "section": {}
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
