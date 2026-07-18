# Get root decorators

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_root_decorators
```

Toolset:

```text
aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools
```

## What this tool does

Returns root-level decorators on this tree.

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
Use this tool to inspect decorators attached at the BehaviorTree root before
validating SHAR-wide entry conditions or root gating logic.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve a current BehaviorTree asset reference.
- Distinguish root decorators from decorators attached to individual child
  branches.
- Treat an empty collection as a valid tree state.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "behavior_tree": {
    "refPath": "/Game/SHAR_MCP_Validation_Behavior56/BT_Empty56.BT_Empty56"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The disposable tree intentionally had no root decorators. Two complete cycles
returned `[]`, while the same non-empty tree independently returned its root
sequence, RunBehavior child, depths, blackboard, and subtree.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty array means no root-level decorators; it does not mean the tree has
  no nodes or no child-branch decorators.
- Per-child decorators appear in the `list_nodes` traversal instead.
- Refresh the result after changing root decorator configuration.
- Returned UObject refs are session and asset specific.
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

### `behavior_tree`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools \
  aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_root_decorators \
  --arguments '
{
  "behavior_tree": {}
}
'
```

## Expected output

A list of BTDecorator UObjects.

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
