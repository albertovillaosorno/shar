# Find node types

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.find_node_types
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Finds node types that can be created in a particular graph meeting the search
criteria.

Note: there can be thousands of valid node types in a graph, so type_id_filter
should be reasonably specific even if that requires multiple searches.

To list all nodes within a category, use a trailing pipe: e.g.
'Utilities|FlowControl|' returns every node under that category. Without the
trailing pipe, 'Utilities|FlowControl' also matches any node whose type_id
contains that substring (such as a node with category 'Utilities' and title
'FlowControl').

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
Use this tool to discover node type identifiers available in one graph context
before pin inspection or node creation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply a current graph ref.
- Use a narrow type-id filter and explicit context pins.
- Treat the result as graph- and plugin-specific.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {"refPath": "/Game/B_SHAR_MCP_Blueprint_GraphFixture.B_SHAR_MCP_Blueprint_GraphFixture:EventGraph"},
  "type_id_filter": "Branch",
  "context_pins": []
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned the same 21 identifiers. The broad `Branch` filter included
PCG casts and properties, struct operations, and `Utilities|FlowControl|Branch`,
proving the filter is not an exact node-name lookup.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Filtering is broad and can match category or class text.
- Results vary with loaded plugins and graph schema.
- An empty context-pin list performs no compatibility narrowing.
- Use an exact returned identifier with `get_node_type_pins`.
- Availability does not prove successful node creation in a later graph
  revision.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

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

### `context_pins`

- Required: **yes**
- Type: `array<object>`
- Purpose:

If set, only returns nodes whose pin types match those specified in this list

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `type_id_filter`

- Required: **yes**
- Type: `string`
- Purpose:

Substring to match against the node type_id (case-insensitive)

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.find_node_types \
  --arguments '
{
  "context_pins": [],
  "graph": {},
  "type_id_filter": "<value>"
}
'
```

## Expected output

Returns nodes that match the input criteria

### `returnValue`

- Required: **yes**
- Type: `array<string>`
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
