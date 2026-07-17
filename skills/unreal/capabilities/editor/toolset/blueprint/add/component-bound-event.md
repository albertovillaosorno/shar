# Add component bound event

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.add_component_bound_event
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Creates a component bound event node in the event graph.

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
Use this mutation to add one delegate-bound event node for an exact SHAR
Blueprint component template after the component and supported event name have
been discovered from the live generated class.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact Blueprint and target graph in the current editor session.
- Obtain the Blueprint generated default object, then discover the component
  template with ActorTools component reads.
- Enumerate valid delegate names with `list_component_events`; do not invent an
  event name.
- Capture graph DSL before mutation and define node inspection, strict
  compilation, and bounded disposable-asset cleanup before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "component": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_ComponentEventLifecycle.BP_MCP_ComponentEventLifecycle_C:DefaultSceneRoot_GEN_VARIABLE"
  },
  "event_name": "OnComponentActivated",
  "graph": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_ComponentEventLifecycle.BP_MCP_ComponentEventLifecycle:EventGraph"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The disposable actor Blueprint default object exposed one component, which also
matched its root: `DefaultSceneRoot_GEN_VARIABLE`. `list_component_events`
returned `PhysicsVolumeChangedDelegate`, `OnComponentActivated`, and
`OnComponentDeactivated`. Binding `OnComponentActivated` returned
`EventGraph.K2Node_ComponentBoundEvent_0`. `get_node_infos` reported type ID
`AddEvent|OnComponentActivated(DefaultSceneRoot)`, position `(112, 0)`, and the
`OutputDelegate`, `then`, `Component`, and `bReset` output pins. The graph DSL
gained
`(event OnComponentActivated(DefaultSceneRoot) (Component bReset))`, and strict
Blueprint compilation succeeded. Deleting the complete disposable validation
folder restored virtual and physical asset absence.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Use only event names returned by `list_component_events` for the exact
  component.
- Component template paths are nested generated-class identities and are
  session-sensitive.
- The tool does not expose a position argument; Unreal selected `(112, 0)` in
  the reproduced graph.
- The declared return may represent a created or pre-existing bound event, so
  inspect graph pre-state before relying on node-count changes.
- This validation used whole disposable-asset deletion for cleanup; node-level
  removal on a persistent Blueprint requires separate reviewed verification.
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

### `component`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `event_name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the delegate event to bind, e.g. 'OnComponentBeginOverlap'.

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
  editor_toolset.toolsets.blueprint.BlueprintTools.add_component_bound_event \
  --arguments '
{
  "component": {},
  "event_name": "<value>",
  "graph": {}
}
'
```

## Expected output

The created or pre-existing component bound event node.

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
