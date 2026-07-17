# Create

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.data_asset.DataAssetTools.create
```

Toolset:

```text
editor_toolset.toolsets.data_asset.DataAssetTools
```

## What this tool does

Creates a new DataAsset asset in the project.

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
Use this tool to create one typed SHAR DataAsset for reviewed configuration,
registry, gameplay-data, or disposable integration validation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact concrete DataAsset class and confirm it is appropriate
  for the requested project contract.
- Verify that the destination folder and asset path are absent with AssetTools.
- Choose one exact asset name and define class, registry, and existence checks.
- Define deletion of both the created asset and any newly introduced empty
  folder before a disposable validation call.
- Determine separately whether the asset must be saved to disk.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "folder_path": "/Game/SHAR_MCP_Validation",
  "asset_name": "DA_MCP_Validation",
  "asset_type": {
    "refPath": "/Script/Engine.PrimaryDataAsset"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The destination folder and asset were absent before creation. The call
returned `/Game/SHAR_MCP_Validation/DA_MCP_Validation.DA_MCP_Validation`.
`find_assets` returned the package path, `exists` returned `true`, and
`get_asset_class` returned `PrimaryDataAsset`. No `.uasset` or content directory
appeared on disk during the unsaved test. Deleting the asset returned `true` and
removed it from the registry. The virtual folder remained until a separate
folder delete, after which both existence checks were false and no filesystem
residue existed.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Creation registered an in-memory unsaved asset in the verified case; it did
  not by itself create a `.uasset` file.
- The returned object reference includes the object suffix, while AssetTools
  searches and deletion use the package path.
- Creating an asset in a new path also creates a virtual Content Browser folder
  that can remain after the asset is deleted.
- Saving, editing properties, compilation, and source-control behavior require
  separate tools and verification.
- Use only a concrete DataAsset subclass; abstract or incompatible classes may
  be rejected.
- Persistent project assets must not be deleted as cleanup unless they were
  created by the same bounded disposable operation.
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
shar-unreal-mcp describe editor_toolset.toolsets.data_asset.DataAssetTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `asset_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the asset in the folder.

### `asset_type`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `folder_path`

- Required: **yes**
- Type: `string`
- Purpose:

The path to the folder that will contain the asset.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.data_asset.DataAssetTools \
  editor_toolset.toolsets.data_asset.DataAssetTools.create \
  --arguments '
{
  "asset_name": "<value>",
  "asset_type": {},
  "folder_path": "<value>"
}
'
```

## Expected output

The DataAsset that was created.

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
