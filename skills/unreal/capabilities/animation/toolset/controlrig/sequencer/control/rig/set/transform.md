# Set transform

[Return to the central Unreal MCP index](../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_transform
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Set a transform value on a Control Rig control and optionally key it.

Uses ControlRigSequencerLibrary.set_local_control_rig_transform.

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
Use this tool to set and optionally key one Euler-transform Control Rig
control at an exact frame.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Open the exact disposable Level Sequence and derive the live Control Rig
  track and control name.
- Use a compatible bound skeletal actor and confirm the control type before
  mutation.
- Read the transform before and after the call and delete the whole disposable
  sequence fixture afterward.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "control_name": "Locator",
  "control_rig_asset_path": "/AnimatorKit/UtilityRigs/CRU_AddLocator",
  "frame": 20,
  "location_x": 120.0,
  "location_y": -35.0,
  "location_z": 55.0,
  "rotation_pitch": 10.0,
  "rotation_roll": -5.0,
  "rotation_yaw": 25.0,
  "sequence": {
    "refPath": "/Game/SHAR_MCP_Validation_RoundNext/LS_MCP_RoundNext_CR.LS_MCP_RoundNext_CR"
  },
  "set_key": true
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned true. `get_transform` changed from zero location and
rotation to location 120/-35/55 and rotation 10/25/-5 at frame 20.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Sequence, track, section, binding, and control identities become stale after
  reconstruction or deletion.
- `set_key: true` mutates channels as well as the evaluated control value.
- This validation covered the `Locator` Euler-transform control on
  `CRU_AddLocator`; rediscover controls for every other rig.
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
1. Capture pre-state and define an independent postcondition check.

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

Path to identify which CR (used to find the rig).

### `frame`

- Required: **yes**
- Type: `integer`
- Purpose:

The frame number.

### `location_x`

- Required: **no**
- Type: `number`
- Default: `0`
- Purpose:

X location.

### `location_y`

- Required: **no**
- Type: `number`
- Default: `0`
- Purpose:

Y location.

### `location_z`

- Required: **no**
- Type: `number`
- Default: `0`
- Purpose:

Z location.

### `rotation_pitch`

- Required: **no**
- Type: `number`
- Default: `0`
- Purpose:

Pitch rotation in degrees.

### `rotation_roll`

- Required: **no**
- Type: `number`
- Default: `0`
- Purpose:

Roll rotation in degrees.

### `rotation_yaw`

- Required: **no**
- Type: `number`
- Default: `0`
- Purpose:

Yaw rotation in degrees.

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `set_key`

- Required: **no**
- Type: `boolean`
- Default: `true`
- Purpose:

If True, set a keyframe at this frame.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_transform \
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
