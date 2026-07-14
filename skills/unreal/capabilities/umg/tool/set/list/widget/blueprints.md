# List widget blueprints

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.ListWidgetBlueprints
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Lists widget blueprints in a content folder.

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
Use this tool to discover WidgetBlueprint assets in one content folder before
SHAR performs tree inspection or a separately authorized UI edit.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply a normalized Unreal content folder path.
- Treat returned object refs as the authoritative inputs for other UMG tools.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "folderPath": "/AudioWidgets/AudioFader"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned only `/AudioWidgets/AudioFader/AudioFader.AudioFader`.
Querying `/AudioWidgets` returned eight WidgetBlueprints across child folders,
proving folder discovery is recursive. An unmatched folder returned an empty
array.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty result is valid for a folder with no matching WidgetBlueprints.
- Results depend on mounted plugin content and Asset Registry state.
- Folder discovery is recursive and can include descendants.
- Use the full returned object ref, not a package-only path.
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

### `folderPath`

- Required: **yes**
- Type: `string`
- Purpose:

Content folder to search, e.g. "/Game/UI". Searches recursively.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.ListWidgetBlueprints \
  --arguments '
{
  "folderPath": "<value>"
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
