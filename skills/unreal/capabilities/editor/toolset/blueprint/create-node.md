# Create node

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.create_node
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Adds a new node to the graph.

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
Use this mutation to add one exact node type to a reviewed SHAR Blueprint graph
when its native type ID has been discovered in the same live graph context.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact Blueprint and graph references in the current session.
- Discover the node type with `find_node_types`; do not invent type IDs.
- Capture the graph node inventory and choose an explicit initial position.
- Define strict compilation, node inspection, deletion, and asset cleanup before
  creating a disposable node.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_NodeLifecycle.BP_MCP_NodeLifecycle:EventGraph"
  },
  "pos": {
    "x": 160,
    "y": 240
  },
  "type_id": "Utilities|FlowControl|Branch"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`find_node_types` returned `Utilities|FlowControl|Branch` in the disposable
actor Blueprint EventGraph. Two create calls returned distinct
`K2Node_IfThenElse` references. `get_node_infos` reported the requested
`(160, 240)` position, the Branch type ID, `execute` and `Condition` inputs,
`then` and `else` outputs, and the default Boolean value `true`. The Blueprint
compiled with warnings treated as errors.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Node type IDs are graph-context-sensitive and must come from live discovery.
- The returned node reference is a session-sensitive nested object path.
- `declaring_class` is optional and was omitted for the native Branch node.
- Creating a node does not connect it, arrange it, compile the Blueprint, or
  save the asset automatically.
- Use `get_node_infos` rather than the transport result to verify type, pins,
  and position.
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

### `declaring_class`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `pos`

- Required: **yes**
- Type: `object`
- Purpose:

IntPoint

### `type_id`

- Required: **yes**
- Type: `string`
- Purpose:

The type of the node to add. Eg 'Development|PrintString',
'Utilities|Operators|Add', a macro like 'Utilities|FlowControl|ForLoop', a
dispatcher event handler like 'Default|EventDispatcherOnDamaged', an event like
'AddEvent|EventBeginPlay', or a named custom event with
'AddEvent|Custom|MyEventName'.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.create_node \
  --arguments '
{
  "graph": {},
  "pos": {},
  "type_id": "<value>"
}
'
```

## Expected output

Returns the new node

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

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
