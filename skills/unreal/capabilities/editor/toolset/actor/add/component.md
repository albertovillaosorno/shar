# Add component

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.actor.ActorTools.add_component
```

Toolset:

```text
editor_toolset.toolsets.actor.ActorTools
```

## What this tool does

Adds a component to an actor instance or blueprint.

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
Use this tool to add one explicitly typed component to a bounded SHAR scene
actor for disposable integration checks or for a reviewed actor-instance
extension whose persistence and ownership are already defined.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR project must have the owner actor loaded.
- Resolve the exact owner and capture its complete component list with
  `get_components`.
- Confirm the component class is valid for the owner and choose a unique native
  component name.
- Define cleanup with `remove_component` using the exact reference returned by
  this call.
- Do not save the level for a disposable validation component.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "owner": {
    "refPath": "/Temp/Untitled_1.Untitled_1:PersistentLevel.PlayerStart_UAID_F02F74551BF5599B01_1153002503"
  },
  "component_type": {
    "refPath": "/Script/Engine.SceneComponent"
  },
  "name": "SHAR_MCP_ValidationComponent"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The call returned an exact component reference ending in
`SHAR_MCP_ValidationComponent`. A separate `get_components` call returned that
same reference in addition to the actor's four original components. Removing
the returned component restored the original component list exactly, which a
final read confirmed.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The validated case added a component to a loaded actor instance, not a
  Blueprint asset.
- The returned component reference is the cleanup authority; do not reconstruct
  it from naming assumptions when the returned value is available.
- Adding a component changes loaded level state and can become persistent if the
  level is saved.
- Component registration, attachment, transforms, and runtime behavior require
  separate verification when they matter to the requested outcome.
- Actor and component references from `/Temp` worlds are session-specific.
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
shar-unreal-mcp describe editor_toolset.toolsets.actor.ActorTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `component_type`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the new component.

### `owner`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.actor.ActorTools \
  editor_toolset.toolsets.actor.ActorTools.add_component \
  --arguments '
{
  "component_type": {},
  "name": "<value>",
  "owner": {}
}
'
```

## Expected output

The newly added actor component.

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
