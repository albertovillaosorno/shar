# Find actors

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.find_actors
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Searches the scene for actors that match specific criteria.

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
Use this tool to perform bounded actor discovery by label, class, tag, hierarchy
root, world bounds, and collision channels.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass explicit empty strings for unused `name` and `tag` fields.
- Pass an explicit collision-channel array.
- Use `bounds: null` when no spatial filter is intended.
- Rediscover session-local actor refs after level changes.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "name": "Brush",
  "tag": "",
  "actor_type": null,
  "root": null,
  "bounds": null,
  "collision_channels": []
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two unfiltered calls returned the same 145 actors. `Brush` and lowercase `brush`
each returned only `Brush_0`; missing name and tag filters returned empty
arrays. Rooting the query at `Brush_0` returned that actor. A huge bounds box
returned all 145 actors without channels and 64 with `ObjectTypeQuery1`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Name matching is case-insensitive substring matching.
- Collision channels affect the native overlap query only with bounds.
- `bounds: null` is not equivalent to a box with `isValid: false`; the latter
  returned a reduced 23-actor result.
- A non-Actor class can fail inside `GameplayStatics.GetAllActorsOfClass`.
- Results are current-session actor refs and can change after streaming or level
  edits.
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
shar-unreal-mcp describe editor_toolset.toolsets.scene.SceneTools
```

1. Confirm every required input against the current schema.

## Inputs

### `actor_type`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `bounds`

- Required: **no**
- Type: `object`
- Purpose:

Box

### `collision_channels`

- Required: **yes**
- Type: `array<string>`
- Purpose:

If set, bounds checks will uses a native physics overlap query restricted to
these channels. Non-collision actors will be ignored.

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

If set, will only return actors whose label contains this string (case-
insensitive).

### `root`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `tag`

- Required: **yes**
- Type: `string`
- Purpose:

If set, will only return actors that have this tag.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.find_actors \
  --arguments '
{
  "collision_channels": [],
  "name": "<value>",
  "tag": "<value>"
}
'
```

## Expected output

A list of actors that match the criteria.

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
