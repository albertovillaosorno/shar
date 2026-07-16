# Capture asset image

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.EditorAppToolset.CaptureAssetImage
```

Toolset:

```text
EditorToolset.EditorAppToolset
```

## What this tool does

Renders a thumbnail for the specified asset (e.g. static meshes, skeletal
meshes, skeletons, animations, montages, materials, textures).

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
Use this tool to obtain a bounded visual preview of a known Unreal asset before
SHAR accepts an import, compares generated output, or selects an asset for a
more specific metadata or editor inspection workflow.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Confirm the exact virtual asset path with `AssetTools.exists` or bounded asset
  discovery before capture.
- Use an asset type whose thumbnail or preview renderer is available.
- Treat the image as review evidence only; preserve structured asset validation
  separately.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "assetPath": "/Engine/EngineMaterials/DefaultMaterial.DefaultMaterial"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`AssetTools.exists` first returned `true` for the fixture. Two capture calls
then returned byte-identical valid `image/png` payloads at 256 by 256 pixels.
A missing engine asset path raised `Asset not found` and produced no image.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` contains base64 PNG data and a MIME type; decode it in memory or
  route temporary review output outside tracked repository content.
- Preview appearance and dimensions depend on the asset class, thumbnail
  renderer, preview scene, cache, and engine version.
- A valid preview does not prove source provenance, package validity, material
  correctness, animation behavior, collision, or runtime integration.
- Missing assets raise a native error rather than returning an empty image.
- Repeated previews can be stable for a static cached asset, but byte identity
  is
  not a general contract for every asset type.
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
shar-unreal-mcp describe EditorToolset.EditorAppToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `assetPath`

- Required: **yes**
- Type: `string`
- Purpose:

The path to the asset, e.g. '/Game/Meshes/SM_Cube'.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.CaptureAssetImage \
  --arguments '
{
  "assetPath": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

An image of the asset.

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
