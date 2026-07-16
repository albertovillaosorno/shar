# Get graph structure

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.GetGraphStructure
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Returns the complete structure of a PCG graph including all nodes, connections,
exposed parameters, and comment boxes.

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
Use this tool to inspect PCG node inventory, positions, comments, parameter
overrides, and explicit pin-to-pin edges during SHAR procedural-content review.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact indexed `PCGGraph` object path.
- Treat returned node object paths as graph-local identities.
- Use `GetGraphSchema` separately for public parameters and graph interface
  pins.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {"refPath": "/PCG/SampleContent/SpawnPointsFromPoints/SpawnPointsFromPointsGraph.SpawnPointsFromPointsGraph"}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two sample-graph calls returned 14 enabled nodes and 11 edges. The inventory
included Surface Sampler, Execute Blueprint, Density Filter, Projection, Copy
Points, Landscape Data, and Debug nodes with stable positions, comments, titles,
and parameter overrides. `_Default_EmptyGraph` independently returned its Input
and Output nodes with zero edges.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The result describes authored graph structure, not runtime execution order or
  generated data.
- `paramOverrides` shape varies by node type and may contain nested object refs.
- Node paths and generated names can change after graph edits or duplication.
- Edge entries identify source and destination node names plus pin labels.
- Missing graph refs raise a native parameter error.
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

## Inputs

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Graph to inspect

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PCGToolset.PCGToolset \
  PCGToolset.PCGToolset.GetGraphStructure \
  --arguments '
{
  "graph": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

FPCGGraphStructure with graph name, description, nodes, and edges

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
