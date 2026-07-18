# Snap control rig

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.snap_control_rig
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Snap Control Rig controls to a target actor over a frame range.

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
Use this tool to snap selected Control Rig controls to one level actor only
after the installed UE wrapper matches the current snap API.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Open the sequence and resolve the exact Control Rig and control names.
- Use a uniquely labeled target actor and capture its transform plus all target
  control transforms.
- Use a disposable frame range until the UE 5.8 wrapper defect is fixed.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "sequence": {
    "refPath": "/Game/Cinematics/LS_Example.LS_Example"
  },
  "control_rig_asset_path": "/Game/ControlRigs/CR_Example.CR_Example",
  "control_names": [
    "Root_CTRL"
  ],
  "target_actor_name": "MCP_SnapTarget",
  "start_frame": 0,
  "end_frame": 30,
  "keep_offset": false,
  "snap_position": true,
  "snap_rotation": true,
  "snap_scale": false
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two disposable fixtures reached the snap wrapper, but UE 5.8 reported that
`ControlRigSnapperSelection` has no `control_rig` attribute. Independent
control-transform reads confirmed no accepted snap.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The installed wrapper targets a selection field absent from UE 5.8.
- Target matching uses actor label or name substring and can select the wrong
  actor when labels overlap.
- Verify position, rotation, scale, key counts, and frame bounds independently.
- Retry only after the upstream selection construction is updated for UE 5.8.
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

### `control_names`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Names of controls to snap.

### `control_rig_asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to identify which Control Rig.

### `end_frame`

- Required: **yes**
- Type: `integer`
- Purpose:

End frame of the snap range.

### `keep_offset`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, maintain the initial offset.

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `snap_position`

- Required: **no**
- Type: `boolean`
- Default: `true`
- Purpose:

If True, snap position.

### `snap_rotation`

- Required: **no**
- Type: `boolean`
- Default: `true`
- Purpose:

If True, snap rotation.

### `snap_scale`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, snap scale.

### `start_frame`

- Required: **yes**
- Type: `integer`
- Purpose:

Start frame of the snap range.

### `target_actor_name`

- Required: **yes**
- Type: `string`
- Purpose:

Name or label of the target actor in the level.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.snap_control_rig \
  --arguments '
{
  "control_names": [],
  "control_rig_asset_path": "<value>",
  "end_frame": 0,
  "sequence": {},
  "start_frame": 0,
  "target_actor_name": "<value>"
}
'
```

## Expected output

True if the snap succeeded.

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
