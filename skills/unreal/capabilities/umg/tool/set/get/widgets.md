# Get widgets

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.GetWidgets
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Returns blueprint info and all widgets in depth-first order. Children within
each parent are in their panel slot order - this is the hierarchy order shown
in the designer. Info contains ParentClass (pass to CreateWidgetBlueprint) and
RootWidgetClass. Use ObjectTools.list_properties on each returned Widget and
Slot to get property names before calling set_properties.

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
Use this tool to obtain WidgetBlueprint-level metadata and a depth-first widget
inventory before SHAR reviews hierarchy, variables, slots, or classes.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact WidgetBlueprint object ref from `ListWidgetBlueprints`.
- Keep the WidgetBlueprint loaded and unchanged while consuming nested widget
  refs.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {"refPath": "/AudioWidgets/AudioFader/AudioFader.AudioFader"}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned five widgets in depth-first order: `VerticalBox_0`,
`AudioTextBox`, `CanvasPanel_3`, `SliderShadow`, and `Slider`. The root class
was `VerticalBox`, parent class was `UserWidget`, inherited and named-slot
counts were zero, and three widgets were variables.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Parent, slot, and named-slot host absence serializes as the string `"None"`.
- Nested widget and slot refs are session-sensitive and can become stale after
  compilation or edits.
- Depth-first order describes hierarchy traversal, not rendering or event order.
- A missing WidgetBlueprint ref fails during parameter translation.
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
shar-unreal-mcp describe UMGToolSet.UMGToolSet
```

1. Confirm every required input against the current schema.

## Inputs

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint asset (e.g. "/Game/UI/WBP_MyWidget"), excluding the "_C"
suffix.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.GetWidgets \
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

UMGWidgetTreeInfo

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
