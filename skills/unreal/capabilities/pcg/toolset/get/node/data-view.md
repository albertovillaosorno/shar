# Get node data view

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.GetNodeDataView
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Returns a JSON Data View of a specific node's output data from the last graph
execution. On first call, enables inspection so future ExecuteGraphInstance
calls store per-node data. If no inspection data exists, returns an error
prompting re-execution.

IMPORTANT: Inspection state is shared at the graph asset level. If multiple
actors use the same graph, you MUST call this tool (and ExecuteGraphInstance)
on only one actor at a time. Wait for each call to fully complete before
calling on the next actor. Concurrent calls on actors sharing the same graph
will cause a freeze.

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

## Inputs

### `attributeName`

- Required: **yes**
- Type: `string`
- Purpose:

Filter to a single attribute/property (e.g. "$Position", "$Density",
"MyCustomAttr"). Empty = all attributes.

### `endIndex`

- Required: **no**
- Type: `integer`
- Default: `-1`
- Purpose:

Element range end, exclusive. -1 means all elements (Python slice convention).
Default -1.

### `node`

- Required: **yes**
- Type: `object`
- Purpose:

The node whose output to inspect.

### `pCGVolume`

- Required: **yes**
- Type: `object`
- Purpose:

The PCG Volume whose graph was executed.

### `pinLabel`

- Required: **no**
- Type: `string`
- Default: `"Out"`
- Purpose:

Output pin label to read. Defaults to "Out".

### `startIndex`

- Required: **no**
- Type: `integer`
- Default: `0`
- Purpose:

Element range start, inclusive, 0-based. Default 0.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PCGToolset.PCGToolset \
  PCGToolset.PCGToolset.GetNodeDataView \
  --arguments '
{
  "attributeName": "<value>",
  "node": {},
  "pCGVolume": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

JSON string with the data view contents

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
