# Add comment box

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
DataflowAgent.DataflowAgentToolset.AddCommentBox
```

Toolset:

```text
DataflowAgent.DataflowAgentToolset
```

## What this tool does

Adds a comment box around the given nodes.

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
Use this tool to group reviewed Dataflow nodes under one task-owned editor
comment box.
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
  "color": {
    "a": 1.0,
    "b": 0.6,
    "g": 0.4,
    "r": 0.2
  },
  "comment": "MCP Round 50",
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/DF_MCP_Round50.DF_MCP_Round50"
  },
  "nodes": [
    {
      "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/DF_MCP_Round50.DF_MCP_Round50:MCP_Add"
    },
    {
      "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/DF_MCP_Round50.DF_MCP_Round50:MCP_Multiply"
    }
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned comment ID `DCF1E0654D75AF3D851EE8814C6EEFB1`. The matching
remove call accepted that ID, and the two enclosed math nodes remained
unchanged.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Dataflow graph, node, comment, embedded graph, and compatible-asset
  references become stale after deletion.
- Only live-listed compatible classes and template IDs are valid; do not infer
  identifiers from display text.
- Dataflow graph structure does not expose comment boxes; retain the returned
  comment ID and prove it through the matching remove operation.
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

### `color`

- Required: **no**
- Type: `object`
- Default: `{"r":1,"g":1,"b":1,"a":1}`
- Purpose:

Background color of the comment box (defaults to White)

### `comment`

- Required: **no**
- Type: `string`
- Default: declared by the live schema; inspect it with `describe`.
- Purpose:

Text to display on the comment box

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

The Dataflow asset to add the comment to

### `nodes`

- Required: **yes**
- Type: `array<object>`
- Purpose:

List of EdNodes to surround with the comment box

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  DataflowAgent.DataflowAgentToolset \
  DataflowAgent.DataflowAgentToolset.AddCommentBox \
  --arguments '
{
  "graph": {},
  "nodes": []
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

Node ID string of the created comment node, or empty string on failure

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
