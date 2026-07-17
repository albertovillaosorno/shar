# Rename folder

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.rename_folder
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Renames a folder in the outliner.

Updates the folder path for all actors in the folder and any sub-folders. For
example, renaming 'Lighting' to 'Lights' also updates actors in
'Lighting/Spotlights' to 'Lights/Spotlights'. If the new path already exists
then the affected actors will be merged into it.

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
Use this tool to rename one fully inventoried SHAR World Outliner folder while
preserving its actor membership and nested folder relationships.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Enumerate all current folder paths and actors before the rename.
- Confirm the source exists and the destination does not exist unless an
  intentional merge has been approved.
- Bound the expected affected actor count, including actors under subfolders.
- Define the inverse rename or another exact restoration path before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "old_path": "SHAR_MCP_Validation",
  "new_path": "SHAR_MCP_Validation_Renamed"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The source folder contained exactly one actor. The call returned
`returnValue: 1`. The old path then failed as nonexistent, `get_folders`
contained only the renamed temporary path, and `get_actors_in_folder` returned
that same `PlayerStart` actor under the new path. Deleting the renamed folder
returned the actor to its proven original root placement.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The operation recursively updates actors in child folders, so the affected
  scope can exceed the direct source-folder membership.
- Renaming to an existing path merges folder contents; that behavior was not
  exercised.
- The returned integer must match the precomputed affected actor count.
- Folder renames alter loaded level state and can persist if the level is saved.
- A missing source path is reported as an error rather than an empty result.
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

### `new_path`

- Required: **yes**
- Type: `string`
- Purpose:

The new folder path (e.g. 'Lights').

### `old_path`

- Required: **yes**
- Type: `string`
- Purpose:

The current folder path (e.g. 'Lighting').

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.rename_folder \
  --arguments '
{
  "new_path": "<value>",
  "old_path": "<value>"
}
'
```

## Expected output

The number of actors whose folder path was updated.

### `returnValue`

- Required: **yes**
- Type: `integer`
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
