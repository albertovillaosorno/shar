# Delete folder

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.delete_folder
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Deletes a folder from the outliner.

Actors directly in the folder are moved to the parent folder. Sub-folders and
their actors are preserved by re-rooting them under the parent. For example,
deleting 'Lighting' with a sub-folder 'Lighting/Spotlights' leaves 'Spotlights'
intact under the parent.

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
Use this tool to remove one emptyable SHAR World Outliner folder after its
direct actors and subfolder behavior have been completely inventoried.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Confirm the exact folder exists through `get_folders`.
- Capture every directly contained actor and all subfolders before deletion.
- Determine the parent folder or root destination that will receive moved
  actors.
- Bound the expected moved-actor count and define independent membership checks
  for the resulting folder state.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "folder_path": "SHAR_MCP_Validation_Renamed"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The temporary folder contained exactly one `PlayerStart` actor and no
subfolders. The call returned `returnValue: 1`. `get_folders` no longer returned
the deleted path. Enumerating every remaining folder showed the actor belonged
to none, proving that it had moved back to the Outliner root. Final cleanup
returned the same original three-folder inventory.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Actors directly in the deleted folder move to its parent rather than being
  deleted.
- Child folders are re-rooted under the parent, so deletion can change more than
  one visible path even when the returned count is small.
- The returned integer counts moved actors, not deleted folders or subfolders.
- Deleting a missing folder raises an error.
- Folder deletion changes loaded level state and can persist if the level is
  saved.
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

### `folder_path`

- Required: **yes**
- Type: `string`
- Purpose:

The folder path to delete (e.g. 'Lighting/Spotlights').

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.delete_folder \
  --arguments '
{
  "folder_path": "<value>"
}
'
```

## Expected output

The number of actors that were moved.

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
