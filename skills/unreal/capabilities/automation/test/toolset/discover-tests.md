# Discover tests

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
AutomationTestToolset.AutomationTestToolset.DiscoverTests
```

Toolset:

```text
AutomationTestToolset.AutomationTestToolset
```

## What this tool does

Initialize automation worker discovery and load the test list. Must be called
once before ListTests or RunTests. Takes several seconds as it discovers the
local automation worker and enumerates all registered tests. Returns an async
result that completes with a JSON status object when tests are available, or an
error if discovery fails.

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
Use this tool to initialize Unreal's local automation controller before SHAR
lists, selects, runs, or inspects editor automation tests. Treat successful
discovery as a session prerequisite, not as evidence that any test executed.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project and native MCP server must be ready.
- The editor must be idle enough to discover the local automation worker.
- Use the default cached discovery first; force rediscovery only after test
  modules, plugins, or registrations changed in the same editor session.
- Allow a longer timeout than ordinary metadata reads.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "bForceRediscover": false
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive default calls returned `{ "status": "ready" }`. A separate
call with `bForceRediscover: true` also returned `ready`. A following bounded
`ListTests` query returned 20 of 21 MCP-related tests, proving that the report
tree was available without starting a test run.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON string and requires a second JSON parse.
- Discovery initializes and refreshes controller state but does not execute any
  test.
- The operation is asynchronous and can take longer than ordinary read tools;
  use an appropriate timeout and do not assume a client timeout cancelled native
  discovery.
- Forced rediscovery bypasses the cached report tree and should be reserved for
  a known registration change.
- A `ready` result does not prove that a particular test exists; verify it with
  `ListTests`.
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
shar-unreal-mcp describe AutomationTestToolset.AutomationTestToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `bForceRediscover`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

When true, bypass the cached report tree and re-poll workers. Used after
reloading Python test modules mid-session.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  AutomationTestToolset.AutomationTestToolset \
  AutomationTestToolset.AutomationTestToolset.DiscoverTests \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

Value of the result.

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
