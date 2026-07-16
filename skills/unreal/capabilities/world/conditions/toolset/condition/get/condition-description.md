# Get condition description

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
WorldConditionsToolset.WorldConditionTools.GetConditionDescription
```

Toolset:

```text
WorldConditionsToolset.WorldConditionTools
```

## What this tool does

Returns a human-readable description of a single world condition. The condition
must be passed as an FInstancedStruct containing an FWorldConditionBase-derived
struct.

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
Use this tool to render a reflected World Condition into diagnostic text before
SHAR records or reviews a condition used by a Smart Object, StateTree, gameplay
interaction, or another world-condition query. Obtain the instanced struct from
live editor data rather than reconstructing gameplay conditions by hand.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- The supplying plugin or module must be loaded so `_structType` resolves.
- Pass an `FInstancedStruct` containing a reflected `FWorldConditionBase`
  descendant.
- Preserve the source structure separately because the returned text is only a
  human-readable description.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "condition": {
    "_structType": "/Script/WorldConditionsTestSuite.WorldConditionTest"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned `Value == 0`. An empty instanced struct returned
an empty string. A valid non-condition struct also returned an empty string,
while an unknown `_structType` failed during input conversion.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The result is descriptive text, not serialized condition data and not an
  evaluation result.
- Empty or wrong-base reflected structs can return an empty string instead of an
  error; callers must verify the source struct type independently.
- An unknown `_structType` raises an input-conversion error.
- Condition-specific fields supplied outside the live reflected schema were
  ignored by the verified fixture. Do not construct production conditions from
  guessed JSON; pass structures obtained from authoritative editor data.
- Descriptions can change with engine or plugin implementations and must not be
  used as stable identifiers.
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
shar-unreal-mcp describe WorldConditionsToolset.WorldConditionTools
```

1. Confirm every required input against the current schema.

## Inputs

### `condition`

- Required: **yes**
- Type: `object`
- Purpose:

The instanced struct holding the world condition.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  WorldConditionsToolset.WorldConditionTools \
  WorldConditionsToolset.WorldConditionTools.GetConditionDescription \
  --arguments '
{
  "condition": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

Text description of the condition, or empty if invalid.

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
