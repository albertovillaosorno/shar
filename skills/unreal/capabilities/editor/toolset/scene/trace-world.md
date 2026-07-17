# Trace world

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.trace_world
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Traces a line through the world and returns the distance to the first hit.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to measure one bounded SHAR world-space line against current
scene collision before spawn, camera, placement, ground-clearance, or
line-of-sight decisions.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR editor world must be loaded and stable.
- Derive start and end coordinates from current project evidence.
- Confirm the segment length and direction are bounded for the requested check.
- Treat the result as current world collision state; rerun after geometry,
  streaming, collision, or world changes.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "start": {
    "x": 0,
    "y": 0,
    "z": 100000
  },
  "end": {
    "x": 0,
    "y": 0,
    "z": -100000
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two identical vertical traces returned `returnValue: 100000`, proving a
stable first hit at world-space `z: 0` along the 200,000-unit segment. Two
identical horizontal traces from `(0, 0, 100000)` to `(1000, 0, 100000)`
returned `null`, reproducing the no-hit branch. No actor, component, asset, or
editor state changed.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool returns only the distance from the start point, not the hit actor,
  component, location, normal, material, or collision channel.
- `null` is a successful no-hit result.
- The trace schema exposes no collision-channel, ignore-list, shape, or complex-
  collision selection.
- A hit at distance zero can mean the start lies inside collision; choose start
  points deliberately.
- Results depend on current loaded and collision-enabled world state.
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `end`

- Required: **yes**
- Type: `object`
- Purpose:

Vector

### `start`

- Required: **yes**
- Type: `object`
- Purpose:

Vector

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.trace_world \
  --arguments '
{
  "end": {},
  "start": {}
}
'
```

## Expected output

The distance from start to the hit point, or None if nothing was hit.

### `returnValue`

- Required: **no**
- Type: `number`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
