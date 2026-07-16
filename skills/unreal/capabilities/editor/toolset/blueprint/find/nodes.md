# Find nodes

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.find_nodes
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Finds nodes in a graph by title, class, and/or execution role.

All filters are optional and ANDed together. Useful for locating specific event
chains in large graphs like EventGraph before reading them with
get_connected_subgraph.

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
Use this tool to discover bounded node refs by graph, title, class, or entry-
point posture before requesting detailed node or pin information.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply a current graph ref.
- Use the narrowest title or class filter that satisfies the inspection.
- Keep returned node refs within the current loaded graph revision.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {
    "refPath": "/Game/B_SHAR_MCP_Blueprint_GraphFixture.B_SHAR_MCP_Blueprint_GraphFixture:EventGraph"
  },
  "title": "BeginPlay",
  "node_class": null,
  "entry_points_only": false
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated `BeginPlay` and `Tick` filters each returned one exact event node. A
missing title returned an empty array. An empty title returned all three event
nodes; `entry_points_only: true` returned the same set because every node in the
fixture was an event entry point.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty result is valid and does not mean the graph is missing.
- Title and class filters depend on editor-visible node metadata.
- Node refs become stale after structural edits, recompilation, or deletion.
- Entry-point filtering can equal the full result when every node is an entry
  point.
- Use `get_node_infos` for pin and type details.
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

## Inputs

### `entry_points_only`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, only returns entry point nodes; nodes with an exec output but no exec
input (event nodes, function entry nodes, macro input tunnels, etc.). Nothing
can drive these nodes; they start execution themselves.

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `node_class`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `title`

- Required: **yes**
- Type: `string`
- Purpose:

Case-insensitive substring to match against the node's displayed title. Pass an
empty string to match any title.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.find_nodes \
  --arguments '
{
  "graph": {},
  "title": "<value>"
}
'
```

## Expected output

All nodes in the graph matching the given filters.

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
