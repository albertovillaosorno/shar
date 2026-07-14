# Get asset class

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.asset.AssetTools.get_asset_class
```

Toolset:

```text
editor_toolset.toolsets.asset.AssetTools
```

## What this tool does

Gets the class of an asset.

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
Use this tool to confirm an asset class before routing SHAR validation to a
class-specific toolset. The verified BaseMaterial fixture resolved as a
`MaterialFunction` before tag and dependency inspection.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and Asset Registry must be ready.
- Obtain an exact virtual asset path from `find_assets` or another registry
  read.
- Confirm existence first when the caller cannot tolerate a missing asset.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "asset_path": "/BaseMaterial/Materials/Functions/MF_Rotate2D"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned `MaterialFunction`. A separate `find_assets` call using
`/Script/Engine.MaterialFunction` as the class reference returned the same
fixture asset exactly once.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The result is the class name, not the full class reference path.
- Class names for generated Blueprint assets can include generated-class
  conventions that differ from the package name.
- The verified fixture is engine-plugin content, not an authored SHAR asset.
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
shar-unreal-mcp describe editor_toolset.toolsets.asset.AssetTools
```

1. Confirm every required input against the current schema.

## Inputs

### `asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Content path to the asset.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.asset.AssetTools \
  editor_toolset.toolsets.asset.AssetTools.get_asset_class \
  --arguments '
{
  "asset_path": "<value>"
}
'
```

## Expected output

The class name of the asset (e.g. 'StaticMesh', 'Material', 'HeroCharacter_C').

### `returnValue`

- Required: **yes**
- Type: `string`
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
