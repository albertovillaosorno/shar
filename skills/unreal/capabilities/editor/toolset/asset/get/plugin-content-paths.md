# Get plugin content paths

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.asset.AssetTools.get_plugin_content_paths
```

Toolset:

```text
editor_toolset.toolsets.asset.AssetTools
```

## What this tool does

Returns the root content paths for plugins that have content.

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
Use this tool to establish which virtual plugin content roots are available to
SHAR asset discovery before searching outside `/Game`. The verified editor had
no project-plugin content roots, while engine-plugin inclusion exposed the
`/BaseMaterial/` fixture used for the remaining AssetTools checks.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and Asset Registry must be ready.
- Choose `include_engine` deliberately; it changes the discovery scope from
  project plugins to project and engine plugins.
- Treat returned values as virtual content roots, not filesystem paths.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "include_engine": true
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned the same 84 unique virtual roots and included
`/BaseMaterial/`. Two calls with `include_engine` set to `false` returned an
empty array, independently confirming that the current SHAR project has no
content-bearing project plugins.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The observed count depends on the Unreal version and enabled engine plugins;
  do not treat 84 as a fixed catalog size.
- Returned roots end with `/` and can represent plugins whose registry query
  currently returns no assets.
- `include_engine: false` excludes engine-plugin fixtures needed by the current
  asset-tool baseline.
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

### `include_engine`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

When True, also includes content plugins under the Engine directory.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.asset.AssetTools \
  editor_toolset.toolsets.asset.AssetTools.get_plugin_content_paths \
  --arguments '
{}
'
```

## Expected output

A list of virtual content root paths (e.g. ['/PluginName/']) for each in-scope
plugin that has a content directory.

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
