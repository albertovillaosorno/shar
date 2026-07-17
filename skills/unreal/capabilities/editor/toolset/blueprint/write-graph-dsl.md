# Write graph dsl

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.write_graph_dsl
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Populates a Blueprint graph with nodes from a DSL script and compiles the
Blueprint.

Call get_graph_dsl_docs() for the full syntax reference and examples.

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
Use this mutation to author a bounded SHAR Blueprint function or event graph
from the live S-expression grammar when node-by-node construction would be less
clear and the complete target graph is disposable or separately recoverable.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Load the current grammar with `get_graph_dsl_docs`; do not rely on remembered
  syntax.
- Resolve one exact graph and capture its current DSL with `read_graph_dsl`.
- Prefer a uniquely named disposable function graph for initial validation.
- Confirm every referenced event or node type is available in that graph
  context.
- Define strict compilation, independent DSL read-back, and whole-asset cleanup
  before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "code": "(fn MCP_DSL_Lifecycle ()
  (Development|PrintString "MCP DSL lifecycle"))",
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_DSL_Lifecycle.BP_MCP_DSL_Lifecycle:MCP_DSL_Lifecycle"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A disposable function graph initially serialized as
`(fn MCP_DSL_Lifecycle ())`. Writing the validated function returned `null`.
`read_graph_dsl` then returned the same function body with the
`Development|PrintString` statement and `MCP DSL lifecycle` literal, plus the
normal trailing newline. The Blueprint compiled with warnings treated as
errors. Writing the original empty function form also returned `null`, but a
fresh read showed the authored statement was still present and strict
compilation still succeeded. Deleting the complete disposable validation folder
restored virtual and physical asset absence.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The writer is not declarative replacement: omitting an existing statement from
  a later script did not remove that statement.
- A captured `read_graph_dsl` result is therefore not a guaranteed rollback
  program.
- An arbitrary custom event named `MCP_DSL_Lifecycle` was rejected because the
  corresponding `AddEvent` identity did not exist; the graph remained
  byte-identical after that rejected call.
- Use event names and node type IDs supported by the exact graph context.
- The tool has no structured return value; verify with `read_graph_dsl` and
  strict compilation.
- Persistent graph cleanup requires separately reviewed node or graph removal;
  this validation used whole disposable-asset deletion.
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

### `code`

- Required: **yes**
- Type: `string`
- Purpose:

The S-expression DSL script to convert into Blueprint nodes.

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.write_graph_dsl \
  --arguments '
{
  "code": "<value>",
  "graph": {}
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
