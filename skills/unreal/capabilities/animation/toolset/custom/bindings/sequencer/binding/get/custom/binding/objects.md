# Get custom binding objects

[Return to the central Unreal MCP index](../../../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_binding_objects
```

Toolset:

```text
animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools
```

## What this tool does

Get the custom binding instances for a binding.

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
Use this tool to resolve the UMovieSceneCustomBinding objects attached to one
exact binding proxy.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a binding whose type was checked with `get_custom_binding_type`.
- Keep the owning sequence open while resolving session objects.
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
In two cycles, the camera actor returned one UObject whose path ended
`_CustomBinding_0`. CameraComponent and a zero-GUID proxy returned `[]`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Custom-binding UObject paths contain fixture- and actor-specific identities
  and should not be persisted.
- Empty is valid for standard or invalid bindings.
- Results are not the actors resolved by `get_bound_objects`.
- Returned objects can become stale after sequence reconstruction.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

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
  animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_binding_objects \
  --arguments '
{
  "binding": {}
}
'
```

## Expected output

List of UMovieSceneCustomBinding objects.

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
