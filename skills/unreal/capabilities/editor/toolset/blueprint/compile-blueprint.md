# Compile blueprint

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Execution or transient mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.compile_blueprint
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Compiles the given Blueprint.

Blueprints should be compiled after all graph modifications are complete.

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
Use this tool after bounded SHAR Blueprint graph, variable, component, parent,
or metadata changes to prove that the resulting Blueprint remains compilable.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply an exact Blueprint object reference from current editor state.
- Complete all intended graph and metadata mutations before compiling.
- Decide whether warnings must fail the operation and set `warnings_as_errors`
  explicitly.
- Define independent generated-class, spawn, log, or other postcompile checks.
- Determine separately whether the compiled Blueprint must be saved.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "blueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_Validation.BP_MCP_Validation"
  },
  "warnings_as_errors": true
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call completed with `returnValue: null` and no error while
`warnings_as_errors` was enabled. Asset class inspection returned the generated
`BP_MCP_Validation_C` class. An independent asset-backed scene spawn produced a
valid actor with the requested label and transform, proving that the compiled
class was usable. Compilation alone created no content directory or `.uasset`
file. The spawned actor, Blueprint asset, and virtual folder were then removed
without residue.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A successful compile returns `null`; it does not return diagnostics or a
  generated-class reference.
- With `warnings_as_errors: true`, warnings or errors can reject the call.
- Compilation does not prove gameplay semantics, graph reachability, runtime
  initialization, serialization, or save success.
- Compilation did not save the unsaved Blueprint in the verified case.
- Inspect editor logs or perform targeted graph and spawn checks when the task
  requires stronger evidence than compile completion.
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

### `warnings_as_errors`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If True, raise on compile warnings as well as errors. Defaults to False.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.compile_blueprint \
  --arguments '
{
  "blueprint": {}
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
