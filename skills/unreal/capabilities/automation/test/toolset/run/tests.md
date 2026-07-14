# Run tests

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Execution or transient mutation likely**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
AutomationTestToolset.AutomationTestToolset.RunTests
```

Toolset:

```text
AutomationTestToolset.AutomationTestToolset
```

## What this tool does

Run a set of automation tests by name. Requires DiscoverTests() to have
completed. Starts executing the specified tests and returns an async result
that completes with a JSON summary when all tests finish.

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
Use this tool to execute a small, explicitly reviewed set of Unreal automation
tests by their exact full paths after SHAR has discovered and listed them. Prefer
this capability when the intended identities are already known and the target
set is easier to audit as a list than as a filter.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- `DiscoverTests` must have completed successfully in the same editor session.
- `ListTests` must confirm every exact full path immediately before execution.
- `GetTestStatus` must report an idle `Ready` controller.
- Bound the list to reviewed tests whose editor and filesystem side effects are
  understood, and choose a timeout suitable for the slowest selected test.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "testNames": [
    "AI.ModelContextProtocol.Analytics.HashToolIdentifier.should be deterministic for the same input"
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The validated call executed exactly one test and returned `total: 1`,
`passed: 1`, and `failed: 0`. The requested analytics hash test entered
`Success` state with no errors or warnings. `GetTestStatus` and
`GetTestResults` independently confirmed the same single completion.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON string and requires a second JSON parse.
- The operation is asynchronous and completes only when the selected tests
  finish; a client timeout does not by itself prove native execution stopped.
- An empty `testNames` array raises an error.
- In the verified session, an unknown nonempty test name returned an empty
  zero-count success instead of an error and replaced the detailed result set.
  Prevalidate every identity with `ListTests` and require returned membership to
  match the requested set.
- Tests can mutate editor, asset, filesystem, or process state according to
  their own contracts; exact selection does not make an unsafe test safe.
- Controller aggregate fields can differ by selection method, so the returned
  per-test summary is the primary completion evidence.
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `testNames`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Array of full test paths as returned by ListTests.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  AutomationTestToolset.AutomationTestToolset \
  AutomationTestToolset.AutomationTestToolset.RunTests \
  --arguments '
{
  "testNames": []
}
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
- Verify changed state through a separate read or inspection.
- Use another capability to confirm the postcondition.
- Inspect editor logs when state is not directly observable.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
