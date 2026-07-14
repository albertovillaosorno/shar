# Add subgraph node

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.AddSubgraphNode
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Adds a subgraph node to the graph.

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
[TODO]
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
[TODO]
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
[FILL_ME]
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
[TODO]
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
[TODO]
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

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

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Graph to modify.

### `jsonParams`

- Required: **yes**
- Type: `string`
- Purpose:

The Json string representing a dictionary of the params to override on the
graph. Optional. Default is empty. Only non-default params need be included

### `nodeComment`

- Required: **yes**
- Type: `string`
- Purpose:

The comment attached to the node, if needed. Default is empty and will clear
the comment.

### `nodeName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the added node. (Must be unique identifier in the graph)

### `nodeTitle`

- Required: **yes**
- Type: `string`
- Purpose:

The Display Title of the node. Optional. Default is empty.

### `subGraphForNode`

- Required: **yes**
- Type: `object`
- Purpose:

The subgraph to use in the added node.

### `xPositionIdx`

- Required: **no**
- Type: `integer`
- Default: `0`
- Purpose:

The X coordinate of the position of the node in the editor. Optional. Default
is 0. Typical node size X is 200

### `yPositionIdx`

- Required: **no**
- Type: `integer`
- Default: `0`
- Purpose:

The Y coordinate of the position of the node in the editor. Optional. Default
is 0. Typical node size Y is 100

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PCGToolset.PCGToolset \
  PCGToolset.PCGToolset.AddSubgraphNode \
  --arguments '
{
  "graph": {},
  "jsonParams": "<value>",
  "nodeComment": "<value>",
  "nodeName": "<value>",
  "nodeTitle": "<value>",
  "subGraphForNode": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

The Added Node object

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
