# Get game feature state

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GameFeaturesToolset.GameFeaturesToolset.GetGameFeatureState
```

Toolset:

```text
GameFeaturesToolset.GameFeaturesToolset
```

## What this tool does

Gets the current state of a Game Feature Plugin.

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
Use this tool to read the current lifecycle state of one exact discovered Game
Feature plugin.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require doctor readiness and refresh the live Game Features schema.
- Use a uniquely named content-only Game Feature with a matching root
  GameFeatureData asset.
- Confirm discovery, enabled status, feature identity, state, and active
  status before mutation.
- Define deactivation and complete plugin/content cleanup before activation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "pluginName": "MCPRoundNextFeature260718"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The disposable content-only Game Feature was independently listed as
discovered, enabled, and valid. The initial state reader returned `Unknown`
while the active reader returned false.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Lifecycle requests are asynchronous; poll both state and active status
  instead of trusting the Boolean alone.
- A newly discovered feature may report `Unknown`; successful deactivation of
  the tested feature settled at `Loaded`, not unloaded.
- Deactivate the feature before deleting its GameFeatureData asset or plugin
  directory.
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

### `pluginName`

- Required: **yes**
- Type: `string`
- Purpose:

Name of the Game Feature Plugin.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GameFeaturesToolset.GameFeaturesToolset \
  GameFeaturesToolset.GameFeaturesToolset.GetGameFeatureState \
  --arguments '
{
  "pluginName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Allowed values:

  - `"Uninitialized"`
  - `"Installed"`
  - `"Registered"`
  - `"Loaded"`
  - `"Active"`
  - `"Unknown"`
- Purpose:

Simplified state enum. Raises an error if the subsystem is unavailable or the
plugin is not found.

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
