# Connect pins

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.connect_pins
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Makes a connection between source (output) and dest (input) pins.

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
Use this mutation to connect one compatible SHAR Blueprint output pin to one
input pin after both PinIDs have been read from live node metadata.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain both node references through the current graph session.
- Read exact pin directions and indices with `get_node_infos`.
- Confirm source and destination types are compatible.
- Capture both connection lists before mutation and define `break_pins` as the
  exact inverse.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "input_pin": {
    "direction": "EGPD_Input",
    "index_id": 0,
    "node": {
      "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_NodeLifecycle.BP_MCP_NodeLifecycle:EventGraph.K2Node_IfThenElse_1"
    }
  },
  "output_pin": {
    "direction": "EGPD_Output",
    "index_id": 0,
    "node": {
      "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_NodeLifecycle.BP_MCP_NodeLifecycle:EventGraph.K2Node_IfThenElse_0"
    }
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The first Branch `then` output and second Branch `execute` input were both
unconnected before mutation. The call returned `null`. Fresh node metadata then
reported one reciprocal connection on each pin, with the expected opposite node,
direction, and pin index.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- PinIDs are session-sensitive and combine node reference, direction, and index.
- The tool has no structured return value.
- Type compatibility must be established before invocation.
- Verify both ends of the connection; one transport success is insufficient.
- Use `break_pins` with the same PinIDs as the bounded inverse.
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

### `input_pin`

- Required: **yes**
- Type: `object`
- Purpose:

PinID

### `output_pin`

- Required: **yes**
- Type: `object`
- Purpose:

PinID

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.connect_pins \
  --arguments '
{
  "input_pin": {},
  "output_pin": {}
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
