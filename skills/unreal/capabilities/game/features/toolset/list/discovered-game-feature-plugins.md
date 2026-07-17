# List discovered game feature plugins

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GameFeaturesToolset.GameFeaturesToolset.ListDiscoveredGameFeaturePlugins
```

Toolset:

```text
GameFeaturesToolset.GameFeaturesToolset
```

## What this tool does

Lists all discovered Game Feature Plugins sorted by name. This includes enabled
and disabled plugins. Only enabled plugins are known by the Game Features
system beyond identifying if a plugin is a Game Feature Plugin. Use the Plugins
toolset to do general plugin enable/disable tasks.

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
Use this tool as the first Game Features preflight before SHAR inspects feature
state or considers activation. The complete discovered inventory determines
whether later feature-specific work has a real plugin identity or must stop.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- The Plugin Manager and Game Features subsystem must have completed startup
  discovery.
- Use the returned names exactly in later Game Features calls.
- Do not substitute the general plugin inventory for this GFP-specific list.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned the same empty array. Independent general plugin
reads reported 893 discovered plugins, including `Niagara`, proving that the
empty GFP inventory was not a Plugin Manager failure.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- An empty array is valid and means no Game Feature Plugin is currently
  discovered in this editor session.
- General Unreal plugins are intentionally excluded even when they are enabled
  and own content mounts.
- Discovery can change after project configuration, plugin installation, or an
  editor restart; repeat this preflight in the same session as dependent reads.
- Without a returned identity, state, activation, and deactivation tools cannot
  be validated truthfully.
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
  GameFeaturesToolset.GameFeaturesToolset.ListDiscoveredGameFeaturePlugins \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Sorted names of all discovered Game Feature Plugins.

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
