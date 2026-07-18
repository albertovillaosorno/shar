# Load anim into rig

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.load_anim_into_rig
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Load an animation sequence into a Control Rig section.

Finds the skeletal mesh component from the binding associated with the
section's track.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Confirm execution scope, cancellation behavior, and expected side effects
before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to insert one reviewed SHAR AnimSequence into an existing,
compatible Control Rig section at an explicit Sequencer start frame.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The section must belong to a Control Rig track whose binding resolves to a
  real `SkeletalMeshComponent`.
- The AnimSequence, skeletal mesh, and Control Rig must use compatible skeleton
  semantics and the rig must contain a working backward-solve mapping.
- Capture every target channel and its keys before invocation.
- Choose the start frame, reset behavior, reduction policy, and tolerance
  explicitly, and define binding and actor cleanup before testing.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "cr_section": {
    "refPath": "/Game/SHAR_MCP_Validation_CR53/LS_CR53.LS_CR53:MovieScene_0.MovieSceneControlRigParameterTrack_9.MovieSceneControlRigParameterSection_0"
  },
  "anim_sequence_path": "/Game/SHAR_MCP_Validation_Anim53/SK_Ned_53_Anim_ndr_wave",
  "start_frame": 20,
  "reset_controls": true,
  "key_reduce": false,
  "tolerance": 0.001
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Before invocation, all nine `Root_CTRL` transform channels were empty. The
call returned `true`. Independent key reads then found 29 keys on every channel
from frame 20 through frame 48. The loaded values included location
`x: 0.995`, `y: -0.46`, `z: -0.72`, rotation Z at 360 degrees, and unit scale.
The binding removal returned `true`, and a fresh scene search confirmed that
the disposable skeletal actor was absent.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The owning binding must resolve a `SkeletalMeshComponent`; a no-mesh
  SkeletalMeshActor binding was rejected before mutation.
- Rig compatibility is semantic, not merely type-based. `CRU_AddLocator`
  returned `false` and created no keys for the same Ned animation.
- `start_frame` offsets the generated key range.
- `key_reduce: false` creates dense per-frame keys.
- A successful call still requires independent channel-name, key-count, frame-
  range, and value inspection.
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

### `anim_sequence_path`

- Required: **yes**
- Type: `string`
- Purpose:

Content path to the AnimSequence asset.

### `cr_section`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `key_reduce`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, perform key reduction.

### `reset_controls`

- Required: **no**
- Type: `boolean`
- Default: `true`
- Purpose:

If True, reset controls to initial value per frame.

### `start_frame`

- Required: **no**
- Type: `integer`
- Default: `0`
- Purpose:

Frame at which to insert the animation.

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
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.load_anim_into_rig \
  --arguments '
{
  "anim_sequence_path": "<value>",
  "cr_section": {}
}
'
```

## Expected output

True if the load succeeded.

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
