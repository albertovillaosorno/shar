# Add event

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.add_event
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Adds an event node to the Blueprint's event graph.

If event_name matches an inherited overridable event, the new node is an
override of that event. Otherwise a new custom event with the given name is
created. Idempotent — if an event node with that name already exists, that node
is returned.

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
Use this mutation to add one inherited override or uniquely named custom event
to a reviewed SHAR Blueprint EventGraph when the exact event identity and
cleanup path are known.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact loaded Blueprint asset reference.
- Check `list_events` for the intended name before mutation.
- Choose an explicit graph position and a unique custom event name when not
  overriding an inherited event.
- Define node inspection, strict compilation, exact node deletion, and event
  inventory verification before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "blueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_EventParentLifecycle.BP_MCP_EventParentLifecycle"
  },
  "event_name": "MCP_EventLifecycle",
  "position": {
    "x": 640,
    "y": 320
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`list_events` contained no `MCP_EventLifecycle` entry before mutation. The
call returned an `EventGraph.K2Node_CustomEvent_0` reference. `get_node_infos`
reported type ID `AddEvent|Custom|MCP_EventLifecycle`, position `(640, 320)`,
and `OutputDelegate` plus `then` outputs. `list_events` then returned one
implemented event with that name, and strict compilation succeeded. Deleting
the returned node removed the event from `list_events`, and strict compilation
succeeded again.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- A name that does not resolve to an inherited event creates a custom event.
- The returned nested node reference is session-sensitive.
- The operation does not save the Blueprint automatically.
- Event inventory and node metadata are separate verification surfaces.
- Use the exact returned node with `delete_node` for disposable cleanup.
- Recompilation or structural graph edits can invalidate cached node refs.
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

### `event_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of an inherited event to override (e.g. 'ReceiveAnyDamage') or a name
for a new custom event.

### `position`

- Required: **no**
- Type: `object`
- Default: `{"x":0,"y":0}`
- Purpose:

IntPoint

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.add_event \
  --arguments '
{
  "blueprint": {},
  "event_name": "<value>"
}
'
```

## Expected output

The event node.

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
