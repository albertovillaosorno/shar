# Set create event function

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.set_create_event_function
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Binds a function to a Create Event node.

Use list_compatible_event_functions to find valid function names.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Capture pre-state, bound the target set, and verify the resulting editor or
asset state through an independent read.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this mutation to assign or reassign the function implemented by one exact
SHAR Create Event node after its delegate signature has been fixed by a live
connection.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Discover and create `EventDispatchers|CreateEvent` in the exact graph.
- Connect its `OutputDelegate` pin to a compatible delegate input first.
- Create or identify functions whose signatures match that connected delegate.
- Read the current assignment with `get_create_event_function`.
- Define strict compilation, reassignment verification, and disposable-asset
  cleanup before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "node": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_CreateEventLifecycle.BP_MCP_CreateEventLifecycle:EventGraph.K2Node_CreateDelegate_1"
  },
  "function_name": "MCP_CreateEventTarget"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The Create Event node initially returned an empty function name. An assignment
attempt before connecting `OutputDelegate` was rejected and left that value
unchanged. The output was then connected to the `Delegate` input of
`Utilities|Time|SetTimerbyEvent`. Assigning `MCP_CreateEventTarget` returned
`null`; the exact getter returned that name and strict compilation passed.
Reassigning `MCP_CreateEventAlternate` produced the same verified result.
Passing an empty string was rejected, and the alternate assignment remained
unchanged. Reassigning the first valid function restored
`MCP_CreateEventTarget`, and strict compilation passed again. Deleting the
complete disposable validation folder removed all virtual and physical assets.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The Create Event output must be connected before Unreal can infer a delegate
  signature and validate function compatibility.
- The requested function must match that delegate signature.
- An empty string did not clear the assignment; it was rejected as incompatible.
- The tool has no structured return value; verify with
  `get_create_event_function`.
- A rejected assignment can enumerate compatible function names, but that error
  text is diagnostic rather than a stable discovery API.
- Node references and compatibility can change after graph or signature edits.
- The operation does not save the Blueprint automatically.
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
shar-unreal-mcp describe editor_toolset.toolsets.blueprint.BlueprintTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `function_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the function to bind.

### `node`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.set_create_event_function \
  --arguments '
{
  "function_name": "<value>",
  "node": {}
}
'
```

## Expected output

The live interface does not declare a structured output schema.

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
