# Validate new plugin name and location

[Return to the central Unreal MCP index](../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PluginToolset.PluginToolset.ValidateNewPluginNameAndLocation
```

Toolset:

```text
PluginToolset.PluginToolset
```

## What this tool does

Validates that PluginName and RelativePluginLocation are acceptable for a new
plugin.

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
Use this tool to preflight a proposed SHAR project-plugin name, optional plugin
subdirectory, placement choice, and exact live template before invoking
`CreatePlugin`. The validation call is non-mutating and is suitable for safely
rejecting bad candidates before any files are written.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- `IsPluginCreationAllowed` must return `true` in the same editor session.
- Obtain `templateInfo` from `GetPluginTemplateDescriptions` and preserve every
  returned field exactly.
- Keep `bPlaceInEngine` false unless engine placement is explicitly required and
  the selected template permits it.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "pluginName": "SharMcpValidationProbe",
  "relativePluginLocation": "Validation",
  "bPlaceInEngine": false,
  "templateInfo": {
    "name": "Content Only",
    "description": "Create a blank plugin that can only contain content.",
    "onDiskPath": "../../../Engine/Plugins/Editor/PluginBrowser/Templates/ContentOnly",
    "defaultTemplateName": "",
    "bCanBePlacedInEngine": true
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls with the validated arguments returned `true`. No plugin or
folder was created. Separate probes rejected an existing plugin name, an empty
name, a name containing spaces, and engine placement with a template that does
not permit engine placement.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Invalid candidates raise a native tool error instead of returning `false` in
  the verified editor session.
- The template descriptor must exactly match one returned by live discovery; a
  reconstructed descriptor with an invented path is rejected.
- `true` proves only the current preflight result. It does not create the
  plugin,
  prove later filesystem writes, compile modules, save packages, or prevent the
  name from becoming occupied before creation.
- Engine placement is rejected when `bCanBePlacedInEngine` is false.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

<!-- markdownlint-disable-next-line MD013 -->
- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe PluginToolset.PluginToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `bPlaceInEngine`

- Required: **yes**
- Type: `boolean`
- Purpose:

Use Engine Plugins directory rather than Game Plugins directory location. Only
some Templates allow placing in Engine. See the TemplateInfo's
bCanBePlacedInEngine. This should be false unless explicitly requested by the
user.

### `pluginName`

- Required: **yes**
- Type: `string`
- Purpose:

The proposed plugin name.

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

The plugin template to potentially create from.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PluginToolset.PluginToolset \
  PluginToolset.PluginToolset.ValidateNewPluginNameAndLocation \
  --arguments '
{
  "bPlaceInEngine": false,
  "pluginName": "<value>",
  "relativePluginLocation": "<value>",
  "templateInfo": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

True if the name and location are valid.

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
