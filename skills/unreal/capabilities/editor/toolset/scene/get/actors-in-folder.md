# Get actors in folder

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.get_actors_in_folder
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Returns the actors in the specified outliner folder.

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
Use this tool to inspect one SHAR World Outliner group after discovering its
exact path. The verified example enumerated the environment actors grouped under
`Lighting` before scene and viewport checks.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and an active Level Editor world must be ready.
- Obtain the exact folder path from `get_folders` in the same loaded world.
- Set `recursive` explicitly so child-folder inclusion is deliberate.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "folder_path": "Lighting",
  "recursive": true
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned the same six actors: a directional light, a
static-mesh actor, sky atmosphere, sky light, exponential-height fog, and
volumetric cloud. Every returned reference belonged to the current level, and
`get_folders` independently reported the `Lighting` path.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Actor references belong to the currently loaded world and can include
  transient level paths and instance identifiers.
- `recursive` changes the target set when child folders exist; do not rely on
  its default when result cardinality matters.
- A folder result proves membership, not visibility in the active viewport.
  Use `GetVisibleActors` for that separate question.
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

### `folder_path`

- Required: **yes**
- Type: `string`
- Purpose:

The folder path to query (e.g. 'Lighting/Spotlights').

### `recursive`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, also includes actors in sub-folders.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.get_actors_in_folder \
  --arguments '
{
  "folder_path": "<value>"
}
'
```

## Expected output

A list of actors in the specified folder.

### `returnValue`

- Required: **yes**
- Type: `array<object>`
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
