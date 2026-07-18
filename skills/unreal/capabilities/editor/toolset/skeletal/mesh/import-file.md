# Import file

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.import_file
```

Toolset:

```text
editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools
```

## What this tool does

Imports a mesh file from disk as a SkeletalMesh asset.

The source file must contain a skeleton hierarchy and skinned mesh data.

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
Use this tool to import one reviewed FBX as a skeletal asset using an exact
matching skeleton.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a task-owned skeletal-mesh copy and verify skeleton or Physics Asset
  compatibility before mutation.
- Capture the matching asset, class, existence, or assignment reader and
  define whole-folder cleanup.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "asset_name": "SK_MCP_Round50_Imported",
  "create_physics_asset": false,
  "folder_path": "/Game/SHAR_MCP_Validation_Round50_260718",
  "import_animations": true,
  "import_materials": false,
  "import_textures": false,
  "skeleton": {
    "refPath": "/Engine/Tutorial/SubEditors/TutorialAssets/Character/TutorialTPP_Skeleton.TutorialTPP_Skeleton"
  },
  "source_file": "C:/SHAR_MCP_Validation/round50_sequence.fbx"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The importer returned one existing object and ObjectTools identified it as
`/Script/Engine.SkeletalMesh`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Skeletal import and Physics Asset assignment are compatibility-sensitive; a
  true return alone is insufficient.
- Skeletal mesh, skeleton, Physics Asset, and imported UObject references
  become stale after cleanup.
- The source FBX and supplied skeleton must be compatible. The return may
  contain more than one imported UObject for richer files.
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
shar-unreal-mcp describe editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `asset_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the new asset.

### `create_physics_asset`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

When True, create a PhysicsAsset bound to the imported mesh's skeleton.

### `folder_path`

- Required: **yes**
- Type: `string`
- Purpose:

The content-browser folder to create the asset in.

### `import_animations`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

When True, create AnimSequence assets for any animations contained in the file.

### `import_materials`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

When True, create Material assets for any materials referenced in the file.
When False, the imported mesh has no materials assigned.

### `import_textures`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

When True, create Texture2D assets for any textures referenced by the imported
materials. Only effective when import_materials is True.

### `skeleton`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `source_file`

- Required: **yes**
- Type: `string`
- Purpose:

The absolute path to the source mesh file on disk.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools \
  editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.import_file \
  --arguments '
{
  "asset_name": "<value>",
  "folder_path": "<value>",
  "source_file": "<value>"
}
'
```

## Expected output

The assets produced by the import. The first entry is the imported
SkeletalMesh; additional entries may include a newly created Skeleton (when
none was supplied), a PhysicsAsset, AnimSequences, Materials, and Texture2Ds,
depending on which options are enabled.

### `returnValue`

- Required: **yes**
- Type: `array<object>`
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
