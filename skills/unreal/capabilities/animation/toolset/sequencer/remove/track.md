# Remove track

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.sequencer.SequencerTools.remove_track
```

Toolset:

```text
animation_toolset.toolsets.sequencer.SequencerTools
```

## What this tool does

Remove a track from a binding.

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
Use this tool to remove track during a bounded SHAR mission, dialogue, camera,
or cinematic Level Sequence lifecycle.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live SequencerTools schema.
- Open the exact disposable or recoverable Level Sequence and capture the
  matching independent reader before mutation.
- Resolve the nested object from the same sequence and rediscover it after
  structural edits that can stale references.
- Define the inverse operation or whole disposable-asset cleanup before
  invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "binding": {
    "bindingId": "1D8F3629-497A-6373-E3C4-9799962DEA56",
    "sequence": {
      "refPath": "/Game/SHAR_MCP_Validation_Replacement_2ce1ae2e/LS_MCP_Replacement_2ce1ae2e.LS_MCP_Replacement_2ce1ae2e"
    }
  },
  "track": {
    "refPath": "/Game/SHAR_MCP_Validation_Replacement_2ce1ae2e/LS_MCP_Replacement_2ce1ae2e.LS_MCP_Replacement_2ce1ae2e:MovieScene_0.MovieScene3DTransformTrack_2"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`get_tracks_on_binding` changed from 3 items to 2 items after the exact
removal in the disposable SHAR sequence. The removed identity no longer
appeared in the independent inventory.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Sequence and nested object references are live editor identities and can
  become stale after structural edits or closing Sequencer.
- Removal invalidates the removed nested reference immediately; do not reuse
  it in later calls.
- This validation used a disposable sequence and exact whole-folder cleanup.
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

### `binding`

- Required: **yes**
- Type: `object`
- Purpose:

MovieSceneBindingProxy

### `track`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.sequencer.SequencerTools \
  animation_toolset.toolsets.sequencer.SequencerTools.remove_track \
  --arguments '
{
  "binding": {},
  "track": {}
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
