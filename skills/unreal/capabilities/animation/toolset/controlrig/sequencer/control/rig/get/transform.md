# Get transform

[Return to the central Unreal MCP index](../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_transform
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Get the transform value of a Control Rig control at a frame.

Automatically detects whether the control is a Transform or EulerTransform type
and uses the appropriate API.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Use the returned structured evidence directly, but still confirm the live
schema because names do not prove side effects.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to read location and rotation from a Transform-compatible Control
Rig control at an exact Sequencer frame.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The LevelSequence must contain the requested Control Rig track.
- Discover the exact control name and type through `get_controls_info`.
- Read the exact authored frame rather than relying on the playhead.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "sequence": {
    "refPath": "/Game/LS_SHAR_MCP_TransformRead_2.LS_SHAR_MCP_TransformRead_2"
  },
  "control_rig_asset_path": "/Game/CR_SHAR_MCP_TransformRead_2",
  "control_name": "EulerControl",
  "frame": 24
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles keyed EulerControl to location `(-1, -2, -3)` and rotation `(10, 20,
30)` at frame 24. The generic getter returned those fields and intentionally
omitted scale.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The return value is JSON text and requires a second parse.
- Missing controls return an identity location and rotation rather than raising.
- The current proof used an EULER_TRANSFORM control.
- Keying a native Transform control through `set_transform` failed because
  Unreal could not pythonize RigControlType value `7`.
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
shar-unreal-mcp describe animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

1. Confirm every required input against the current schema.

## Inputs

### `control_name`

- Required: **yes**
- Type: `string`
- Purpose:

The control name (e.g. 'body_ctrl').

### `control_rig_asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to identify which CR.

### `frame`

- Required: **yes**
- Type: `integer`
- Purpose:

The frame number.

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_transform \
  --arguments '
{
  "control_name": "<value>",
  "control_rig_asset_path": "<value>",
  "frame": 0,
  "sequence": {}
}
'
```

## Expected output

JSON string with 'location' and 'rotation' dicts.

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Confirm the response belongs to the open editor project.
- Reject evidence derived from stale discovery state.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
