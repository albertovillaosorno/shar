# Set actor folder

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.set_actor_folder
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Assigns an actor to the specified folder in the outliner.

Creates the folder implicitly if it does not already exist. Pass an empty
string to move the actor to the root of the outliner.

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
Use this tool to place one exact SHAR scene actor into a deterministic World
Outliner folder before bounded organization, review, or folder-scoped queries.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must have the target level loaded.
- Resolve the exact actor through a current scene read.
- Enumerate folders with `get_folders` and capture the actor's current folder
  membership through `get_actors_in_folder`.
- Confirm the destination path is absent or intentionally shared.
- Define restoration to the captured folder; use an empty path only when root
  placement has been proven.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "actor": {
    "refPath": "/Temp/Untitled_1.Untitled_1:PersistentLevel.PlayerStart_UAID_F02F74551BF5599B01_1153002503"
  },
  "folder_path": "SHAR_MCP_Validation"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The actor was proven at the Outliner root by enumerating every current folder
and confirming it belonged to none. The call returned `returnValue: null`.
`get_folders` then included `SHAR_MCP_Validation`, and an independent
`get_actors_in_folder` call returned exactly the requested `PlayerStart` actor.
The subsequent rename-delete cycle restored root placement.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool creates the destination folder implicitly when it does not exist.
- `get_actors_in_folder` rejects an empty path, so root placement must be proven
  by excluding the actor from every path returned by `get_folders`.
- An empty destination moves the actor to the Outliner root.
- Folder assignment changes loaded level state and can persist if the level is
  saved.
- Actor references from `/Temp` worlds are session-specific.
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

### `actor`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `folder_path`

- Required: **yes**
- Type: `string`
- Purpose:

The folder path to assign (e.g. 'Lighting/Spotlights'). Pass an empty string to
move the actor to the root.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.set_actor_folder \
  --arguments '
{
  "actor": {},
  "folder_path": "<value>"
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
