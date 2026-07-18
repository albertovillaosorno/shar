# Bind to event property

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.BindToEventProperty
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Adds a Blueprint event handler graph node bound to a widget's multicast
delegate event,

Typical events: UButton::OnClicked / OnPressed / OnReleased / OnHovered /
OnUnhovered, UCheckBox::OnCheckStateChanged, USlider::OnValueChanged. The
matching delegate UPROPERTY must exist on PropertyClass (or a parent of it).

Preconditions: - PropertyName must exist in the blueprint. - PropertyClass must
be the widget's class (or a parent class) that declares the delegate.

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
Use this tool to bind one named Widget Blueprint variable delegate to a
generated event node.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a disposable Widget Blueprint with a Canvas root and a named child
  widget marked as a Blueprint variable.
- Refresh the UMG and Blueprint schemas, then require successful widget
  compilation and exact event-node discovery.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "eventName": "OnClicked",
  "propertyClass": {
    "refPath": "/Script/UMG.Button"
  },
  "propertyName": "MCP_Button",
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation_Round50_260718/WBP_MCP_Round50.WBP_MCP_Round50"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Widget compilation succeeded, `MCP_Button` remained a variable, and Blueprint
node discovery returned `K2Node_ComponentBoundEvent_0` in the Event Graph.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Widget, slot, graph, and generated-node references are Blueprint subobjects
  and become stale after reconstruction or deletion.
- Compile after structural changes and rediscover the generated event node
  instead of predicting its object path.
- `propertyName` is the Widget Blueprint variable name, while `eventName` is
  the delegate name. The widget must be marked as a variable first.
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

### `eventName`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the multicast delegate UPROPERTY on PropertyClass (e.g. "OnClicked").

### `propertyClass`

- Required: **yes**
- Type: `object`
- Purpose:

Class declaring the delegate, typically the widget's class (e.g.
UButton::StaticClass()).

### `propertyName`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the blueprint variable owning the event.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint that will own the event handler.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.BindToEventProperty \
  --arguments '
{
  "eventName": "<value>",
  "propertyClass": {},
  "propertyName": "<value>",
  "widgetBlueprint": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
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
