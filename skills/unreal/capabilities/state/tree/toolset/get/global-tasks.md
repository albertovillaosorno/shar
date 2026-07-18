# Get global tasks

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
state_tree_toolset.toolsets.state_tree.StateTreeTools.get_global_tasks
```

Toolset:

```text
state_tree_toolset.toolsets.state_tree.StateTreeTools
```

## What this tool does

Returns global tasks that run across all states.

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
Use this tool to enumerate editor nodes for tasks that run across all states in
an editor-authored StateTree.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require a StateTree with valid StateTreeEditorData.
- Call `get_editor_data` first and preserve the asset unchanged during
  inspection.
- Treat returned nodes as editor structs whose identity belongs to the current
  asset state.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "state_tree": {
    "refPath": "/Game/AI/ST_Example.ST_Example"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The disposable StateTree constructor produced a valid asset without
StateTreeEditorData. This reader failed closed with `StateTree has no editor
data`, so no empty global-task result was fabricated.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Missing editor data is distinct from a valid empty global-task array.
- Global tasks are not tasks attached to one specific state.
- Returned editor-node structs can become stale after graph mutation or
  compilation.
- Use `get_node_description` only with nodes from the same unchanged StateTree.
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

### `state_tree`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  state_tree_toolset.toolsets.state_tree.StateTreeTools \
  state_tree_toolset.toolsets.state_tree.StateTreeTools.get_global_tasks \
  --arguments '
{
  "state_tree": {}
}
'
```

## Expected output

Global task editor nodes.

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
