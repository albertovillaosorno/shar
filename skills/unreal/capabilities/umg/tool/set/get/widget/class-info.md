# Get widget class info

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.GetWidgetClassInfo
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Returns the Category, Description and if it's a Panel for a single widget
class. Same per-entry data as ListWidgetClasses, but lets callers query a class
they already have without scanning every UClass. Returns an empty entry if
WidgetClass is null. Can be used to get more information on the class from the
Description and Category.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Use the returned structured evidence directly, but still confirm the live
schema because names do not prove side effects.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to inspect one exact widget class's category, editor description,
and panel capability before SHAR chooses hierarchy operations.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a stable widget class ref from `ListWidgetClasses` or `GetWidgets`.
- Avoid transient `REINST` and `SKEL` class refs for durable plans.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetClass": {"refPath": "/Script/UMG.VerticalBox"}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two VerticalBox calls returned category `Panel`, `bIsPanel: true`, and guidance
describing vertical automatic child layout. Two Image calls returned category
`Common`, `bIsPanel: false`, and no-children guidance.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `bIsPanel` describes child-hosting capability, not current child count.
- Descriptions are editor documentation and can change by engine revision.
- Generated or transient classes can disappear after recompilation.
- A missing class ref fails during parameter translation.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

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

## Inputs

### `widgetClass`

- Required: **yes**
- Type: `object`
- Purpose:

The widget class to inspect.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.GetWidgetClassInfo \
  --arguments '
{
  "widgetClass": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

UMGWidgetClassEntry

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Confirm the response belongs to the open editor project.
- Reject evidence derived from stale discovery state.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
