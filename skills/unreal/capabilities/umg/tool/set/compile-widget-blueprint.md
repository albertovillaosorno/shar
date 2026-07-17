# Compile widget blueprint

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.CompileWidgetBlueprint
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Compiles a widget blueprint. Returns false with error details if compilation
fails. Errors include missing BindWidget bindings, type mismatches, and graph
errors. Call after all widgets and properties are set. Save separately via
AssetTools.save_asset.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Confirm execution scope, cancellation behavior, and expected side effects
before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool after bounded SHAR widget-tree, property, binding, animation, or
graph changes to prove that the Widget Blueprint remains compilable.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply an exact Widget Blueprint reference from current editor state.
- Complete all intended widget-tree, binding, and graph mutations first.
- Define independent parent, class, widget-tree, binding, or log checks for
  the requested UI outcome.
- Treat compilation and saving as separate operations.
- Keep the exact asset deletion path available for disposable validation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_Validation.WBP_MCP_Validation"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned `returnValue: true`. Independent Blueprint inspection
confirmed `/Script/UMG.UserWidget` as the parent, while asset-class inspection
returned the generated `WBP_MCP_Validation_C` class. Compilation created no
content directory or `.uasset` file. The unsaved Widget Blueprint and its
virtual folder were deleted afterward, leaving no registry or filesystem
residue.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `true` proves compile completion, not UI behavior, layout correctness,
  binding semantics, animation playback, accessibility, or runtime creation.
- Missing `BindWidget` fields, type mismatches, and graph errors can reject the
  call.
- Compilation does not save the asset; use the reviewed save capability
  separately when persistence is required.
- Inspect widget structure and editor logs when the task needs stronger evidence
  than the boolean compile result.
- The validated Blueprint had an empty widget tree, so populated-tree behavior
  remains separately reviewable.
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

The widget blueprint to compile.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.CompileWidgetBlueprint \
  --arguments '
{
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
