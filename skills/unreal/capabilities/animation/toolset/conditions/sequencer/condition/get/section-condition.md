# Get section condition

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.conditions.SequencerConditionTools.get_section_condition
```

Toolset:

```text
animation_toolset.toolsets.conditions.SequencerConditionTools
```

## What this tool does

Get the condition on a section.

Returns the class path of the condition, or an empty string if no condition is
set.

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
Use this tool to read the condition class authored on one exact MovieScene
section before SHAR evaluates platform, group, or director-driven gating.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact MovieSceneSection ref from `get_sections`.
- Treat an empty string as a valid unconditioned section.
- Verify the condition class through an authorized disposable round trip when
  positive evidence is required.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "section": {"refPath": "/Game/LS_SHAR_MCP_ConditionFixture_1.LS_SHAR_MCP_ConditionFixture_1:MovieScene_0.MovieSceneCameraCutTrack_0.MovieSceneCameraCutSection_0"}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two disposable camera-sequence cycles initially returned `""`, round-tripped
`/Script/MovieScene.MovieSceneGroupCondition`, and returned `""` again after
clearing. Platform and Director Blueprint conditions also round-tripped in the
probe cycle.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- No condition is represented by an empty string, not `None` or JSON null.
- A successful setter must still be verified through this read.
- Invalid condition classes raise during mutation.
- Passing a non-section ref fails during parameter translation.
- Condition configuration fields require separate inspection.
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
shar-unreal-mcp describe animation_toolset.toolsets.conditions.SequencerConditionTools
```

1. Confirm every required input against the current schema.

## Inputs

### `section`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.conditions.SequencerConditionTools \
  animation_toolset.toolsets.conditions.SequencerConditionTools.get_section_condition \
  --arguments '
{
  "section": {}
}
'
```

## Expected output

Class path of the condition, or empty string.

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
