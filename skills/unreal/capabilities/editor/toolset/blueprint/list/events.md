# List events

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.list_events
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Lists all events visible on the Blueprint — locally defined custom events plus
inheritable events from the parent class chain and implemented interfaces.

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
Use this tool to inspect Blueprint events and distinguish implemented entries
from inherited events available to override.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact existing compiled Blueprint asset ref.
- Treat descriptions as editor guidance, not stable identifiers.
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
Two calls returned the same 22 records. `ReceiveTick` was implemented; inherited
radial-damage, point-damage, hit, and end-play entries were present but not
implemented. A missing Blueprint failed during parameter translation.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The list combines implemented and available inherited events.
- Descriptions are verbose and engine-version dependent.
- Use `name` as the identifier.
- Availability does not prove an event will fire.
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
  editor_toolset.toolsets.blueprint.BlueprintTools.list_events \
  --arguments '
{
  "blueprint": {}
}
'
```

## Expected output

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
