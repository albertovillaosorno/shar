# Add struct function param

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.add_struct_function_param
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Adds a struct input or output to a function or event dispatcher.

Note: output params are not supported on event dispatchers.

Use this to add parameters of any UStruct type, including custom structs and
engine structs not in the basic list supported by add_function_param (e.g.
HitResult, GameplayTag).

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
Use this tool to add one exact script-struct input or output to a reviewed SHAR
Blueprint function graph.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Supply an exact function or dispatcher graph reference.
- Resolve the exact `ScriptStruct` and confirm the parameter name is absent.
- Set `input_param` explicitly; output parameters are invalid on dispatchers.
- Choose an optional container type exactly from the live schema.
- Define parameter removal and compile verification before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_FunctionLifecycle.BP_MCP_FunctionLifecycle:ValidatePayload"
  },
  "param_name": "Hit",
  "struct_type": {
    "refPath": "/Script/Engine.HitResult"
  },
  "input_param": false
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Adding `Hit` with exact struct `/Script/Engine.HitResult` returned result-node
PinID index `2`, direction `EGPD_Input`. The result-node read independently
reported `Hit` after output `Success`. Strict compilation succeeded. Removing
`Hit` returned `null`; after all removals, the function DSL contained an empty
input signature and the graph still compiled before exact graph deletion.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Function outputs appear as input pins on the result node.
- The struct identity must resolve to an exact `ScriptStruct`.
- Node-info and graph DSL did not independently surface the struct type in the
  verified session; retain the validated call evidence.
- Output parameters are unsupported on event dispatchers.
- Custom structs, containers, and schema evolution need separate verification.
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

### `struct_type`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.add_struct_function_param \
  --arguments '
{
  "graph": {},
  "input_param": false,
  "param_name": "<value>",
  "struct_type": {}
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
