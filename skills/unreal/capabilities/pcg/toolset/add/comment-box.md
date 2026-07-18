# Add comment box

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.AddCommentBox
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Adds a comment box around the given nodes. Note: If the bounding box of the
nodes include other nodes, they will be included in the comment.

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
Use this tool to group reviewed PCG nodes inside one labeled editor comment
box.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  exact live toolset schema before mutation.
- Use a unique disposable graph and define whole-folder cleanup before
  invocation.
- Use `GetGraphStructure`, `GetGraphSchema`, `GetNodeInfo`, or the matching
  instance reader as the independent postcondition.
- Open the exact PCG graph editor and use a Slate snapshot to verify unique
  visible comment text.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "color": {
    "a": 1.0,
    "b": 0.8,
    "g": 0.4,
    "r": 0.2
  },
  "comment": "MCP Comment c297c180",
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation_PCG_c297c180/PCG_MCP_Main_c297c180.PCG_MCP_Main_c297c180"
  },
  "nodes": [
    {
      "refPath": "/Game/SHAR_MCP_Validation_PCG_c297c180/PCG_MCP_Main_c297c180.PCG_MCP_Main_c297c180:CreatePoint_c297c180"
    },
    {
      "refPath": "/Game/SHAR_MCP_Validation_PCG_c297c180/PCG_MCP_Main_c297c180.PCG_MCP_Main_c297c180:Transform_c297c180"
    }
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The unique comment text was absent before the fresh add and visible in the PCG
editor Slate snapshot afterward.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- PCG graph, node, comment, and volume references are transient editor
  identities and become stale after deletion.
- The reproduced lifecycle used one disposable folder and removed both graphs,
  the Dataflow companion graph, and the PCG Volume afterward.
- Comment identifiers are opaque editor GUID strings and are not exposed by
  `GetGraphStructure`.
- The reproduced postcondition used unique visible text in a Slate snapshot of
  the open PCG editor.
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
shar-unreal-mcp describe PCGToolset.PCGToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `color`

- Required: **no**
- Type: `object`
- Default: `{"r":1,"g":1,"b":1,"a":1}`
- Purpose:

The color of the comment box. Defaults to White

### `comment`

- Required: **no**
- Type: `string`
- Default: declared by the live schema; inspect it with `describe`.
- Purpose:

The comment to put on the comment box.

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Graph to add comment to.

### `nodes`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The list of nodes to include in the comment box.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PCGToolset.PCGToolset \
  PCGToolset.PCGToolset.AddCommentBox \
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

The unique id of the comment node. (Can be used to edit/remove the comment)

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
