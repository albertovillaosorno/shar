# Set byte track enum

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.sequencer.SequencerTools.set_byte_track_enum
```

Toolset:

```text
animation_toolset.toolsets.sequencer.SequencerTools
```

## What this tool does

Configure a byte track to use a specific enum type.

Byte tracks can animate enum properties. Call this after adding the track and
before setting property_name_and_path.

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
Use this tool to assign a UEnum to a MovieSceneByteTrack only after the
installed UE object-loading call is compatible with UE 5.8.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve a real MovieSceneByteTrack from the intended sequence.
- Resolve the exact UEnum object path and capture the track property
  configuration.
- Use a disposable track until the wrapper object-loading signature is fixed.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "track": {
    "refPath": "/Game/Cinematics/LS_Example.LS_Example:MovieScene_0.MovieSceneByteTrack_0"
  },
  "enum_class_path": "/Script/Engine.EAnimationMode"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A disposable byte track reached this wrapper. UE 5.8 reported `load_object()
missing required argument outer (pos 1)` because the wrapper calls
`unreal.load_object(name=...)` without the required outer.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The installed wrapper cannot load the enum object under the UE 5.8 signature.
- The enum must be compatible with the property animated by the byte track.
- Verify the assigned enum and property path independently after any future
  successful call.
- Do not continue to key authoring when enum assignment failed.
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

### `enum_class_path`

- Required: **yes**
- Type: `string`
- Purpose:

Path to the enum (e.g. the result of unreal.AnimationMode.static_enum()).

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
  animation_toolset.toolsets.sequencer.SequencerTools.set_byte_track_enum \
  --arguments '
{
  "enum_class_path": "<value>",
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
