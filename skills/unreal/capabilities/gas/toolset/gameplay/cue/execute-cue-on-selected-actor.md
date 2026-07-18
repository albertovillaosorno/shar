# Execute cue on selected actor

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GASToolsets.GameplayCueToolset.ExecuteCueOnSelectedActor
```

Toolset:

```text
GASToolsets.GameplayCueToolset
```

## What this tool does

Executes a gameplay cue non-replicated on the currently selected actor in the
editor. Useful for previewing cue effects without network replication. Requires
a PIE session or a configured GameplayCueManager to produce visible results.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Confirm execution scope, cancellation behavior, and expected side effects
before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to dispatch one disposable Gameplay Cue against the exact
selected validation actor.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a disposable selected actor, a unique temporary Gameplay Cue tag, and an
  actor-based cue-notify asset.
- Capture project Gameplay Tag config before mutation and require exact
  restoration after removing the tag.
- Use scene actor discovery for the generated notify class as the independent
  execution postcondition.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "cueTag": "GameplayCue.MCP.Round50",
  "location": {
    "x": 3600,
    "y": 400,
    "z": 150
  },
  "normal": {
    "x": 0,
    "y": 0,
    "z": 1
  },
  "normalizedMagnitude": 0.75
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The selected actor was read back exactly. Execution returned true, and scene
discovery changed from zero cue-notify actors to one `GCN_MCP_Round50_C`
actor.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Execution depends on active editor actor selection and persistent Gameplay
  Tag state.
- The notify actor is a world side effect and must be removed before deleting
  its Blueprint asset.
- Cue-notify registry readers lagged immediately after asset creation even
  though execution spawned the actor. Verify the world side effect directly.
- Gameplay-tag config is persistent project state; remove the disposable tag
  and restore its config byte-for-byte.
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
shar-unreal-mcp describe GASToolsets.GameplayCueToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `cueTag`

- Required: **yes**
- Type: `string`
- Purpose:

The fully-qualified tag of the cue to execute, e.g.
"GameplayCue.Character.Death".

### `location`

- Required: **yes**
- Type: `object`
- Purpose:

World-space location parameter passed to the cue.

### `normal`

- Required: **yes**
- Type: `object`
- Purpose:

World-space direction parameter passed to the cue.

### `normalizedMagnitude`

- Required: **yes**
- Type: `number`
- Purpose:

A normalized (0.0-1.0) magnitude value passed to the cue.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GASToolsets.GameplayCueToolset \
  GASToolsets.GameplayCueToolset.ExecuteCueOnSelectedActor \
  --arguments '
{
  "cueTag": "<value>",
  "location": {},
  "normal": {},
  "normalizedMagnitude": 0.0
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

True if the cue was dispatched. Raises a script error if no actor is selected
or the tag does not exist.

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
