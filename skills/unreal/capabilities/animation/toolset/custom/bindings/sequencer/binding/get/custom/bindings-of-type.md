# Get custom bindings of type

[Return to the central Unreal MCP index](../../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_bindings_of_type
```

Toolset:

```text
animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools
```

## What this tool does

Find all bindings of a given custom type in the current sequence.

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
Use this tool to find every binding in the current sequence whose custom-binding
class derives from a requested class.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require an open current sequence.
- Supply a reflected UMovieSceneCustomBinding class path discovered from live
  class metadata.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "binding_type_class":
    "/Script/MovieSceneTracks.MovieSceneSpawnableActorBinding"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles returned the CineCameraActor binding for its concrete class,
`MovieSceneSpawnableActorBindingBase`, and `MovieSceneCustomBinding`.
ReplaceableActorBinding returned `[]`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Matching is polymorphic: base classes include derived custom bindings.
- Results depend on the currently open sequence and contain session-specific
  GUIDs.
- A valid class with no matches returns `[]`.
- A missing class path raises instead of returning an empty list.
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

### `binding_type_class`

- Required: **yes**
- Type: `string`
- Purpose:

Full class path of the custom binding type.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools \
  animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_bindings_of_type \
  --arguments '
{
  "binding_type_class": "<value>"
}
'
```

## Expected output

List of matching SequencerBindingProxy objects.

### `returnValue`

- Required: **yes**
- Type: `array<object>`
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
