# Replace widget with child

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.ReplaceWidgetWithChild
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Replaces a panel widget with its first child, removing the panel from the tree.
The widget to replace must be a UPanelWidget with only one child.

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
Use this tool to remove one unnecessary single-child SHAR panel while keeping
its child in the same outer hierarchy position.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the panel and its child through `GetWidgets`.
- Confirm the panel has exactly one child.
- Capture the panel outer slot and child properties.
- Define the expected child parent and slot after replacement.
- Compile and inspect the resulting tree before saving.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetStructure.WBP_MCP_WidgetStructure"
  },
  "widgetToReplace": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetStructure.WBP_MCP_WidgetStructure:WidgetTree.VerticalBox_0"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The wrapper had exactly one child, `ActionButton`. The call returned `true`.
`GetWidgets` changed from `RootCanvas`, `VerticalBox_0`, `ActionButton` to
`RootCanvas`, `ActionButton`. The button became a direct child of `RootCanvas`
and received `CanvasPanelSlot_0`. The resulting tree compiled successfully.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The addressed widget must be a panel with exactly one child.
- The child receives a new slot in the panel parent.
- Panel-specific layout properties can be lost and may need reapplication.
- The boolean result requires an independent hierarchy and slot check.
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

The widget blueprint containing the panel widget.

### `widgetToReplace`

- Required: **yes**
- Type: `object`
- Purpose:

The panel widget to replace with its first child.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.ReplaceWidgetWithChild \
  --arguments '
{
  "widgetBlueprint": {},
  "widgetToReplace": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

true on success, false if validation failed.

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
