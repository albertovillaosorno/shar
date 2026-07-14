# List enabled game feature plugins

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GameFeaturesToolset.GameFeaturesToolset.ListEnabledGameFeaturePlugins
```

Toolset:

```text
GameFeaturesToolset.GameFeaturesToolset
```

## What this tool does

Lists all enabled Game Feature Plugins  sorted by name. Enabled plugins are the
only plugins known by the Game Features system beyond identifying if a plugin
is a Game Feature Plugin. Use the Plugins toolset to do general plugin
enable/disable tasks.

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
Use this tool to enumerate enabled Game Feature Plugins before SHAR checks
runtime state or plans a feature transition. Compare it with the discovered GFP
inventory to distinguish an empty project from a disabled feature set.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- The Plugin Manager and Game Features subsystem must have completed startup.
- Run `ListDiscoveredGameFeaturePlugins` in the same editor session for the
  complete GFP comparison set.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned the same empty array. The discovered GFP
inventory was also empty. Independent general plugin reads reported 276 enabled
plugins, including `Niagara`, confirming that this result is specific to Game
Feature Plugins rather than all enabled plugins.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty array is valid and means no enabled Game Feature Plugin is known to
  the Game Features subsystem.
- This is not the general enabled-plugin inventory; use `PluginToolset` for that
  separate question.
- Enabled does not necessarily mean `Active`; state inspection requires a real
  GFP identity from the discovered inventory.
- Results can change after plugin configuration or editor restart.
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

This tool accepts no input fields.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GameFeaturesToolset.GameFeaturesToolset \
  GameFeaturesToolset.GameFeaturesToolset.ListEnabledGameFeaturePlugins \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Sorted names of all enabled Game Feature Plugins.

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
