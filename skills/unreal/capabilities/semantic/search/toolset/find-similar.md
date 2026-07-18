# Find similar

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
SemanticSearchToolset.SemanticSearchToolset.FindSimilar
```

Toolset:

```text
SemanticSearchToolset.SemanticSearchToolset
```

## What this tool does

Find assets whose embeddings are semantically similar to the given asset's
embedding. Vector-only (no BM25). The source asset must already be indexed by
the SemanticSearch plugin.

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
Use this tool to find vector-nearest assets after the Semantic Search index has
been populated and the source asset is registered.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a full object path such as `/Game/X/Foo.Foo` or an existing engine object
  path.
- Require a non-empty Semantic Search vector index before invocation.
- Bound `k`, class filters, and path regular expressions to the intended asset
  scope.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "assetPath": {
    "refPath": "/Engine/BasicShapes/Cube.Cube"
  },
  "classFilter": [],
  "pathRegexes": [
    "^/Engine/BasicShapes/"
  ],
  "k": 5
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`/Engine/BasicShapes/Cube.Cube` resolved as a valid source asset, but the call
failed closed with `The SemanticSearch index is empty — nothing to compare
against.` The text-search indexing route also reported HTTP 401 without an
embedding provider credential.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A valid source asset is insufficient when the vector index count is zero.
- The source asset is excluded from its own result set.
- An empty candidate set can validly return an empty array after filters are
  applied.
- Do not place provider credentials in repository files or capability examples.
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
shar-unreal-mcp describe SemanticSearchToolset.SemanticSearchToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `assetPath`

- Required: **yes**
- Type: `object`
- Purpose:

SoftObjectPath of the reference asset.

### `classFilter`

- Required: **yes**
- Type: `array<object>`
- Purpose:

Same semantics as Search::ClassFilter. See Search for the list of currently-
indexed base classes. Pass an empty array for no class filter.

### `k`

- Required: **no**
- Type: `integer`
- Default: `10`
- Purpose:

Maximum number of results to return. Must be &gt;= 1. Defaults to 10.

### `pathRegexes`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Same semantics as Search::PathRegexes. Pass an empty array for no path filter.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  SemanticSearchToolset.SemanticSearchToolset \
  SemanticSearchToolset.SemanticSearchToolset.FindSimilar \
  --arguments '
{
  "assetPath": {},
  "classFilter": [],
  "pathRegexes": []
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

Array of FSemanticSearchResult sorted by relevance (highest Score first)

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
