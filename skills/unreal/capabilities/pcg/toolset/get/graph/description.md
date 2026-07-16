# Get graph description

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.GetGraphDescription
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Returns the description of a PCG graph.

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
Use this tool to read a PCG graph's authored description before SHAR relies on
graph intent or prepares a separately authorized description change.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact indexed `PCGGraph` object path.
- Treat an empty string as a valid no-description result.
- Verify behavior through graph structure and schema rather than inferring it
  from prose alone.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {"refPath": "/PCG/GraphTemplates/_Default_EmptyGraph._Default_EmptyGraph"}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned an empty string for `_Default_EmptyGraph`. Repeated reads of
`SpawnPointsFromPointsGraph`, loop templates, sampler templates, and
`SimpleForest` also returned empty strings while their schemas and structures
loaded successfully.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Empty means no graph description is authored; it is not an error.
- Description text is optional documentation and does not determine execution
  behavior.
- Node comments and titles are available separately through `GetGraphStructure`.
- Missing graph refs fail during parameter translation.
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

Graph to get the description of.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PCGToolset.PCGToolset \
  PCGToolset.PCGToolset.GetGraphDescription \
  --arguments '
{
  "graph": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

The graph's description text, or empty string if the graph has no description
set.

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
