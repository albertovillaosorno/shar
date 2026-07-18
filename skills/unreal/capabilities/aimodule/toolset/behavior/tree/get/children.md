# Get children

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_children
```

Toolset:

```text
aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools
```

## What this tool does

Returns direct child nodes of a composite node.

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
Use this tool to enumerate the direct task or composite children of a SHAR
BehaviorTree composite while validating branch structure.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain a current composite-node reference from `list_nodes`.
- Confirm the target is a `BTCompositeNode`; an arbitrary task or decorator does
  not satisfy the live input type.
- Do not mutate the tree between resolving and querying the composite.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "composite": {
    "refPath": "/Game/SHAR_MCP_Validation_Behavior56/BT_Empty56.BT_Empty56:MCP_RootSequence56"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The disposable root sequence contained one RunBehavior task. Two complete
cycles returned exactly one direct child,
`/Game/SHAR_MCP_Validation_Behavior56/BT_Empty56.BT_Empty56:MCP_RunBehavior56`.
The flat node list contained the same root and task at depths `[0, 1]`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty array is valid for a composite with no direct children.
- The result contains direct children only; it is not a recursive subtree walk.
- Services and decorators are not returned as child nodes by this tool.
- Structural edits can invalidate refs; refresh `list_nodes` after mutation.
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
shar-unreal-mcp describe aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools
```

1. Confirm every required input against the current schema.

## Inputs

### `composite`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools \
  aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_children \
  --arguments '
{
  "composite": {}
}
'
```

## Expected output

A list of child BTNode UObjects.

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
