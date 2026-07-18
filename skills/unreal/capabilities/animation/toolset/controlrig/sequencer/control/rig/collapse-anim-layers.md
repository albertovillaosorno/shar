# Collapse anim layers

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.collapse_anim_layers
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Collapse all sections and layers on a Control Rig track into one section.

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
Use this tool to request collapse of the active SHAR Control Rig track after a
reviewed layer-editing pass.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live SequencerControlRigTools schema.
- Open the exact disposable Level Sequence, discover the Control Rig track and
  section from live returns, and validate every control name and type with
  `get_controls_info`.
- Capture the matching value, mask, selection, key, layer, or space reader
  before mutation and define whole-folder deletion.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "control_rig_asset_path": "/Game/SHAR_MCP_Validation_ControlRig_Large_260718/CR_MCP_Large_260718",
  "reduce_keys": false,
  "sequence": {
    "refPath": "/Game/SHAR_MCP_Validation_ControlRig_Large_260718/LS_MCP_Large_260718.LS_MCP_Large_260718"
  },
  "tolerance": 0.001
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned true and the Control Rig track remained present and
non-layered. The editor layer inventory still contained `Base Layer` and the
merged layer.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Sequence, Control Rig track, section, binding, and control identities are
  live editor references and become stale after structural edits, closing
  Sequencer, or deleting the asset.
- Validate every control name and type through `get_controls_info`; a Boolean
  return or numeric value alone does not prove type compatibility.
- Animation-layer indices are active-editor identities and change after
  duplicate, reorder, merge, or delete operations.
- `get_anim_layers` returns JSON text and requires a second parse.
- A true return did not remove the named editor-layer inventory in this
  fixture; it preserved `Base Layer` and the merged layer.
- Verify sections, track mode, and layer inventory separately after collapse.
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

### `control_rig_asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to identify which Control Rig.

### `reduce_keys`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, perform key reduction after collapsing.

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `tolerance`

- Required: **no**
- Type: `number`
- Default: `0.001`
- Purpose:

Key reduction tolerance.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.collapse_anim_layers \
  --arguments '
{
  "control_rig_asset_path": "<value>",
  "sequence": {}
}
'
```

## Expected output

True if collapsing succeeded.

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
