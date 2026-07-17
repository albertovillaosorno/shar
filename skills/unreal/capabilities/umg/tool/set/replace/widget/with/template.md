# Replace widget with template

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.ReplaceWidgetWithTemplate
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Replaces a widget instance in the blueprint's widget tree with a new instance
created from a different template widget class. Preserves references for
members that exist on both classes with a compatible type/signature: bindings,
BP graph variable references, animation bindings, and delegate bindings.
Members without a compatible counterpart on the new class are listed in the
returned report; references to those members in the outer blueprint will become
orphaned graph nodes / dangling bindings.

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
Use this tool to replace one reviewed SHAR widget class while preserving its
position and compatible Blueprint, binding, animation, and delegate references.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact widget and replacement class.
- Capture widget name, parent, slot, variable state, descendants, and
  references.
- Inventory the old and new class members used by the Blueprint.
- Define acceptance rules for every unmatched and referenced-member report.
- Compile and inspect the resulting tree before saving.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_TemplateReplace.WBP_MCP_TemplateReplace"
  },
  "widgetToReplace": {
    "refPath": "/Game/SHAR_MCP_Validation/WBP_MCP_TemplateReplace.WBP_MCP_TemplateReplace:WidgetTree.ActionControl"
  },
  "templateClass": {
    "refPath": "/Script/UMG.CheckBox"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
An unbound `Button` named `ActionControl` was replaced with a `CheckBox`.
The report returned `bSuccess: true`, an empty warning, four unmatched
functions,
nine unmatched properties, and zero referenced unmatched functions or
properties. `GetWidgets` preserved the name, root parent, and
`CanvasPanelSlot_0` while changing only the widget class. Compilation returned
`true`, and complete disposable cleanup left no asset or filesystem residue.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `bSuccess: true` does not mean all old-class members matched the new class.
- Inspect every unmatched array and the referenced subsets before acceptance.
- Nonzero unmatched counts can be safe only when the Blueprint does not use
  those members and project semantics do not depend on them.
- Widget name and outer slot were preserved in the validated case, but all
  relevant properties still require independent comparison.
- Replacement changes unsaved state and needs separate compile and save
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

### `templateClass`

- Required: **yes**
- Type: `object`
- Purpose:

The widget class to create the replacement from.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

The widget blueprint containing the widget to replace.

### `widgetToReplace`

- Required: **yes**
- Type: `object`
- Purpose:

The widget instance to replace (must be in WidgetBlueprint's tree).

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.ReplaceWidgetWithTemplate \
  --arguments '
{
  "templateClass": {},
  "widgetBlueprint": {},
  "widgetToReplace": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Report whose bSuccess flag indicates whether the replacement happened,
MissingReferencesWarning describes warnings that need action, and the
Unmatched* arrays list members that have no compatible counterpart on the new
class (with the *Referenced* subsets being the ones the blueprint actually uses
today).

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
