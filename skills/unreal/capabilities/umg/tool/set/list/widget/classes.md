# List widget classes

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.ListWidgetClasses
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Lists available widget classes, optionally filtered by name substring.

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
Use this tool to discover widget classes and editor descriptions before SHAR
inspects class metadata or prepares a separately authorized widget addition.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply an explicit class-name substring filter.
- Treat results as dependent on loaded modules, WidgetBlueprints, and editor
  state.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "filter": "VerticalBox"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two `VerticalBox` calls returned one Panel-category class with `bIsPanel: true`.
`Audio` returned 35 classes, including native AudioWidgets classes, generated
WidgetBlueprint classes, and transient `REINST` and `SKEL` classes. An unmatched
filter returned an empty array.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Filtering is substring-based rather than an exact class lookup.
- Results can include transient reinstanced and skeleton classes unsuitable for
  durable references.
- Prefer stable `/Script/...` or generated `_C` classes after verifying their
  asset owner.
- Class descriptions are editor guidance and can change by engine revision.
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
shar-unreal-mcp describe UMGToolSet.UMGToolSet
```

1. Confirm every required input against the current schema.

## Inputs

### `filter`

- Required: **yes**
- Type: `string`
- Purpose:

Substring to match against class names. Pass empty string to return all
classes.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.ListWidgetClasses \
  --arguments '
{
  "filter": "<value>"
}
'
```

## Expected output

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
