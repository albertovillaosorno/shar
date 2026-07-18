# Set default value

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.keyframing.SequencerKeyframingTools.set_default_value
```

Toolset:

```text
animation_toolset.toolsets.keyframing.SequencerKeyframingTools
```

## What this tool does

Set the default value of a channel.

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
Use this tool to set the fallback value of a SHAR channel before adding or
removing keys.
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
  "section": {
    "refPath": "/Game/SHAR_MCP_Validation_SeqRound_260718/LS_MCP_SeqRound_260718.LS_MCP_SeqRound_260718:MovieScene_0.MovieScene3DTransformTrack_3.MovieScene3DTransformSection_0"
  },
  "value": 24.5
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The default-value getter changed from 0 to 24.5 for `Location.X`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Channel names are section-specific. Scalar typed sections in this fixture
  exposed the literal channel name `None`, while the transform section exposed
  names such as `Location.X`.
- Key readers return a JSON-encoded string, whereas selection and bake readers
  return native arrays; parse each output according to its live schema.
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

Name of the channel.

### `section`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `value`

- Required: **yes**
- Type: `number`
- Purpose:

The default value.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.keyframing.SequencerKeyframingTools \
  animation_toolset.toolsets.keyframing.SequencerKeyframingTools.set_default_value \
  --arguments '
{
  "channel_name": "<value>",
  "section": {},
  "value": 0.0
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
