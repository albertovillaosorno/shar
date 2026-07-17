# Rename widget

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.RenameWidget
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Renames a widget. Returns updated widget info or empty on failure.

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
Use this tool to assign a reviewed semantic name to one SHAR widget while
preserving its class, parent relationship, slot, and subtree.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact widget through `GetWidgets`.
- Confirm the requested display name is unique in the widget tree.
- Capture the current widget reference, parent, slot, and child relationships.
- Retain the updated widget reference returned by the rename for all later
  operations.
- Define compile and structure checks after mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree"
  },
  "widget": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree:WidgetTree.ActionButton"
  },
  "newDisplayName": "PrimaryAction"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned updated widget info with name `PrimaryAction` and a new
reference ending in `WidgetTree.PrimaryAction`. Its class remained
`/Script/UMG.Button`, and its parent and slot still belonged to `LeftColumn`.
`GetWidgets` no longer returned `ActionButton` and returned `PrimaryAction` in
the same depth-first position. The renamed reference was then accepted by
`MoveWidget`, and the resulting tree compiled successfully.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Renaming changes the widget object reference; stale pre-rename references
  must not be reused.
- The operation changes the internal widget name, not only a designer caption.
- Bindings, graph references, animations, generated variables, and external
  assumptions require independent verification.
- Duplicate or invalid names can fail or be normalized by Unreal.
- Rename changes the unsaved Widget Blueprint and requires separate compile and
  save decisions.
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

### `newDisplayName`

- Required: **yes**
- Type: `string`
- Purpose:

The new display name.

### `widget`

- Required: **yes**
- Type: `object`
- Purpose:

The widget to rename.

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
  UMGToolSet.UMGToolSet.RenameWidget \
  --arguments '
{
  "newDisplayName": "<value>",
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
