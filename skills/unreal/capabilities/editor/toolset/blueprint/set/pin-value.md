# Set pin value

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.set_pin_value
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Sets the value of a Blueprint graph pin.

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
Use this mutation to replace the serialized default value of one exact SHAR
Blueprint input data pin after its current PinID and value have been read.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain the current PinID from `get_node_infos`.
- Confirm the pin is an input data pin that supports a default value.
- Read and retain the current serialized value with `get_pin_value`.
- Define the exact replacement string, strict compile check, and inverse value
  before mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "pin": {
    "direction": "EGPD_Input",
    "index_id": 1,
    "node": {
      "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_PinLifecycle.BP_MCP_PinLifecycle:EventGraph.K2Node_IfThenElse_0"
    }
  },
  "value": "false"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A live Branch `Condition` input initially returned the serialized string
`true`. The mutation returned `null`; `get_pin_value` then returned `false`, and
the Blueprint compiled with warnings treated as errors. A second mutation
restored `true`, a fresh read returned `true`, and the restored graph compiled
strictly again.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Values are passed as serialized strings, not typed JSON primitives.
- The tool has no structured return value; verify with `get_pin_value`.
- Execution pins and output pins do not provide meaningful editable defaults.
- PinIDs can become stale after graph edits or recompilation.
- Use the retained pre-state value as the exact inverse.
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

### `pin`

- Required: **yes**
- Type: `object`
- Purpose:

PinID

### `value`

- Required: **yes**
- Type: `string`
- Purpose:

The value to assign as a string. Format depends on the pin's data type:;
Numeric pins (int/float): "42" or "3.14"; Boolean pins: "true" or "false";
String/Name/Text pins: "Hello World"; Object reference pins:
"/Game/Path/To/Asset.Asset"

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.set_pin_value \
  --arguments '
{
  "pin": {},
  "value": "<value>"
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
