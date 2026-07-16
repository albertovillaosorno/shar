# Update metadata tags

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.asset.AssetTools.update_metadata_tags
```

Toolset:

```text
editor_toolset.toolsets.asset.AssetTools
```

## What this tool does

Sets or removes metadata tags on an asset.

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
Use this mutation to attach reviewed package metadata to a SHAR asset for local
provenance, workflow state, or bounded automation markers.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass an existing editable asset content path.
- Read current package metadata before setting values.
- Keep metadata distinct from Asset Registry tags.
- Save the asset after accepted metadata changes when persistence matters.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "asset_path": "/Game/SHAR_MCP_AssetLifecycle_8/ST_Original",
  "set_tags": {
    "SHARProbe": "AssetLifecycle",
    "Cycle": "8"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles returned JSON null and GetMetadataTags immediately read back
`SHARProbe` and `Cycle`. GetAssetTags did not expose those package metadata
values.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- This is a persistent package-metadata mutation and returns JSON null.
- Package metadata is separate from Asset Registry tags and FindAssets tag
  filters.
- Duplicating the asset preserved content but produced an empty metadata map.
- Metadata authored on a duplicate was preserved when that asset moved.
- Removing metadata is currently blocked by UStruct conversion whether
  `set_tags` is omitted, empty, or nonempty; do not claim removal success.
- Verify changes with GetMetadataTags rather than GetAssetTags.
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

Content path to the asset.

### `remove_tags`

- Required: **no**
- Type: `array<string>`
- Purpose:

Tag names to remove. Removing a tag that does not exist is a no-op.

### `set_tags`

- Required: **no**
- Type: `object`
- Purpose:

Tag names mapped to the values to set.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.asset.AssetTools \
  editor_toolset.toolsets.asset.AssetTools.update_metadata_tags \
  --arguments '
{
  "asset_path": "<value>"
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
