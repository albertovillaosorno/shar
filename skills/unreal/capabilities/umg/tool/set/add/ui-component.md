# Add ui component

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.AddUIComponent
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Adds a UI component of the given class to the named widget.

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
Use this tool to attach one reviewed native UI behavior component to an exact
SHAR widget in a Widget Blueprint.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve a real `UIComponent` subclass with `ObjectTools.search_subclasses`.
- Use an exact loaded Widget Blueprint and widget instance name.
- Capture the widget's current `uIComponents` array through `GetWidgets`.
- Define component removal and compilation before a disposable mutation.
- Treat component properties as a separate inspection and mutation stage.
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
    "refPath": "/Script/UMG.MouseHoverComponent"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Class discovery returned seven loaded `UIComponent` classes, including native
`MouseHoverComponent` and `ScaleBoxComponent`. The button initially had no UI
components. Adding `MouseHoverComponent` returned one exact component reference.
Adding `ScaleBoxComponent` appended a second reference. Independent `GetWidgets`
reads reproduced the same class order, and the Widget Blueprint compiled.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Do not guess component classes; discover loaded subclasses first.
- Addition is keyed by widget name and component class, not a component name.
- The return value contains the complete component array for the widget.
- A component instance lives in the Widget Blueprint extension container.
- Duplicate-class behavior and component-specific properties need separate
  verification.
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

The UIComponent subclass to add.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint to modify.

### `widgetName`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the widget instance to add the component to.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.AddUIComponent \
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
- Type: `object`
- Purpose:

Info for the widget the component was added to, including the populated
UIComponents array. Returns an empty info on failure.

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
