# Add node

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.AddNode
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Adds a native node to the graph.

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
Use this tool to add exact live-discovered node types to a reviewed procedural
graph.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  exact live toolset schema before mutation.
- Use a unique disposable graph and define whole-folder cleanup before
  invocation.
- Use `GetGraphStructure`, `GetGraphSchema`, `GetNodeInfo`, or the matching
  instance reader as the independent postcondition.
- Discover native node property and pin schemas before supplying names or JSON
  overrides.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation_PCG_c297c180/PCG_MCP_Main_c297c180.PCG_MCP_Main_c297c180"
  },
  "jsonParams": "{\"pointsToCreate\":[{\"transform\":{\"location\":{\"x\":0,\"y\":0,\"z\":0},\"rotation\":{\"pitch\":0,\"yaw\":0,\"roll\":0},\"scale\":{\"x\":1,\"y\":1,\"z\":1}},\"density\":1.0,\"boundsMin\":{\"x\":-10,\"y\":-10,\"z\":-10},\"boundsMax\":{\"x\":10,\"y\":10,\"z\":10},\"color\":{\"x\":1,\"y\":0.25,\"z\":0.1,\"w\":1},\"steepness\":0.0,\"seed\":42,\"metadataEntry\":0}],\"coordinateSpace\":\"LocalComponent\",\"bCullPointsOutsideVolume\":false}",
  "nativeNodeType": "Create Points",
  "nodeComment": "Initial source",
  "nodeName": "CreatePoint_c297c180",
  "nodeTitle": "MCP Point Source",
  "xPositionIdx": 300,
  "yPositionIdx": 100
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`GetGraphStructure` added exact Create Points and Transform Points node names,
types, titles, comments, positions, and reflected parameter overrides.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- PCG graph, node, comment, and volume references are transient editor
  identities and become stale after deletion.
- The reproduced lifecycle used one disposable folder and removed both graphs,
  the Dataflow companion graph, and the PCG Volume afterward.
- Property overrides are JSON text and must match the live native-node schema
  exactly. Unspecified defaults may be omitted from read-back.
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

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Graph to modify.

### `jsonParams`

- Required: **yes**
- Type: `string`
- Purpose:

The Json string representing a dictionary of the params to set on the node.
Optional. Default is empty. Only non-default params need be included.

### `nativeNodeType`

- Required: **yes**
- Type: `string`
- Purpose:

The native type of the added node.

### `nodeComment`

- Required: **yes**
- Type: `string`
- Purpose:

The comment attached to the node, if needed. Default is empty.

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
  PCGToolset.PCGToolset.AddNode \
  --arguments '
{
  "graph": {},
  "jsonParams": "<value>",
  "nativeNodeType": "<value>",
  "nodeComment": "<value>",
  "nodeName": "<value>",
  "nodeTitle": "<value>"
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
