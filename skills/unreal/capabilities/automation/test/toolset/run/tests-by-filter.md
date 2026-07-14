# Run tests by filter

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Execution or transient mutation likely**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
AutomationTestToolset.AutomationTestToolset.RunTestsByFilter
```

Toolset:

```text
AutomationTestToolset.AutomationTestToolset
```

## What this tool does

Run automation tests selected by a filter expression. Requires DiscoverTests()
to have completed. Much faster than RunTests when targeting a large batch
because the engine narrows the report tree in a single pass instead of running
a per-leaf membership check against the requested name list.

Filter syntax (multiple expressions joined by '+'): "StartsWith:System.Engine"
prefix match against the full test path "^Foo"                       prefix
anchor (equivalent to StartsWith:) "Bar$"                       suffix anchor
"Substring"                  bare token matches anywhere in the path
"Group:Smoke"                expand a named group from
AutomationControllerSettings ini Groups

Returns an async result that completes with the same JSON summary as RunTests.

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
Use this tool to execute a bounded Unreal automation test set when one reviewed
filter expresses the target more safely and efficiently than enumerating many
full paths. For a single test, anchor both ends of the exact path.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- `DiscoverTests` must have completed successfully in the same editor session.
- Preflight the proposed expression with `ListTests` or equivalent bounded
  discovery and record the expected identities and count.
- `GetTestStatus` must report an idle `Ready` controller.
- Review the side effects and timeout needs of every matched test before
  execution.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "filterExpression": "^AI.ModelContextProtocol.Analytics.HashToolIdentifier.should be deterministic for the same input$"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The exact anchored filter executed one test and returned `total: 1`,
`passed: 1`, and `failed: 0`. The only matched analytics hash test entered
`Success` state with no errors or warnings. `GetTestStatus` and
`GetTestResults` independently confirmed that single run.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON string and requires a second JSON parse.
- An unmatched filter raises a native no-tests-matched error.
- Prefix, suffix, bare-substring, combined, and named-group expressions can
  select more tests than expected; preflight exact membership and count before
  running.
- The operation is asynchronous; a client timeout does not prove the native run
  was cancelled.
- Tests can mutate editor, asset, filesystem, or process state according to
  their own contracts.
- The returned per-test summary, not transport completion or aggregate status
  alone, is the execution evidence.
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

### `filterExpression`

- Required: **yes**
- Type: `string`
- Purpose:

Filter expression as described above.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  AutomationTestToolset.AutomationTestToolset \
  AutomationTestToolset.AutomationTestToolset.RunTestsByFilter \
  --arguments '
{
  "filterExpression": "<value>"
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
