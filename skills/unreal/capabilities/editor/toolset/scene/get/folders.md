# Get folders

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.get_folders
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Returns all folder paths currently in use in the outliner.

Includes all intermediate parent paths. For example, if an actor is assigned to
'Lighting/Spotlights', both 'Lighting' and 'Lighting/Spotlights' are returned.

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
Use this tool to inspect SHAR World Outliner organization before folder-scoped
validation or automation. The verified level exposed the `HLOD`,
`HLOD/HLOD0_Instancing`, and `Lighting` paths used by its generated world and
environment actors.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- A Level Editor world with initialized World Outliner state must be loaded.
- Read the current level first when folder evidence will drive another scene
  operation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned the same three folder paths. An independent
`get_actors_in_folder` query for `Lighting` with recursion enabled returned six
actors, confirming that the reported folder was active in the loaded world.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Intermediate parent paths are included. The `HLOD` parent and
  `HLOD/HLOD0_Instancing` child therefore count as separate returned strings.
- Results describe only the currently loaded world and can change when actors
  move between folders or another level is loaded.
- Folder paths do not identify their actors; use `get_actors_in_folder` for that
  follow-up.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe editor_toolset.toolsets.scene.SceneTools
```

1. Confirm every required input against the current schema.

## Inputs

This tool accepts no input fields.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.get_folders \
  --arguments '
{}
'
```

## Expected output

A sorted list of unique folder path strings.

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
