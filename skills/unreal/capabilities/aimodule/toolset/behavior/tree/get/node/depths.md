# Get node depths

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_node_depths
```

Toolset:

```text
aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools
```

## What this tool does

Returns tree depths for all nodes, matching list_nodes order.

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
Use this tool to obtain a depth array aligned with the current SHAR
BehaviorTree flat-node order for hierarchy audits or deterministic reports.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve a current BehaviorTree asset reference.
- Read `list_nodes` from the same unchanged tree state.
- Require node and depth arrays to have identical lengths before pairing them.
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
The two-node disposable tree returned `[0, 1]` in two complete cycles. The
parallel node list was the root sequence followed by its RunBehavior task, and
single-index checks independently confirmed depths `0` and `1`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty tree validly returns an empty depth array.
- Entries have meaning only when paired with the same-state `list_nodes` result.
- Root decorators are emitted before the root composite and receive depth `0`.
- Structural mutation invalidates both ordering and the paired depth array.
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
  aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_node_depths \
  --arguments '
{
  "behavior_tree": {}
}
'
```

## Expected output

A list of depth integers.

### `returnValue`

- Required: **yes**
- Type: `array<integer>`
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
