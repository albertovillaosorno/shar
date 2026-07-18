# Get node description

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
state_tree_toolset.toolsets.state_tree.StateTreeTools.get_node_description
```

Toolset:

```text
state_tree_toolset.toolsets.state_tree.StateTreeTools
```

## What this tool does

Returns a human-readable description for a node.

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
Use this tool to render the editor-provided human description for a
StateTreeEditorNode returned by the same StateTree.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require a StateTree with valid StateTreeEditorData.
- Obtain the node struct from `get_tasks`, `get_enter_conditions`,
  `get_global_tasks`, or `get_evaluators`.
- Preserve the full polymorphic node struct and keep the asset unchanged before
  description.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "state_tree": {
    "refPath": "/Game/AI/ST_Example.ST_Example"
  },
  "node": {
    "node": {
      "_structType": "/Script/StateTreeModule.StateTreeTaskBase"
    },
    "instance": {},
    "instanceObject": {
      "refPath": "None"
    },
    "executionRuntimeData": {},
    "executionRuntimeDataObject": {
      "refPath": "None"
    },
    "iD": "00000000-0000-0000-0000-000000000000",
    "expressionIndent": 0,
    "expressionOperand": "Copy"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The live schema accepted the complete StateTreeEditorNode shape. The disposable
StateTree then failed at the prerequisite with `StateTree has no editor data`,
proving the description is editor-data-dependent.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A fabricated or partial node struct is not a substitute for a node returned by
  the same StateTree.
- Polymorphic fields depend on `_structType` and can be lost by lossy
  reconstruction.
- The returned text is presentation guidance, not a stable node identity.
- Re-resolve the node and description after graph mutation, compile, reload, or
  undo.
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

### `node`

- Required: **yes**
- Type: `object`
- Purpose:

StateTreeEditorNode

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
  state_tree_toolset.toolsets.state_tree.StateTreeTools.get_node_description \
  --arguments '
{
  "node": {},
  "state_tree": {}
}
'
```

## Expected output

Formatted description string.

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
