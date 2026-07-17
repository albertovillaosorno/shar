# Remove ui component

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.RemoveUIComponent
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Removes a UI component of the given class from the named widget.

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
Use this tool to remove one exact UI behavior component class from a reviewed
SHAR widget after its properties and ordering dependencies are captured.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact component class from `GetWidgets`.
- Capture the component instance, class, order, and any property dependencies.
- Confirm the component is disposable or has an approved reconstruction path.
- Define the expected remaining component order.
- Compile and inspect the Widget Blueprint after removal.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_UIComponents.WBP_MCP_UIComponents"
  },
  "widgetName": "ActionButton",
  "componentClass": {
    "refPath": "/Script/UMG.ScaleBoxComponent"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Removing `ScaleBoxComponent` returned `true` and left only
`MouseHoverComponent`. Removing `MouseHoverComponent` returned `true` and left
an empty component array. Independent `GetWidgets` reads confirmed both states.
The empty-component Widget Blueprint compiled successfully, and deleting the
entire disposable asset left no registry, folder, or filesystem residue.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Removal is class-based and can affect all assumptions tied to that class.
- The boolean result requires an independent absence and remaining-order check.
- Missing-class and duplicate-class behavior need separate verification.
- Component properties and external references are not returned by removal.
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

### `componentClass`

- Required: **yes**
- Type: `object`
- Purpose:

The UIComponent subclass to remove.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint to modify.

### `widgetName`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the widget instance to remove the component from.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.RemoveUIComponent \
  --arguments '
{
  "componentClass": {},
  "widgetBlueprint": {},
  "widgetName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

True if the component was removed, false if it was not found.

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
