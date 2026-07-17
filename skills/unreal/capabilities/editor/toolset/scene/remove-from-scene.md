# Remove from scene

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.remove_from_scene
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Deletes an actor from the scene.

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
Use this tool to delete one exact disposable or explicitly approved SHAR scene
actor after its identity, dependencies, persistence requirements, and recovery
contract have been verified.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve the exact actor from a current scene read or a creation return value.
- Capture the complete scene count and any label, transform, component, parent,
  reference, or dependent evidence required by the task.
- Confirm the actor is disposable or that an approved reconstruction path
  exists.
- Restrict the call to one exact returned actor reference and define the
  expected post-removal scene inventory.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "actor": {
    "refPath": "/Temp/Untitled_1.Untitled_1:PersistentLevel.TargetPoint_UAID_00E04C680267E2EE02_1158365599"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The addressed actor was the exact disposable `TargetPoint` returned by
`add_to_scene_from_class`. The call returned `returnValue: true`. A fresh
`find_actors` query returned no actor with the validation label, and the full
scene count returned from 146 to its captured value of 145. Final cleanup found
no remaining validation actor.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Actor removal is destructive for the addressed loaded actor and its owned
  runtime instance state.
- A `true` response requires an independent absence check and scene-count or
  identity comparison.
- The validated deletion was safe because the actor was created during the same
  bounded operation and its exact returned reference was retained.
- Removing project actors can invalidate references, attachments, level logic,
  or gameplay contracts and can persist when the level is saved.
- Removal of a missing actor was not exercised.
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
shar-unreal-mcp describe editor_toolset.toolsets.scene.SceneTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `actor`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.remove_from_scene \
  --arguments '
{
  "actor": {}
}
'
```

## Expected output

True if the actor was removed.

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
