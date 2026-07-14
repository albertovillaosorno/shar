# Get widget tree depth

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.GetWidgetTreeDepth
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Returns the maximum depth of the widget tree. Depth: root with no children = 0;
root + children = 1; etc.

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
Use this tool to measure maximum descendant depth for a complete WidgetBlueprint
tree or one current start widget.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact loaded WidgetBlueprint object ref.
- Pass `startWidget: null` for the complete tree.
- Obtain a non-null start widget from `GetWidgets` on the same blueprint.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "widgetBlueprint": {"refPath": "/AudioWidgets/AudioFader/AudioFader.AudioFader"},
  "startWidget": null
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two full-tree calls returned depth `2`. Starting at root `VerticalBox_0` also
returned `2`; leaf `AudioTextBox` returned `0`; `CanvasPanel_3` returned `1`. A
start widget from another WidgetBlueprint raised an ownership error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Depth counts edges: a leaf is zero and a root with grandchildren is two.
- Named-slot content is a separate relationship and may not follow ordinary tree
  depth.
- Nested widget refs become stale after compilation or structural edits.
- A foreign start widget raises an error rather than returning `-1`.
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

### `startWidget`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `widgetBlueprint`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.GetWidgetTreeDepth \
  --arguments '
{
  "widgetBlueprint": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `integer`
- Purpose:

Depth as int32; -1 on error.

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
