# Remove function param

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.remove_function_param
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Removes an input or output from a function or event dispatcher.

Note: output params are not supported on event dispatchers.

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
Use this tool to remove one exact input or output parameter from a disposable or
explicitly approved SHAR function or dispatcher signature.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply the current exact function or dispatcher graph reference.
- Capture the parameter name, caller-facing direction, type, container, and all
  graph or call-site references.
- Set `input_param` to the original caller-facing direction.
- Confirm the parameter is disposable or has an approved reconstruction path.
- Define node, DSL, and compile checks for the resulting signature.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_FunctionLifecycle.BP_MCP_FunctionLifecycle:ValidatePayload"
  },
  "param_name": "Hit",
  "input_param": false
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The four parameters were removed in reverse order using their original
caller-facing directions. Every removal returned `null`. After removal, strict
compilation succeeded and graph DSL changed from `(fn ValidatePayload (Count
Mesh))` to `(fn ValidatePayload ())`. Entry and result node reads from the
populated state had previously proved the exact parameter names, positions, and
directions.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Success returns `null`; verify the resulting signature independently.
- `input_param` describes the caller-facing direction, not the entry or result
  node pin direction.
- Graph DSL omitted output parameters in the verified session, so result-node
  reads are required before destructive removal.
- Removal can break function call nodes, overrides, or interface contracts.
- Output removal is unsupported for event dispatchers.
- The operation is destructive in unsaved state and needs a separate save
  decision.
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

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `input_param`

- Required: **yes**
- Type: `boolean`
- Purpose:

True to remove an input param, False to remove an output.

### `param_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the param to remove.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.remove_function_param \
  --arguments '
{
  "graph": {},
  "input_param": false,
  "param_name": "<value>"
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
