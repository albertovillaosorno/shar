# Add node pin

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.add_node_pin
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Adds a pin to a node that supports dynamic pin addition.

Works for Switch nodes (adds one case pin), Sequence nodes (adds one Then
output), commutative binary operators like Add/Multiply (adds one input), Make
Array nodes, etc.

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
Use this mutation to append one supported dynamic pin to a reviewed SHAR
Blueprint node, such as an additional Sequence execution output.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Discover and create a node type that supports dynamic pins.
- Capture the current pin inventory with `get_node_infos`.
- Confirm the operation is bounded to one exact node.
- Retain the returned PinID and define `remove_node_pin` as the inverse.
- Compile and inspect the node after the addition.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "node": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_PinLifecycle.BP_MCP_PinLifecycle:EventGraph.K2Node_ExecutionSequence_0"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The live-discovered Sequence node initially exposed `then_0` and `then_1`.
The call returned an `EGPD_Output` PinID at index `2` for the same node.
`get_node_infos` then reported `then_2` with index `2`, and the Blueprint
compiled with warnings treated as errors while the added output existed.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Only node classes with supported dynamic-pin behavior accept this mutation.
- The tool chooses the new pin semantics; no name or type argument is exposed.
- The returned PinID is session-sensitive and must be retained immediately.
- Verify the resulting pin name, direction, index, and node ownership.
- Remove disposable additions with `remove_node_pin` before deleting the node.
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
  editor_toolset.toolsets.blueprint.BlueprintTools.add_node_pin \
  --arguments '
{
  "node": {}
}
'
```

## Expected output

The PinID of the newly added pin.

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
