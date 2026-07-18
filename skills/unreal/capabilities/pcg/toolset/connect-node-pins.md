# Connect node pins

[Return to the central Unreal MCP index](../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.ConnectNodePins
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Add an edge between two nodes connected to the specified pins.

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
Use this tool to connect one compatible output pin to one exact input pin.
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
  "fromNode": {
    "refPath": "/Game/SHAR_MCP_Validation_PCG_c297c180/PCG_MCP_Main_c297c180.PCG_MCP_Main_c297c180:CreatePoint_c297c180"
  },
  "fromPinLabel": "Out",
  "toNode": {
    "refPath": "/Game/SHAR_MCP_Validation_PCG_c297c180/PCG_MCP_Main_c297c180.PCG_MCP_Main_c297c180:Transform_c297c180"
  },
  "toPinLabel": "In"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`GetGraphStructure` added the exact Create Points.Out to Transform Points.In
edge. The same fixture also connected Transform Points.Out to the graph
output.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- PCG graph, node, comment, and volume references are transient editor
  identities and become stale after deletion.
- The reproduced lifecycle used one disposable folder and removed both graphs,
  the Dataflow companion graph, and the PCG Volume afterward.
- The native return was false or an empty value even when `GetGraphStructure`
  proved the requested edge mutation. Never trust the Boolean result alone.
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

### `fromNode`

- Required: **yes**
- Type: `object`
- Purpose:

The source node of the edge to add.

### `fromPinLabel`

- Required: **yes**
- Type: `string`
- Purpose:

The label of the source pin of the source node.

### `toNode`

- Required: **yes**
- Type: `object`
- Purpose:

The destination node of the edge to add.

### `toPinLabel`

- Required: **yes**
- Type: `string`
- Purpose:

The label of the destination pin of the destination node.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PCGToolset.PCGToolset \
  PCGToolset.PCGToolset.ConnectNodePins \
  --arguments '
{
  "fromNode": {},
  "fromPinLabel": "<value>",
  "toNode": {},
  "toPinLabel": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The list of nodes (conversions and/or filters) that were added to allow the
connection to work.

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
