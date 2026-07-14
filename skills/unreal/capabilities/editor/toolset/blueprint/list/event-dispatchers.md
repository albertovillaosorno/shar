# List event dispatchers

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.list_event_dispatchers
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Lists all event dispatchers defined on a Blueprint.

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
Use this tool to enumerate event dispatchers declared by a SHAR Blueprint before
binding or dispatcher-graph inspection.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact existing Blueprint asset ref.
- Treat an empty result as a valid declared-dispatcher baseline.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "blueprint": {
    "refPath": "/Game/B_SHAR_MCP_Blueprint_ReadFixture.B_SHAR_MCP_Blueprint_ReadFixture"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned an empty array. The same Blueprint had two ordinary graphs
and 22 visible events, proving the empty result was dispatcher-specific. A
missing Blueprint failed during parameter translation.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Empty means no Blueprint-declared dispatcher graph was found.
- Events and event dispatchers are separate concepts.
- Structural edits can replace dispatcher graph refs.
- Component events and inherited delegates are not equivalent dispatchers.
- Missing Blueprint refs raise a native error.
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
shar-unreal-mcp describe editor_toolset.toolsets.blueprint.BlueprintTools
```

1. Confirm every required input against the current schema.

## Inputs

### `blueprint`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.list_event_dispatchers \
  --arguments '
{
  "blueprint": {}
}
'
```

## Expected output

A list of event dispatcher graphs, one per dispatcher.

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
