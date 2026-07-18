# Bake channel keys

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.keyframing.SequencerKeyframingTools.bake_channel_keys
```

Toolset:

```text
animation_toolset.toolsets.keyframing.SequencerKeyframingTools
```

## What this tool does

Bake a channel's values over a frame range.

Evaluates the channel curve at every frame in the range and returns the
computed values. Useful for extracting animation data or verifying
interpolation results.

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
Use this tool to sample a SHAR animation channel over a bounded frame range
for inspection or export planning.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live SequencerKeyframingTools schema.
- Open the exact disposable Level Sequence and discover channel names with
  `get_channel_names` instead of guessing.
- Capture keys, defaults, editor-open state, or selection before mutation and
  define whole-folder cleanup.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "channel_name": "Location.X",
  "end_frame": 5,
  "section": {
    "refPath": "/Game/SHAR_MCP_Validation_SeqRound_260718/LS_MCP_SeqRound_260718.LS_MCP_SeqRound_260718:MovieScene_0.MovieScene3DTransformTrack_3.MovieScene3DTransformSection_0"
  },
  "start_frame": 0
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`get_keys` independently returned float keys at frames 0 and 5; the bounded
bake call returned the observed value list `[1]`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Channel names are section-specific. Scalar typed sections in this fixture
  exposed the literal channel name `None`, while the transform section exposed
  names such as `Location.X`.
- Key readers return a JSON-encoded string, whereas selection and bake readers
  return native arrays; parse each output according to its live schema.
- In Unreal 5.8, the tested 0-through-5 range returned `[1]` rather than one
  value per integer frame, despite two readable source keys. Treat the result
  shape as native behavior to verify, not as an assumed inclusive sample
  count.
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
shar-unreal-mcp describe animation_toolset.toolsets.keyframing.SequencerKeyframingTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `channel_name`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the channel (e.g. 'Location.X').

### `end_frame`

- Required: **yes**
- Type: `integer`
- Purpose:

End of the evaluation range.

### `section`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `start_frame`

- Required: **yes**
- Type: `integer`
- Purpose:

Start of the evaluation range.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.keyframing.SequencerKeyframingTools \
  animation_toolset.toolsets.keyframing.SequencerKeyframingTools.bake_channel_keys \
  --arguments '
{
  "channel_name": "<value>",
  "end_frame": 0,
  "section": {},
  "start_frame": 0
}
'
```

## Expected output

List of float values, one per frame in the range.

### `returnValue`

- Required: **yes**
- Type: `array<number>`
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
