# Remove function graph

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.remove_function_graph
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Removes a function graph or event dispatcher from the Blueprint.

Note: This will close the Blueprint editor window if it is currently open.

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
Use this tool to remove one exact disposable or explicitly approved SHAR
function graph or event dispatcher from a Blueprint.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Confirm the exact implemented function or dispatcher with `list_functions`
  and `list_graphs`.
- Capture its full signature, graph DSL, local variables, nodes, and call sites.
- Confirm it is disposable or has an approved reconstruction path.
- Close or avoid opening the Blueprint editor because removal can close it.
- Define graph absence, function absence, failed lookup, and compile checks.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "blueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_FunctionLifecycle.BP_MCP_FunctionLifecycle"
  },
  "graph_name": "ValidatePayload"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Before removal, graph and function reads both identified the implemented
`ValidatePayload` function. The call returned `null`. `list_graphs` then
returned only construction and event graphs, and `list_functions` no longer
contained `ValidatePayload`. Exact `get_graph` lookup failed with a not-found
error, strict compilation succeeded, and deleting the disposable Blueprint left
no registry, folder, or filesystem residue.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Success returns `null`; require graph, function, and lookup absence checks.
- Removal is destructive and can break call sites, interfaces, or overrides.
- The tool can close the Blueprint editor window when the asset is open.
- The validated disposable Blueprint was not open in an asset editor.
- Capture the complete graph because this tool returns no reconstruction data.
- Saving is separate; unsaved removal can still affect the loaded editor state.
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

### `blueprint`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `graph_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the function or dispatcher to remove.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.remove_function_graph \
  --arguments '
{
  "blueprint": {},
  "graph_name": "<value>"
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
