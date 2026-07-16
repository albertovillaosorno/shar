# Find assets

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.asset.AssetTools.find_assets
```

Toolset:

```text
editor_toolset.toolsets.asset.AssetTools
```

## What this tool does

Searches the project for assets that match specific criteria.

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
Use this tool to locate one exact registry asset before SHAR class, tag,
metadata, dependency, or referencer inspection. The verified fixture was the
enabled BaseMaterial function `MF_Rotate2D`; a separate `/Game` search confirmed
that the current project has no authored assets yet.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and Asset Registry must be ready.
- Supply the narrowest virtual folder root and a meaningful name filter.
- Keep `recursive` explicit when a folder has subfolders.
- Use `get_plugin_content_paths` before searching plugin roots.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "folder_path": "/BaseMaterial",
  "name": "MF_Rotate2D",
  "recursive": true
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned only `/BaseMaterial/Materials/Functions/MF_Rotate2D`.
A lowercase name produced the same result, while a missing name returned an
empty array. Independent existence, class, and exact registry-tag checks
confirmed the returned asset.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `folder_path` and `name` are required by the current live schema even when an
  empty string is accepted.
- An empty folder path broadens the search to `/Game` and plugin content, and
  this tool has no result-limit argument.
- Name matching was case-insensitive in the verified editor.
- Results are virtual asset paths without file extensions.
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
shar-unreal-mcp describe editor_toolset.toolsets.asset.AssetTools
```

1. Confirm every required input against the current schema.

## Inputs

### `asset_type`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `folder_path`

- Required: **yes**
- Type: `string`
- Purpose:

The folder to search within. Pass an empty string to search the entire project
(/Game/ and all plugin content folders including project plugins and engine
plugins).

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

If set, will only return assets whose name contains this string (case-
insensitive).

### `recursive`

- Required: **no**
- Type: `boolean`
- Default: `true`
- Purpose:

Whether to search subfolders or not.

### `tags`

- Required: **no**
- Type: `object`
- Purpose:

If set, will only return assets whose asset registry tags contain all specified
key-value pairs with exact value matches.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.asset.AssetTools \
  editor_toolset.toolsets.asset.AssetTools.find_assets \
  --arguments '
{
  "folder_path": "<value>",
  "name": "<value>"
}
'
```

## Expected output

A list of asset paths that match the criteria.

### `returnValue`

- Required: **yes**
- Type: `array<string>`
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
