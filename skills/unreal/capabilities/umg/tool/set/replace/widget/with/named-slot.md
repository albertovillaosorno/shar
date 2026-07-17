# Replace widget with named slot

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.ReplaceWidgetWithNamedSlot
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Replaces a host widget with the content of one of its named slots. The host
must implement INamedSlotInterface (e.g., a UUserWidget exposing named slots).
The slot's content widget is moved up to take the host's place in the tree.

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
Use this tool to remove one SHAR named-slot host while promoting the selected
slot content into the host's outer hierarchy position.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact host, slot name, and bound content through
  `GetNamedSlots`.
- Capture the host parent, outer slot, and complete binding list.
- Confirm the selected content can replace the host semantically.
- Define the expected promoted parent, slot, and remaining bindings.
- Compile and inspect the resulting tree before saving.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_NamedContainer.WBP_MCP_NamedContainer"
  },
  "widgetToReplace": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_NamedContainer.WBP_MCP_NamedContainer:WidgetTree.NamedHostInstance"
  },
  "namedSlot": "Content"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Before replacement, `GetNamedSlots` reported one `Content` binding from
`NamedHostInstance` to `SlotAction`. The call returned `true`. `GetWidgets` then
removed the host and promoted the button directly under `ContainerRoot` using
`CanvasPanelSlot_0`, the host's former outer slot. A new named-slot read
returned
an empty array, and the resulting container compiled successfully. Both
temporary Widget Blueprints and their virtual folder were deleted without disk
residue.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The host must implement `INamedSlotInterface` and have content in the named
  slot.
- Replacement removes the host widget and promotes only the selected content.
- The promoted widget receives a normal slot in the host's former parent.
- Host-specific properties, layout, bindings, and behavior do not transfer
  automatically.
- The boolean result requires independent hierarchy, slot, and binding checks.
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

### `namedSlot`

- Required: **yes**
- Type: `string`
- Purpose:

The slot whose content replaces WidgetToReplace.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint containing the host widget.

### `widgetToReplace`

- Required: **yes**
- Type: `object`
- Purpose:

The host widget to replace.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.ReplaceWidgetWithNamedSlot \
  --arguments '
{
  "namedSlot": "<value>",
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
