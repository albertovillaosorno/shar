# Find actors

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Expected read-only**
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
Use this tool to resolve a narrowly named actor before SHAR scene validation or
actor-property inspection. The verified example located the template
`PlayerStart` before reading its editor label and world transform.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and an active Level Editor world must be ready.
- Bound the search with a meaningful name, tag, type, root, or bounds criterion.
- The live schema requires `name`, `tag`, and `collision_channels`, even when an
  unused criterion is represented by an empty string or array.
- Consume returned actor references immediately through a compatible actor read.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "collision_channels": [],
  "name": "PlayerStart",
  "tag": ""
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The validated call returned exactly one actor reference under the current
`/Temp/Untitled_1` level. Independent `get_label` and `get_actor_transform`
reads returned the `PlayerStart` label, location near `(-200, 0, 92)`, yaw
`180`, and unit scale.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Supplying empty `name`, `tag`, and `collision_channels` values with no other
  filter produced a very large world-wide actor result in the verified level.
  Do not use that shape for routine discovery.
- Returned references can contain transient `/Temp/...` level paths and Unreal
  actor instance identifiers. Re-discover them after a level reload instead of
  persisting them as project constants.
- An empty collision-channel array is still required by the current live schema
  when collision filtering is not used.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

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
