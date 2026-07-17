# Set named slot content

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.SetNamedSlotContent
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Sets content for a named slot. Returns full widget info including Slot pointer.

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
Use this tool to place one reviewed SHAR widget class into an exposed named
slot on a Widget Blueprint host.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact Widget Blueprint and host widget.
- Prove the host exposes the requested slot through its compiled widget class.
- Capture current named-slot bindings with `GetNamedSlots`.
- Choose a unique content name and exact widget class.
- Define host replacement or complete disposable asset cleanup before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_NamedContainer.WBP_MCP_NamedContainer"
  },
  "hostWidget": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_NamedContainer.WBP_MCP_NamedContainer:WidgetTree.NamedHostInstance"
  },
  "slotName": "Content",
  "widgetClass": {
    "refPath": "/Script/UMG.Button"
  },
  "widgetName": "SlotAction"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A compiled disposable host Widget Blueprint exposed a `NamedSlot` named
`Content`. The call returned a `Button` named `SlotAction`. Its ordinary parent
and slot were `None`, `bIsVariable` was `true`, and `GetWidgets` identified
`NamedHostInstance` as its named-slot host. `GetNamedSlots` returned exactly one
binding connecting host, slot name, and content widget. The bound container
compiled successfully before host replacement.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Named-slot content is not represented by an ordinary panel parent or slot.
- `GetNamedSlots` is the authoritative binding check; `GetWidgets` supplies the
  content widget and named-slot host identity.
- The validated content was automatically exposed as a Blueprint variable.
- The host class must implement `INamedSlotInterface` and expose the exact slot.
- Replacing existing slot content, invalid slot names, and inherited hosts need
  separate verification.
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

### `hostWidget`

- Required: **yes**
- Type: `object`
- Purpose:

The widget that owns the named slot, or null to target the root WidgetTree.

### `slotName`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the slot to fill (e.g., "content", "header").

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint to modify.

### `widgetClass`

- Required: **yes**
- Type: `object`
- Purpose:

The widget class to place in the slot.

### `widgetName`

- Required: **yes**
- Type: `string`
- Purpose:

Name for the new widget instance.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.SetNamedSlotContent \
  --arguments '
{
  "hostWidget": {},
  "slotName": "<value>",
  "widgetBlueprint": {},
  "widgetClass": {},
  "widgetName": "<value>"
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
