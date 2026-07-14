# Get widget description

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.GetWidgetDescription
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Full property dump of every widget in the tree. Each line: [N] Type Name
Prop:Value ... slot:(SlotProp:Value ...) N is the 0-based index into
result.Widgets -- use result.Widgets[N] to get the widget ref without text
parsing.

Same indentation format as GetTaggedWidgetDescription; richer per-widget
detail.

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
Use this tool to produce a bounded human-readable widget-tree and property dump
linked to a flat widget array for SHAR UI review.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact loaded WidgetBlueprint object ref.
- Pass `startWidget: null` for the root.
- Use `maxDepth` to bound output before inspecting large trees.
- Correlate each `[N]` line with `widgets[N]`.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {"refPath": "/AudioWidgets/AudioFader/AudioFader.AudioFader"},
  "startWidget": null,
  "maxDepth": 1
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two depth-one calls returned three widgets and three indexed lines: root
VerticalBox plus AudioTextBox and CanvasPanel children. Full depth returned five
widgets, five lines, and 2,282 characters, adding the Image and Slider
grandchildren with current property and slot dumps.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The description is diagnostic text, not a stable serialization or write
  format.
- Property text can contain asset paths, floats, and long style structures.
- `maxDepth` prunes both description lines and the returned widget array.
- Widget refs and property text become stale after edits or compilation.
- A missing WidgetBlueprint ref fails during parameter translation.
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

### `maxDepth`

- Required: **no**
- Type: `integer`
- Default: `-1`
- Purpose:

1 = no limit; 0 = StartWidget only; N = N levels.

### `startWidget`

- Required: **no**
- Type: `object`
- Purpose:

nullptr = full tree from root.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The WBP asset.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.GetWidgetDescription \
  --arguments '
{
  "widgetBlueprint": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

UMGWidgetDescriptionResult

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
