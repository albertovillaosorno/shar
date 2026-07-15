# Get binding id

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.sequencer.SequencerTools.get_binding_id
```

Toolset:

```text
animation_toolset.toolsets.sequencer.SequencerTools
```

## What this tool does

Get the binding ID for a binding proxy.

The binding ID can be used with get_bound_objects to resolve what actor or
component the binding references at runtime.

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
Use this tool to convert a binding proxy into its MovieScene object-binding ID
for ID-based Sequencer operations.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact LevelSequence ref and a binding proxy from that same sequence.
- Verify the proxy name or tag before conversion.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "sequence": {"refPath": "/Game/LS_SHAR_MCP_Sequence_BindingFixture_1.LS_SHAR_MCP_Sequence_BindingFixture_1"},
  "binding": {
    "bindingId": "2F11F273-46E4-6DAB-E493-ECBADECB5346",
    "sequence": {"refPath": "/Game/LS_SHAR_MCP_Sequence_BindingFixture_1.LS_SHAR_MCP_Sequence_BindingFixture_1"}
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
In both camera cycles, the returned `guid` matched the binding proxy GUID.
`sequenceId` and `resolveParentIndex` were both `0` for the direct actor and
component proxies.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The returned object-binding ID is not a binding proxy.
- The sequence argument and proxy must refer to the same sequence.
- GUID values change across reconstructed fixtures.
- Preserve all returned fields when passing the ID to another endpoint.
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
shar-unreal-mcp describe animation_toolset.toolsets.sequencer.SequencerTools
```

1. Confirm every required input against the current schema.

## Inputs

### `binding`

- Required: **yes**
- Type: `object`
- Purpose:

MovieSceneBindingProxy

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.sequencer.SequencerTools \
  animation_toolset.toolsets.sequencer.SequencerTools.get_binding_id \
  --arguments '
{
  "binding": {},
  "sequence": {}
}
'
```

## Expected output

The MovieSceneObjectBindingID.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

MovieSceneObjectBindingID

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
