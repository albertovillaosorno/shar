# Import file

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.texture.TextureTools.import_file
```

Toolset:

```text
editor_toolset.toolsets.texture.TextureTools
```

## What this tool does

Imports an image file from disk as a Texture2D asset.

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
Use this tool to import one small reviewed image as a Texture2D asset.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a tiny task-owned image at an absolute readable path and a unique
  disposable destination name.
- Capture asset existence, class, and pixel dimensions as independent
  postconditions.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "asset_name": "T_MCP_Round50",
  "folder_path": "/Game/SHAR_MCP_Validation_Round50_260718",
  "source_file": "C:/SHAR_MCP_Validation/mcp_round50.png"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The imported object existed as Texture2D and `get_size` returned the exact
authored dimensions 4 by 3 pixels.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Import can overwrite or rename colliding destinations; require a unique
  asset name and exact returned identity.
- Texture class and dimensions prove import shape, not color-management or
  compression correctness.
- The source must be an absolute readable image path. This validation covered
  one 4 by 3 RGBA PNG.
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
shar-unreal-mcp describe editor_toolset.toolsets.texture.TextureTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `asset_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the new asset.

### `folder_path`

- Required: **yes**
- Type: `string`
- Purpose:

The content-browser folder to create the asset in.

### `source_file`

- Required: **yes**
- Type: `string`
- Purpose:

The absolute path to the source image file on disk.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.texture.TextureTools \
  editor_toolset.toolsets.texture.TextureTools.import_file \
  --arguments '
{
  "asset_name": "<value>",
  "folder_path": "<value>",
  "source_file": "<value>"
}
'
```

## Expected output

The assets produced by the import (typically a single Texture2D).

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
