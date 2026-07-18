# Get children

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
state_tree_toolset.toolsets.state_tree.StateTreeTools.get_children
```

Toolset:

```text
state_tree_toolset.toolsets.state_tree.StateTreeTools
```

## What this tool does

Returns child states of a state.

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
Use this tool to enumerate direct child states for a StateTreeState returned by
the same editor-authored StateTree.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the state from `get_root_states` or `get_children` on a StateTree with
  editor data.
- Keep the StateTree unchanged while using the returned StateTreeState
  reference.
- Do not pass the StateTree asset itself where a StateTreeState is required.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "state": {
    "refPath": "/Game/AI/ST_Example.ST_Example:StateTreeState_0"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A disposable StateTree asset was intentionally supplied as the `state` argument.
Parameter translation failed closed with `not valid StateTreeState for property
state`, confirming strict state-object typing.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The example shows the required object-reference shape; resolve a real state
  before use.
- A StateTree asset and a StateTreeState object are not interchangeable.
- The result is direct children only and does not recursively traverse
  descendants.
- Re-resolve state refs after graph mutation, compilation, reload, or undo.
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
shar-unreal-mcp describe state_tree_toolset.toolsets.state_tree.StateTreeTools
```

1. Confirm every required input against the current schema.

## Inputs

### `state`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  state_tree_toolset.toolsets.state_tree.StateTreeTools \
  state_tree_toolset.toolsets.state_tree.StateTreeTools.get_children \
  --arguments '
{
  "state": {}
}
'
```

## Expected output

Child states.

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
