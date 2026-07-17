# Move widget

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.MoveWidget
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Moves a widget to a new parent panel at the specified position. Returns updated
widget info with new Slot.

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
Use this tool to move one SHAR widget to a reviewed parent panel and sibling
position while preserving the widget instance and subtree.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact widget and destination panel through `GetWidgets`.
- Capture the source parent, slot, sibling order, and widget subtree.
- Confirm the destination accepts the widget and will not create a hierarchy
  cycle.
- Choose the destination child index explicitly.
- Define inverse movement or disposable subtree removal before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree"
  },
  "widget": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree:WidgetTree.PrimaryAction"
  },
  "newParent": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree:WidgetTree.RightColumn"
  },
  "childIndex": 0
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned the same `PrimaryAction` widget reference with parent
`RightColumn` and a new slot ending in `RightColumn.VerticalBoxSlot_0`.
`GetWidgets` changed from `RootCanvas`, `LeftColumn`, `PrimaryAction`,
`RightColumn` to `RootCanvas`, `LeftColumn`, `RightColumn`, `PrimaryAction`,
proving the new depth-first hierarchy. Compilation returned `true`. Removing
`RightColumn` then removed the moved button with its destination subtree.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Moving a widget replaces its panel slot; retain the returned slot reference.
- Slot-specific layout properties may be lost or require translation when the
  parent panel class changes.
- The destination must be a compatible `PanelWidget` and must not be inside the
  moved widget's subtree.
- `childIndex: -1` appends; explicit indices define sibling order.
- Movement changes the unsaved Widget Blueprint and requires separate compile
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

Position in new parent's child list (0 = first child). -1 (default) appends to
end.

### `newParent`

- Required: **yes**
- Type: `object`
- Purpose:

The destination panel widget.

### `widget`

- Required: **yes**
- Type: `object`
- Purpose:

The widget to move.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint to modify.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.MoveWidget \
  --arguments '
{
  "newParent": {},
  "widget": {},
  "widgetBlueprint": {}
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
