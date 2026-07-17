# Add function param

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.add_function_param
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Adds an input or output to a function or event dispatcher

Note: output params are not supported on event dispatchers.

Supported parameter types: Primitives: 'bool', 'int', 'float', 'byte',
'string', 'name', 'text' Structs:    'Vector', 'Rotator', 'Transform',
'Vector2D', 'LinearColor'

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
Use this tool to add one primitive or built-in struct input or output to a
reviewed SHAR Blueprint function graph.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply an exact function or dispatcher graph reference.
- Confirm the parameter name is absent from the intended signature.
- Choose a supported type string and optional container type exactly.
- Set `input_param` explicitly; output parameters are invalid on dispatchers.
- Define parameter removal and compile verification before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_FunctionLifecycle.BP_MCP_FunctionLifecycle:ValidatePayload"
  },
  "param_name": "Count",
  "param_type": "int",
  "input_param": true
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Adding input `Count` returned entry-node PinID index `1`, direction
`EGPD_Output`; adding output `Success` returned result-node PinID index `1`,
direction `EGPD_Input`. Node-info reads reported `Count` on the function entry
and `Success` on the return node. The graph DSL serialized the input signature
as `(fn ValidatePayload (Count Mesh))`. Strict compilation succeeded before and
after all parameters were removed.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Function inputs appear as output pins on the entry node; function outputs
  appear as input pins on the result node.
- Pin direction therefore describes graph flow, not caller-facing direction.
- The verified node-info read exposed names, indices, and directions but not the
  reflected type metadata; the exact type came from the validated call.
- Graph DSL included input names but omitted output names and all types.
- Container and dispatcher behavior need separate verification.
- The operation changes unsaved state and needs separate compile and save
  decisions.
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

### `container_type`

- Required: **no**
- Type: `string`
- Allowed values:

  - `"ARRAY"`
  - `"SET"`
  - `"MAP"`
- Purpose:

ContainerType

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `input_param`

- Required: **yes**
- Type: `boolean`
- Purpose:

True to add an input param, false to add an output.

### `param_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the new parameter.

### `param_type`

- Required: **yes**
- Type: `string`
- Purpose:

The parameter type as a string (see above).

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.add_function_param \
  --arguments '
{
  "graph": {},
  "input_param": false,
  "param_name": "<value>",
  "param_type": "<value>"
}
'
```

## Expected output

The PinID of the newly created parameter.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

PinID

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
