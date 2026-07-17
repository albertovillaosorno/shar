# Add widget

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.AddWidget
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Adds a widget to the tree at the specified position. Returns full widget info
including Slot pointer. When ParentWidget is null and no root exists, the new
widget becomes the root of the tree. Use ObjectTools.list_properties on the
returned Widget and Slot to get property names before calling set_properties.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Capture pre-state, bound the target set, and verify the resulting editor or
asset state through an independent read.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to add one exact SHAR widget class at a reviewed location in a
Widget Blueprint tree for bounded HUD, menu, prompt, overlay, or debug UI work.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply an exact loaded Widget Blueprint reference.
- Inspect the current tree with `GetWidgets` and resolve the intended parent.
- Confirm the widget class is valid and the display name is unique.
- Choose the child index explicitly when sibling order matters.
- Define exact widget or subtree removal and compile verification before a
  disposable mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree"
  },
  "parentWidget": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree:WidgetTree.LeftColumn"
  },
  "widgetClass": {
    "refPath": "/Script/UMG.Button"
  },
  "widgetDisplayName": "ActionButton",
  "childIndex": 0
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The disposable tree began empty. Adding a `CanvasPanel` without a parent
created the root. Two `VerticalBox` widgets added at indices `0` and `1` became
root children. Adding the button at index `0` under `LeftColumn` returned its
exact widget, parent, and `VerticalBoxSlot_0` references. `GetWidgets` then
returned depth-first order `RootCanvas`, `LeftColumn`, `ActionButton`,
`RightColumn`. Later removals restored an empty, compilable widget tree.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Omitting `parentWidget` creates the root only when the tree is empty.
- The supplied parent must accept children; not every widget is a panel.
- `childIndex: -1` appends, while explicit indices control panel order.
- The returned slot type depends on the parent panel class.
- Widget references are asset-internal identities and can change after rename or
  replacement.
- Addition changes the unsaved Widget Blueprint and requires separate compile
  and save decisions.
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `childIndex`

- Required: **no**
- Type: `integer`
- Default: `-1`
- Purpose:

Position in parent's child list (0 = first child). -1 (default) appends to end.

### `parentWidget`

- Required: **no**
- Type: `object`
- Purpose:

The panel widget to add to. Pass null to add to root, or to make this widget
the root if the tree is empty.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint to modify.

### `widgetClass`

- Required: **yes**
- Type: `object`
- Purpose:

The widget class to instantiate.

### `widgetDisplayName`

- Required: **yes**
- Type: `string`
- Purpose:

Display name for the new widget instance.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.AddWidget \
  --arguments '
{
  "widgetBlueprint": {},
  "widgetClass": {},
  "widgetDisplayName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

UMGWidgetInfo

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Verify changed state through a separate read or inspection.
- Use another capability to confirm the postcondition.
- Inspect editor logs when state is not directly observable.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
