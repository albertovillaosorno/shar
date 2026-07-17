# Remove widget

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.RemoveWidget
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Removes a widget and its children from the tree.

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
Use this tool to remove one exact disposable or explicitly approved SHAR widget
subtree after its descendants, bindings, variables, animations, and references
have been inventoried.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact widget and descendants through `GetWidgets`.
- Capture the parent, slot, sibling order, subtree, and any binding or variable
  dependencies.
- Confirm the subtree is disposable or has an approved reconstruction path.
- Define the expected remaining depth-first widget order.
- Compile after removal and decide separately whether to save.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree"
  },
  "widget": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_WidgetTree.WBP_MCP_WidgetTree:WidgetTree.RightColumn"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Before removal, `RightColumn` contained the moved `PrimaryAction` button.
The call returned `true`. `GetWidgets` then returned only `RootCanvas` and
`LeftColumn`, proving that the addressed panel and its child were removed as a
subtree. Removing `RootCanvas` returned `true` and left an empty tree.
`CompileWidgetBlueprint` returned `true` for both the populated moved tree and
the final empty tree.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Removal deletes the addressed widget and all descendants.
- The boolean result requires an independent `GetWidgets` absence and remaining-
  order check.
- Removing the root can leave a valid empty Widget Blueprint.
- Bindings, graph references, animations, generated variables, and named-slot
  relationships can outlive structural assumptions and need separate checks.
- Removal changes the unsaved Widget Blueprint and requires separate compile and
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

### `widget`

- Required: **yes**
- Type: `object`
- Purpose:

The widget to remove (along with its children).

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
  UMGToolSet.UMGToolSet.RemoveWidget \
  --arguments '
{
  "widget": {},
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
