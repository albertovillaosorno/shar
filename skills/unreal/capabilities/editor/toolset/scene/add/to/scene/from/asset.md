# Add to scene from asset

[Return to the central Unreal MCP index](../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: World and UI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.scene.SceneTools.add_to_scene_from_asset
```

Toolset:

```text
editor_toolset.toolsets.scene.SceneTools
```

## What this tool does

Creates a new actor in the scene from an asset.

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
Use this tool to instantiate one reviewed SHAR asset as a scene actor for
bounded placement, visual, collision, material, or import verification.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR editor world must be loaded and PIE must be stopped.
- Resolve the exact asset path through the Asset Registry and confirm the
  requested actor label is absent.
- Capture the complete scene actor count and define the intended transform.
- Decide explicitly whether parenting or ground snapping is required.
- Retain the exact returned actor reference and define `remove_from_scene` as
  cleanup before creating a disposable actor.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "asset_path": "/Engine/BasicShapes/Cube.Cube",
  "name": "SHAR_MCP_ValidationCube",
  "xform": {
    "location": {
      "x": 2468,
      "y": 1357,
      "z": 500
    },
    "rotation": {
      "pitch": 0,
      "yaw": 30,
      "roll": 0
    },
    "scale": {
      "x": 1.25,
      "y": 1.25,
      "z": 1.25
    }
  },
  "snap_to_ground": false
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The pre-state contained 145 actors and no matching validation label. The call
returned one exact `StaticMeshActor` reference. `find_actors` returned only that
actor, the scene count became 146, and independent label and transform reads
matched the request within floating-point normalization. The actor contained
one `StaticMeshComponent`; a reflected property read returned
`/Engine/BasicShapes/Cube.Cube` as its `staticMesh`. Removing the returned actor
restored the count to 145 and left no matching label.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The validated case spawned an unparented static-mesh asset with ground
  snapping disabled.
- The returned actor class depends on the supplied asset type; do not assume
  every asset produces a `StaticMeshActor`.
- Parent-supplied transforms are parent-local, while unparented transforms are
  world-space.
- `snap_to_ground` can change the requested Z coordinate and requires a fresh
  transform read.
- Creation changes loaded level state and can persist if the level is saved.
- Generated actor references in `/Temp` worlds are session-specific.
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

### `asset_path`

- Required: **yes**
- Type: `string`
- Purpose:

The path to the asset to spawn (e.g. '/Game/Blueprints/MyActor').

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the actor instance.

### `parent`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `snap_to_ground`

- Required: **no**
- Type: `boolean`
- Default: `false`
- Purpose:

If set to true, will attempt to adjust the actors Z position so that the bottom
of its bounding box is on the ground.

### `xform`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a 3D transformation with optional location, rotation, and scale.
Unset fields mean "identity" when creating objects and "don't change" when
modifying existing ones.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.scene.SceneTools \
  editor_toolset.toolsets.scene.SceneTools.add_to_scene_from_asset \
  --arguments '
{
  "asset_path": "<value>",
  "name": "<value>",
  "xform": {}
}
'
```

## Expected output

The created actor or nothing if creation was unsuccessful.

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
