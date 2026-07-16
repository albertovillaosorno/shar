# Find or create track

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.find_or_create_track
```

Toolset:

```text
animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools
```

## What this tool does

Add a Control Rig track to a binding using a Control Rig asset.

This is the standard way to add a Control Rig to Sequencer. Uses
ControlRigSequencerLibrary.find_or_create_control_rig_track.

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
Use this mutation to attach an authored Control Rig to a skeletal binding in a
disposable or SHAR LevelSequence before control discovery, keying, or layered
animation work.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Treat this as a mutation despite the generated read-only posture.
- Open the target LevelSequence and use a live MovieSceneBindingProxy.
- Assign a compatible SkeletalMesh to the bound component first.
- Verify the Control Rig asset exists and supports the requested layered mode.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "sequence": {
        "refPath": (
            "/Game/LS_SHAR_MCP_FindTrackExample."
            "LS_SHAR_MCP_FindTrackExample"
        )
    },
    "binding": {
        "bindingId": "C55E5062-409C-9255-B2EC-E9B96622697B",
        "sequence": {
            "refPath": (
                "/Game/LS_SHAR_MCP_FindTrackExample."
                "LS_SHAR_MCP_FindTrackExample"
            )
        },
    },
    "control_rig_asset_path": "/AnimatorKit/UtilityRigs/CRU_AddLocator",
    "is_layered": False,
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated non-layered CRU_AddLocator calls returned the same track and left
exactly one Control Rig parameter track. Layered AddControl calls were also
idempotent and remained layered when later called with `is_layered: false`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This tool mutates the sequence; the generated operational posture is
  inaccurate.
- The return value is JSON text and requires a second parse.
- `is_layered` is creation-time-only and does not convert an existing track.
- Layered creation is rig-dependent: AddControl succeeded, while CRU_AddLocator
  and CRD_SculptDeformer failed.
- Missing Control Rig assets raise explicitly.
- Binding GUIDs and nested track paths change when the sequence is rebuilt.
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

### `binding`

- Required: **yes**
- Type: `object`
- Purpose:

MovieSceneBindingProxy

### `control_rig_asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to the CR asset (e.g. '/Game/Rigs/CR_Mannequin_Body').

### `is_layered`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, create as a layered Control Rig.

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
  animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.find_or_create_track \
  --arguments '
{
  "binding": {},
  "control_rig_asset_path": "<value>",
  "sequence": {}
}
'
```

## Expected output

JSON string with 'track' refPath and 'control_rig' info.

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
