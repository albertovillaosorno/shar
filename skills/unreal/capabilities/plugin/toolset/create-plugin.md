# Create plugin

[Return to the central Unreal MCP index](../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PluginToolset.PluginToolset.CreatePlugin
```

Toolset:

```text
PluginToolset.PluginToolset
```

## What this tool does

Creates a new plugin from a template and loads it into the editor. Use
GetPluginTemplateDescriptions to obtain a valid TemplateInfo.

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
Use this tool to create one temporary project plugin from an approved Unreal
template before validating descriptor and dependency workflows.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  exact live toolset schema before mutation.
- Require plugin creation or modification permission and capture the project
  descriptor plus target plugin directory before mutation.
- Use a unique disposable project-plugin name and define exact
  plugin-directory and `.uproject` restoration before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "bPlaceInEngine": false,
  "description": "Disposable SHAR MCP plugin validation.",
  "pluginName": "MCPValidationf079e33d",
  "relativePluginLocation": "",
  "templateInfo": {
    "bCanBePlacedInEngine": true,
    "defaultTemplateName": "",
    "description": "Create a blank plugin that can only contain content.",
    "name": "Content Only",
    "onDiskPath": "../../../Engine/Plugins/Editor/PluginBrowser/Templates/ContentOnly"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The unique plugin directory changed from absent to present. Plugin discovery,
metadata, descriptor, mounted asset path, and enabled state all matched the
returned Content Only plugin identity.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Plugin operations persist descriptor or project state and can require an
  editor restart before the live registry reflects every change.
- The returned descriptor and metadata paths can expose private local
  filesystem prefixes; repository guidance must retain only portable
  identities.
- A created plugin remains discoverable in the current editor process after
  physical deletion until plugin discovery refreshes or the editor restarts.
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

### `bPlaceInEngine`

- Required: **yes**
- Type: `boolean`
- Purpose:

Use Engine Plugins directory rather than Game Plugins directory location. Only
some Templates allow placing in Engine. See the TemplateInfo's
bCanBePlacedInEngine. This should be false unless explicitly requested by the
user.

### `description`

- Required: **yes**
- Type: `string`
- Purpose:

A description for the new plugin.

### `pluginName`

- Required: **yes**
- Type: `string`
- Purpose:

Name for the new plugin.

### `relativePluginLocation`

- Required: **yes**
- Type: `string`
- Purpose:

Parent directory for the new plugin relative to template's default location.
This should be empty unless you wish to specify a subdirectory.

### `templateInfo`

- Required: **yes**
- Type: `object`
- Purpose:

The plugin template to create from.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PluginToolset.PluginToolset \
  PluginToolset.PluginToolset.CreatePlugin \
  --arguments '
{
  "bPlaceInEngine": false,
  "description": "<value>",
  "pluginName": "<value>",
  "relativePluginLocation": "<value>",
  "templateInfo": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

Created plugin's descriptor filename. Empty on failure.

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
