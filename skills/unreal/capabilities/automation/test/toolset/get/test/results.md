# Get test results

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
AutomationTestToolset.AutomationTestToolset.GetTestResults
```

Toolset:

```text
AutomationTestToolset.AutomationTestToolset
```

## What this tool does

Get detailed results for the current or most recent test run. Requires
DiscoverTests() to have completed. Returns a JSON object with per-test state,
duration, errors, and warnings.

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
Use this tool to retrieve detailed current or most-recent Unreal automation
results after a bounded SHAR test invocation, or to prove that no result set is
currently available before starting a new controlled run.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- `DiscoverTests` must have completed successfully in the same editor session.
- For executed tests, retain the exact run invocation and wait for controller
  completion before treating results as final.
- Correlate returned test paths with the previously selected identities.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls before any run returned an empty result with zero total,
passed, failed, skipped, and duration values. `GetTestStatus` independently
reported `Ready` with zero completed tests, confirming that the empty result was
valid idle state rather than a transport failure.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON string and requires a second JSON parse.
- An empty zero-count result is valid when no current or previous run is
  available.
- The tool reports the current or most recent run; preserve run-selection
  evidence so stale results cannot be attributed to a later invocation.
- Detailed results are not final while the controller is still running.
- This read does not start, stop, retry, or clear automation tests.
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

This tool accepts no input fields.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  AutomationTestToolset.AutomationTestToolset \
  AutomationTestToolset.AutomationTestToolset.GetTestResults \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
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
