# Get plugin for asset

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PluginToolset.PluginToolset.GetPluginForAsset
```

Toolset:

```text
PluginToolset.PluginToolset
```

## What this tool does

Returns the name of the enabled plugin whose content mount point contains the
given asset path. Accepts full asset paths or mount point prefixes (e.g.
/PluginName/ or /Game/Path/To/Asset).

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
Use this tool to identify the enabled plugin that owns a discovered Unreal
asset before SHAR records provenance, chooses a plugin-specific toolset, or
checks whether an engine asset belongs outside `/Game`.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a full object path or plugin mount prefix.
- The owning plugin must be enabled and have a registered content mount.
- Confirm the asset independently when using a full object path.
- Project content under `/Game` is not owned by an enabled plugin mount.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "assetPath": "/Niagara/Modules/Spawn/Initialization/InitializeParticle.InitializeParticle"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The full object path and the `/Niagara/` mount prefix both returned
`Niagara`. `GetPluginInfo` independently reported the same mounted asset path.
AssetTools confirmed that the object exists and has class `NiagaraScript`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool resolves enabled plugin content mounts, not arbitrary package
  ownership.
- A `/Game` path with no plugin owner raises `No plugin contains asset`.
- A mount prefix is accepted even when no individual asset is supplied.
- Verify full asset existence separately because mount ownership alone does not
  prove the object exists.
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
shar-unreal-mcp describe PluginToolset.PluginToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `assetPath`

- Required: **yes**
- Type: `string`
- Purpose:

The asset or mount point path to look up.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PluginToolset.PluginToolset \
  PluginToolset.PluginToolset.GetPluginForAsset \
  --arguments '
{
  "assetPath": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the plugin that owns the asset.

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
