# Is game feature plugin

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GameFeaturesToolset.GameFeaturesToolset.IsGameFeaturePlugin
```

Toolset:

```text
GameFeaturesToolset.GameFeaturesToolset
```

## What this tool does

Return whether or not a plugin is a Game Feature Plugin. Will error if no
plugin of this name can be found by the Plugin Manager.

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
Use this tool to distinguish an ordinary Unreal plugin from a Game Feature
Plugin before SHAR calls GFP state or transition tools. This prevents a valid
plugin name from being misrouted into the Game Features subsystem.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- Confirm the exact plugin name through `PluginToolset.ListDiscoveredPlugins` or
  another Plugin Manager read.
- Treat a `false` result as a routing decision, not as a missing-plugin result.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "pluginName": "Niagara"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned `false`. Independent PluginToolset reads
confirmed that `Niagara` was discovered, enabled, and owned the verified
`/Niagara/...` asset mount. A deliberately missing plugin name raised a native
could-not-find-plugin error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `false` means the named discovered plugin is not a Game Feature Plugin; it
  does not mean the plugin is disabled or unavailable.
- A missing plugin name raises an error instead of returning `false`.
- `GetGameFeatureState` and `IsGameFeatureActive` rejected the ordinary
  `Niagara` plugin as `GFP not found`.
- Plugin classification can change only with plugin descriptor or installation
  changes; rediscover after such changes or an editor restart.
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
shar-unreal-mcp describe GameFeaturesToolset.GameFeaturesToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `pluginName`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the plugin

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GameFeaturesToolset.GameFeaturesToolset \
  GameFeaturesToolset.GameFeaturesToolset.IsGameFeaturePlugin \
  --arguments '
{
  "pluginName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

True if the plugin is a Game Feature Plugin

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
