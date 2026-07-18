# Remove comment box

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
DataflowAgent.DataflowAgentToolset.RemoveCommentBox
```

Toolset:

```text
DataflowAgent.DataflowAgentToolset
```

## What this tool does

Removes a comment box node from the graph.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Capture pre-state, bound the target set, and verify the resulting editor or
asset state through an independent read.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to remove one exact Dataflow comment box without deleting its
enclosed nodes.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live Dataflow Agent schema.
- Use one unique disposable graph or compatible asset and confirm every class,
  node, or template identity before mutation.
- Capture graph structure, embedded graph properties, asset class, and
  whole-folder cleanup as the postcondition plan.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "commentId": "DCF1E0654D75AF3D851EE8814C6EEFB1",
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/DF_MCP_Round50.DF_MCP_Round50"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned true for the exact comment ID, while graph structure still
contained both original nodes with unchanged names and positions.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Dataflow graph, node, comment, embedded graph, and compatible-asset
  references become stale after deletion.
- Only live-listed compatible classes and template IDs are valid; do not infer
  identifiers from display text.
- The graph reader confirms node preservation but does not provide a direct
  remaining-comment inventory.
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
shar-unreal-mcp describe DataflowAgent.DataflowAgentToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `commentId`

- Required: **yes**
- Type: `string`
- Purpose:

The node GUID string returned by AddCommentBox

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

The Dataflow asset containing the comment

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  DataflowAgent.DataflowAgentToolset \
  DataflowAgent.DataflowAgentToolset.RemoveCommentBox \
  --arguments '
{
  "commentId": "<value>",
  "graph": {}
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
