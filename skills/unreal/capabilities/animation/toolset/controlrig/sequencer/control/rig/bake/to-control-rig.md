# Bake to control rig

[Return to the central Unreal MCP index](../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.bake_to_control_rig
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Bake existing animation on a binding into a Control Rig track.

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
Use this tool to convert an existing SHAR skeletal-animation binding into a
keyed Control Rig track for animation inspection, correction, or migration.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR editor world and target Level Sequence must be open.
- Resolve one skeletal binding with a real `SkeletalMeshComponent` and confirm
  that its source animation section references the intended AnimSequence.
- Use a Control Rig that is compatible with the same skeleton and contains an
  animation control plus a working backward-solve graph.
- Capture the binding's tracks before mutation and define removal of the
  disposable binding and actor as recovery.
- Decide whether dense keys are required; `reduce_keys: false` creates a key at
  every evaluated frame.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "sequence": {
    "refPath": "/Game/SHAR_MCP_Validation_CR53/LS_CR53.LS_CR53"
  },
  "binding": {
    "bindingId": "807BF359-4575-BE47-D436-7581091F055F",
    "sequence": {
      "refPath": "/Game/SHAR_MCP_Validation_CR53/LS_CR53.LS_CR53"
    }
  },
  "control_rig_asset_path": "/Game/SHAR_MCP_Validation_CustomRig53/CR_NedRoot53",
  "reduce_keys": false,
  "tolerance": 0.001,
  "reset_controls": true
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The source binding contained the imported Ned wave AnimSequence and no Control
Rig track. The call returned `true`. A separate track inspection found a new
`MovieSceneControlRigParameterTrack` with one section, nine `Root_CTRL`
transform channels, and 151 keys per channel covering frames 0 through 150.
The rotation channels independently read 90 degrees on X and 180 degrees on Z,
and the three scale channels remained approximately one. Removing the binding
returned `true`, and a fresh scene search confirmed that the disposable actor
was absent.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A compatible backward-solve graph is mandatory. Four AnimatorKit utility
  rigs returned `false`; the Control Rig root module returned `true` but
  produced a section with zero channels and therefore was not a valid bake.
- A `true` result alone is insufficient. Verify the new track, section, control
  names, channel count, frame range, and non-empty key arrays.
- Binding identifiers and generated track references are session-specific.
- `reduce_keys: false` can produce large dense key sets.
- A stationary source root can yield constant root-control values even though
  the bake is valid and every frame is keyed.
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

### `binding`

- Required: **yes**
- Type: `object`
- Purpose:

MovieSceneBindingProxy

### `control_rig_asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to the CR asset to bake into.

### `reduce_keys`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, perform key reduction after baking.

### `reset_controls`

- Required: **no**
- Type: `boolean`
- Default: `true`
- Purpose:

If True, reset all controls to initial value per frame.

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

Key reduction tolerance. Smaller values keep more keys.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools \
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.bake_to_control_rig \
  --arguments '
{
  "binding": {},
  "control_rig_asset_path": "<value>",
  "sequence": {}
}
'
```

## Expected output

True if baking succeeded.

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
