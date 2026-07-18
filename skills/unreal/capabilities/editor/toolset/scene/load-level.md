# Load level

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.load_level
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Loads a level in the editor.

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
Use this tool to switch the editor into one exact saved SHAR map before
map-scoped inspection, scene mutation, or World Partition validation, then
return to the previously captured level.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `doctor` readiness and refresh the live SceneTools schema.
- Save the current level before invocation; the native operation refuses to
  replace a level that has unsaved changes.
- Verify the target map asset exists and record the current level as the exact
  return target before loading.
- Treat actors, bindings, and Slate refs from the prior world as stale after
  the switch.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "level_path": "/Game/SHAR_MCP_Validation_Final_WP/Map_MCP_Final_WP"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The native call returned no structured value. Independent
`get_current_level` reads changed from `/Game/Untitled` to the exact disposable
map path, and a later load restored `/Game/Untitled` before deleting the
fixture.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A `null` return value does not prove that the map changed; compare exact
  `get_current_level` values before and after every call.
- The tool raises when the current level has unsaved changes instead of opening
  an interactive save prompt.
- A successful load invalidates world-owned actor, component, binding, and
  widget refs from the previous level.
- Return to a different level before deleting or replacing the loaded map
  package.
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
shar-unreal-mcp describe editor_toolset.toolsets.scene.SceneTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `level_path`

- Required: **yes**
- Type: `string`
- Purpose:

The path to the level asset.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.load_level \
  --arguments '
{
  "level_path": "<value>"
}
'
```

## Expected output

The live interface does not declare a structured output schema.

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
