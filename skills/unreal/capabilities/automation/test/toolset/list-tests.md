# List tests

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
AutomationTestToolset.AutomationTestToolset.ListTests
```

Toolset:

```text
AutomationTestToolset.AutomationTestToolset
```

## What this tool does

List available automation tests. Requires DiscoverTests() to have completed.
Returns a JSON object: {"tests": ["path1", ...], "total": N, "returned": N}.

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
Use this tool after discovery to locate exact Unreal automation test identities
for SHAR editor, native MCP, import, or validation workflows. Keep both filters
and the result limit explicit so later execution cannot target an unintended
large test set.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- `DiscoverTests` must have completed successfully in the same editor session.
- Choose a narrow name or tag substring from the intended test domain.
- Use a positive bounded `limit` for routine inspection.
- Treat returned full paths as the only valid identities for `RunTests`.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "nameFilter": "MCP",
  "tagFilter": "",
  "limit": 20
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned the same ordered 20 test paths with
`total: 21` and `returned: 20`. Setting `limit` to `0` returned all 21 matches.
An unmatched name and tag-filtered MCP queries returned empty lists without an
error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON string and requires a second JSON parse.
- `nameFilter` and `tagFilter` narrow the same result set; a nonmatching tag can
  reduce a valid name search to zero.
- `total` is the number matching the filters, while `returned` is bounded by
  `limit`.
- `limit: 0` is documented as unlimited and should be avoided for broad routine
  queries. A negative limit also behaved as unlimited in the verified session,
  but that behavior is undocumented and must not be relied upon.
- Test inventory and ordering can change after plugin, module, or engine changes;
  rediscover before using stale identities.
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
shar-unreal-mcp describe AutomationTestToolset.AutomationTestToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `limit`

- Required: **no**
- Type: `integer`
- Default: `200`
- Purpose:

Maximum number of tests to return (0 = unlimited, default 200).

### `nameFilter`

- Required: **yes**
- Type: `string`
- Purpose:

Optional substring filter applied to the test's full path.

### `tagFilter`

- Required: **yes**
- Type: `string`
- Purpose:

Optional substring filter applied to the test's tags.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  AutomationTestToolset.AutomationTestToolset \
  AutomationTestToolset.AutomationTestToolset.ListTests \
  --arguments '
{
  "nameFilter": "<value>",
  "tagFilter": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
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
