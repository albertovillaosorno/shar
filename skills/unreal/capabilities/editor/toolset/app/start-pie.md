# Start pie

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
EditorToolset.EditorAppToolset.StartPIE
```

Toolset:

```text
EditorToolset.EditorAppToolset
```

## What this tool does

Starts a Play-In-Editor or Simulate-In-Editor session using the current level.
Completes after the engine fires PostPIEStarted (session fully started,
BeginPlay called) and Options.WarmupSeconds have elapsed, giving project-
specific initialization (services, authentication, plugin warmup) time to
settle before the agent inspects state or logs. Raises an error if a play
session is already running.

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
Use this tool to start one bounded SHAR simulation session before checking
runtime initialization, autonomous world behavior, logs, or other PIE-only
state, with `StopPIE` defined as mandatory cleanup.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must have the intended current level loaded.
- `IsPIERunning` must return `false` before the call.
- Choose standard PIE or simulation deliberately and keep the play mode
  in-process for deterministic MCP completion tracking.
- Define the matching `StopPIE` call in a `finally` recovery path.
- Choose `warmupSeconds` from the initialization evidence required by the task.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "options": {
    "bSimulate": true,
    "playMode": "PlayMode_Simulate",
    "warmupSeconds": 0
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned `returnValue: null`. A separate `IsPIERunning` call
returned `true`, proving that the simulation session had started. `StopPIE`
then returned `returnValue: null`, and a final independent state read returned
`false`. A second reproduced cycle produced the same lifecycle results.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `bSimulate: true` runs world ticking, AI, physics, and subsystems without
  spawning or possessing a player pawn.
- `warmupSeconds: 0` completes after native PIE startup but adds no
  project-specific settling time.
- A second start while PIE or simulation is active is rejected with
  `A play session is already running.`
- The session can change transient runtime state and logs; always stop the
  session owned by the current operation before further editor mutations.
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
shar-unreal-mcp describe EditorToolset.EditorAppToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `options`

- Required: **yes**
- Type: `object`
- Purpose:

Session configuration: PIE vs Simulate, play mode, optional spawn transform
override, warmup duration. See FPIESessionOptions.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.StartPIE \
  --arguments '
{
  "options": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `null`
- Purpose:

Always null

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
