# Get plugin info

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PluginToolset.PluginToolset.GetPluginInfo
```

Toolset:

```text
PluginToolset.PluginToolset
```

## What this tool does

Gets metadata for a discovered plugin, including description, version, base
directory, content directory, descriptor path, and mounted asset path.

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
Use this tool to inspect the installed native MCP plugin version and content
mount before SHAR regenerates Unreal skills, diagnoses plugin drift, or resolves
plugin-owned assets.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain the exact plugin name from plugin discovery.
- The plugin must be installed and discoverable; it need not be inferred from a
  filesystem path.
- Do not persist returned machine-relative installation directories in skills.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "pluginName": "ModelContextProtocol"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated calls returned identical metadata: version `1`, version name `1.0`,
mount `/ModelContextProtocol/`, and the native MCP description.
`GetPluginDescriptor` returned matching version and description fields, while
`IsEnabled` returned true. The generator normalized `1.0` to revision `1.0.0`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Directory and descriptor fields are installation-relative and should not be
  copied into durable project guidance.
- The raw plugin `versionName` is `1.0`; skill revision generation normalizes it
  to three-part `1.0.0`.
- Metadata does not prove enablement; use `IsEnabled` separately.
- Content mount information can be present even when a plugin contains no
  content assets.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

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

### `pluginName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the plugin.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PluginToolset.PluginToolset \
  PluginToolset.PluginToolset.GetPluginInfo \
  --arguments '
{
  "pluginName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Metadata for the plugin.

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
