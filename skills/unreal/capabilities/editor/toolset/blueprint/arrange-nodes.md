# Arrange nodes

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.arrange_nodes
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Arranges a list of nodes in a readable left-to-right layout.

Organizes nodes into columns based on data/execution flow, with producer nodes
to the left of the nodes they feed into. Call this after building a graph to
avoid nodes overlapping. Connections to nodes outside the list are used as
anchors so the arranged nodes integrate cleanly with the rest of the graph.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this mutation to lay out a bounded set of connected SHAR Blueprint nodes
left to right after graph structure is already correct.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass an explicit bounded list of current node references.
- Read the pre-arrangement positions and connections.
- Arrange only nodes owned by the current graph-editing task.
- Re-read every node after the operation and compile separately.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "nodes": [
    {
      "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_NodeLifecycle.BP_MCP_NodeLifecycle:EventGraph.K2Node_IfThenElse_0"
    },
    {
      "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_NodeLifecycle.BP_MCP_NodeLifecycle:EventGraph.K2Node_IfThenElse_1"
    }
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A connected two-Branch flow began at `(-320, 512)` and `(160, 240)`. The
mutation returned `null`. `get_node_infos` then reported distinct aligned
positions `(-320, 612)` and `(-20, 612)`, with the producer node to the left of
the consumer node.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The operation has no structured return value.
- Layout output depends on graph connections and external anchor nodes.
- Exact coordinates should not be treated as a stable engine-wide constant.
- Manual positions can change even when graph behavior does not.
- Read all affected nodes after arrangement and compile separately.
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
shar-unreal-mcp describe editor_toolset.toolsets.blueprint.BlueprintTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `nodes`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The nodes to arrange.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.arrange_nodes \
  --arguments '
{
  "nodes": []
}
'
```

## Expected output

The live interface does not declare a structured output schema.

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Verify changed state through a separate read or inspection.
- Use another capability to confirm the postcondition.
- Inspect editor logs when state is not directly observable.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
