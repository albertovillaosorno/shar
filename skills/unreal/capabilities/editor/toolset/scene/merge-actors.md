# Merge actors

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.merge_actors
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Merges multiple StaticMesh actors into a single mesh asset and actor.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to merge reviewed StaticMeshActors into one generated mesh asset
and scene actor.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  exact live toolset schema before mutation.
- Use only disposable actors in the unsaved validation level and capture
  current-level and actor inventories before mutation.
- Define actor removal and generated-asset deletion before invocation.
- Keep source actors when validating and include the base asset name in
  `output_path`.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "actors": [
    {
      "refPath": "/Temp/Untitled_1.Untitled_1:PersistentLevel.StaticMeshActor_UAID_00E04C68026738EF02_1135974735"
    },
    {
      "refPath": "/Temp/Untitled_1.Untitled_1:PersistentLevel.StaticMeshActor_UAID_00E04C68026738EF02_1137303736"
    }
  ],
  "destroy_source_actors": false,
  "name": "MCP_Merged_6e1b507e",
  "output_path": "/Game/SHAR_MCP_Validation_Static_6e1b507e/MCP_Merged_6e1b507e"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The exact generated mesh package changed from absent to present. The returned
actor referenced that mesh, actor discovery found the same identity, and mesh
readers reported non-zero vertices, triangles, and valid bounds.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Scene actor references are transient level identities and become invalid
  after actor removal or level replacement.
- The reproduced lifecycle ran in unsaved `/Temp/Untitled_1` and removed every
  validation actor afterward.
- `output_path` is a package base, not only a destination folder; Unreal
  prefixes its final segment with `SM_`.
- The operation creates both an asset and an actor. Delete both, and keep
  source actors unless destruction is explicitly intended.
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

### `actors`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The StaticMeshActors to merge. All must be in the same level.

### `destroy_source_actors`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, removes the source actors after merging.

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

Label for the new merged actor. Defaults to the last segment of output_path.

### `output_path`

- Required: **yes**
- Type: `string`
- Purpose:

Content path for the merged mesh asset (e.g. '/Game/Meshes/Merged').

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.merge_actors \
  --arguments '
{
  "actors": [],
  "name": "<value>",
  "output_path": "<value>"
}
'
```

## Expected output

The merged StaticMeshActor.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

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
