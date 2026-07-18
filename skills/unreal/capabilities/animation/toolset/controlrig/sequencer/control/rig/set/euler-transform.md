# Set euler transform

[Return to the central Unreal MCP index](../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_euler_transform
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Set an EulerTransform control value at a specific frame.

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
Use this tool to set one typed Euler-transform Control Rig value at an exact
SHAR sequence frame.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live SequencerControlRigTools schema.
- Open the disposable Level Sequence, derive the track and section from live
  returns, and validate the control through `get_controls_info`.
- Capture the matching getter and define whole-folder deletion.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "control_name": "EulerControl",
  "control_rig_asset_path": "/Game/SHAR_MCP_Validation_ControlRig_Large_260718/CR_MCP_Large_260718",
  "frame": 24,
  "location_x": 4.0,
  "location_y": 5.0,
  "location_z": 6.0,
  "rotation_pitch": 10.0,
  "rotation_roll": 30.0,
  "rotation_yaw": 20.0,
  "scale_x": 1.1,
  "scale_y": 1.2,
  "scale_z": 1.3,
  "sequence": {
    "refPath": "/Game/SHAR_MCP_Validation_ControlRig_Large_260718/LS_MCP_Large_260718.LS_MCP_Large_260718"
  },
  "set_key": true
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The dedicated getter returned location 4/5/6, rotation 10/20/30, and scale
approximately 1.1/1.2/1.3 at frame 24.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Sequence, Control Rig track, section, binding, and control identities are
  live editor references and become stale after structural edits, closing
  Sequencer, or deleting the asset.
- Validate every control name and type through `get_controls_info`; a Boolean
  return or numeric value alone does not prove type compatibility.
- Dedicated getters proved evaluated values. Ordinary channel readers did not
  reliably prove that `set_key: true` inserted a durable key; use explicit
  keying tools when key creation is required.
- Scale and rotation values use engine floating-point representations; compare
  with tolerance.
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

Name of the EulerTransform control.

### `control_rig_asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to identify which Control Rig.

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

Pitch in degrees.

### `rotation_roll`

- Required: **no**
- Type: `number`
- Default: `0`
- Purpose:

Roll in degrees.

### `rotation_yaw`

- Required: **no**
- Type: `number`
- Default: `0`
- Purpose:

Yaw in degrees.

### `scale_x`

- Required: **no**
- Type: `number`
- Default: `1`
- Purpose:

X scale.

### `scale_y`

- Required: **no**
- Type: `number`
- Default: `1`
- Purpose:

Y scale.

### `scale_z`

- Required: **no**
- Type: `number`
- Default: `1`
- Purpose:

Z scale.

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
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_euler_transform \
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
