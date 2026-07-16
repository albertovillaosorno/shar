# Get custom binding type

<!-- markdownlint-disable-next-line MD013 -->
[Return to the central Unreal MCP index](../../../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_binding_type
```

Toolset:

```text
animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools
```

## What this tool does

Get the custom binding class for a binding.

Returns the class path of the custom binding type, or an empty string for
standard possessable bindings.

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
Use this tool to distinguish modern custom MovieScene bindings from standard
possessable component bindings before SHAR performs binding-type-specific
operations.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Rediscover the binding proxy from the active LevelSequence.
- Keep its sequence and GUID paired.
- Treat an empty string as a valid standard or invalid-binding result.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "binding": {
    "bindingId": "00000000-0000-0000-0000-000000000000",
    "sequence": {
      "refPath": "/Game/LS_SHAR_MCP_CustomBindingFixture_1.LS_SHAR_MCP_CustomBindingFixture_1"
    }
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two disposable camera cycles returned
`/Script/MovieSceneTracks.MovieSceneSpawnableActorBinding` for CineCameraActor
and `""` for CameraComponent. A zero-GUID proxy also returned `""`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Empty string is ambiguous between a standard possessable and an invalid proxy.
- Verify the binding through name, ID, or object reads before interpreting
  absence.
- Binding GUIDs change when a sequence is rebuilt.
- Custom-binding class paths can vary with engine and plugin versions.
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
shar-unreal-mcp describe animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools
```

1. Confirm every required input against the current schema.

## Inputs

### `binding`

- Required: **yes**
- Type: `object`
- Purpose:

MovieSceneBindingProxy

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools \
  animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_binding_type \
  --arguments '
{
  "binding": {}
}
'
```

## Expected output

Class path string, or empty string.

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
