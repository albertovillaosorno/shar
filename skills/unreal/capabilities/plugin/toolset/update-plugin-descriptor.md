# Update plugin descriptor

[Return to the central Unreal MCP index](../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PluginToolset.PluginToolset.UpdatePluginDescriptor
```

Toolset:

```text
PluginToolset.PluginToolset
```

## What this tool does

Updates a plugin's descriptor fields and writes them to its .uplugin file.
Checks out the file via source control if source control is enabled. No-ops if
the serialized descriptor is unchanged (file is not touched).

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
Use this tool to update the complete editable descriptor of one task-owned
SHAR plugin.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  exact live toolset schema before mutation.
- Require plugin creation or modification permission and capture the project
  descriptor plus target plugin directory before mutation.
- Use a unique disposable project-plugin name and define exact
  plugin-directory and `.uproject` restoration before invocation.
- Read the complete editable descriptor and supply every required descriptor
  field.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "newDescriptor": {
    "bCanContainContent": true,
    "bIsBetaVersion": false,
    "bIsExperimentalVersion": false,
    "bIsSealed": false,
    "category": "SHAR Validation",
    "createdBy": "",
    "createdByURL": "",
    "description": "Updated disposable SHAR MCP plugin validation.",
    "docsURL": "",
    "friendlyName": "MCP Validation Plugin",
    "marketplaceURL": "",
    "supportURL": "",
    "version": 2,
    "versionName": "2.0"
  },
  "pluginName": "MCPValidationf079e33d"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`GetPluginDescriptor` changed every requested field exactly, including
friendly name, description, category, version `2`, and version name `2.0`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Plugin operations persist descriptor or project state and can require an
  editor restart before the live registry reflects every change.
- The setter requires the complete descriptor object, not a partial patch;
  preserve every field not intentionally changed.
- The validation plugin directory was deleted and the original `.uproject`
  SHA-256 was restored exactly.
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `newDescriptor`

- Required: **yes**
- Type: `object`
- Purpose:

The new descriptor field values to apply.

### `pluginName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the plugin to update.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PluginToolset.PluginToolset \
  PluginToolset.PluginToolset.UpdatePluginDescriptor \
  --arguments '
{
  "newDescriptor": {},
  "pluginName": "<value>"
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
