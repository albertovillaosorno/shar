# Create widget blueprint

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
UMGToolSet.UMGToolSet.CreateWidgetBlueprint
```

Toolset:

```text
UMGToolSet.UMGToolSet
```

## What this tool does

Creates a new Widget Blueprint asset. Returns the blueprint or nullptr on
failure.

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
Use this tool to create one typed SHAR Widget Blueprint for reviewed HUD,
menu, overlay, prompt, debug UI, or disposable widget-tree validation.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact `UUserWidget` parent class required by the UI contract.
- Verify that the destination folder and asset path are absent with AssetTools.
- Choose one exact asset name and define parent, generated-class, compile, and
  existence checks.
- Define deletion of the asset and any newly introduced empty virtual folder
  before a disposable validation call.
- Determine separately whether the Widget Blueprint must be saved to disk.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "folderPath": "/Game/SHAR_MCP_Validation",
  "assetName": "WBP_MCP_Validation",
  "parentClass": {
    "refPath": "/Script/UMG.UserWidget"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The destination asset was absent before creation. The call returned the Widget
Blueprint object reference, while `find_assets` returned its package path.
`exists` returned `true`, `get_asset_class` returned
`WBP_MCP_Validation_C`, and Blueprint parent inspection returned exactly
`/Script/UMG.UserWidget`. UMG compilation returned `true`. Deleting the Widget
Blueprint and its remaining virtual folder restored all existence checks. No
content directory or `.uasset` file appeared during the unsaved test.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The validated Widget Blueprint was empty and remained unsaved in memory.
- The returned object reference includes the object suffix, while AssetTools
  searches and deletion use the package path.
- `get_asset_class` reports the generated Widget Blueprint class name.
- Creating an asset in a new path also creates a virtual Content Browser folder
  that can remain after asset deletion.
- Widget-tree authoring, binding validation, animation, compilation, runtime
  creation, and saving require separate verification stages.
- Persistent project widgets must not be deleted as cleanup unless they were
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
shar-unreal-mcp describe UMGToolSet.UMGToolSet
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `assetName`

- Required: **yes**
- Type: `string`
- Purpose:

Name for the new blueprint asset.

### `folderPath`

- Required: **yes**
- Type: `string`
- Purpose:

Content folder path, e.g. "/Game/UI/Widgets".

### `parentClass`

- Required: **yes**
- Type: `object`
- Purpose:

The parent UUserWidget class. Get this from GetWidgets Info.ParentClass on the
source blueprint.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  UMGToolSet.UMGToolSet \
  UMGToolSet.UMGToolSet.CreateWidgetBlueprint \
  --arguments '
{
  "assetName": "<value>",
  "folderPath": "<value>",
  "parentClass": {}
}
'
```

## Expected output

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
