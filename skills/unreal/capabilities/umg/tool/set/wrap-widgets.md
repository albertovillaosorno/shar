# Wrap widgets

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.WrapWidgets
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Wraps one or more widgets in a new panel widget of the specified class. Only
the root-most widgets in the selection are wrapped — children of other selected
widgets are skipped because their parent will be wrapped. Returns info for each
newly created wrapper.

Use ObjectTools.list_properties on each returned Widget and Slot to discover
property names before calling set_properties (padding, alignment, anchors, etc.
vary per panel class).

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to place one or more reviewed SHAR widgets inside a new panel
while preserving their internal subtrees.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve every selected widget through `GetWidgets`.
- Capture parent, slot, sibling order, and descendants.
- Choose a valid `PanelWidget` wrapper class.
- Select only intended root-most widgets.
- Define wrapper removal or replacement and compile checks.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetStructure.WBP_MCP_WidgetStructure"
  },
  "widgets": [
    {
      "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetStructure.WBP_MCP_WidgetStructure:WidgetTree.ActionButton"
    }
  ],
  "wrapperClass": {
    "refPath": "/Script/UMG.VerticalBox"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Wrapping the button returned one `VerticalBox_0` wrapper. `GetWidgets` changed
from `RootCanvas`, `ActionButton` to `RootCanvas`, `VerticalBox_0`,
`ActionButton`. The wrapper occupied the original canvas slot, while the
button moved into `VerticalBoxSlot_0`. The wrapped tree compiled successfully.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Only root-most selected widgets are wrapped.
- The wrapper receives the original outer slot; children receive new panel
  slots.
- Slot-specific properties may require translation or reapplication.
- Generated wrapper names must be read from the return value.
- The operation changes unsaved state and needs separate compile and save
  decisions.
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

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint to modify.

### `widgets`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The widgets to wrap. Must all be in WidgetBlueprint's tree.

### `wrapperClass`

- Required: **yes**
- Type: `object`
- Purpose:

The panel widget class to wrap with (must be a UPanelWidget subclass).

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.WrapWidgets \
  --arguments '
{
  "widgetBlueprint": {},
  "widgets": [],
  "wrapperClass": {}
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
- Verify changed state through a separate read or inspection.
- Use another capability to confirm the postcondition.
- Inspect editor logs when state is not directly observable.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
