# Set nanite enabled

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.static_mesh.StaticMeshTools.set_nanite_enabled
```

Toolset:

```text
editor_toolset.toolsets.static_mesh.StaticMeshTools
```

## What this tool does

Enables or disables Nanite for a static mesh.

Changing this setting triggers a mesh rebuild. Nanite is most beneficial for
high-polygon meshes. Low-polygon meshes may not benefit from Nanite.

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
Use this tool to toggle Nanite generation on a reviewed SHAR static mesh.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  exact live toolset schema before mutation.
- Use a disposable imported or duplicated mesh and capture the matching
  Nanite, LOD, material, or BodySetup reader before mutation.
- Define whole-folder asset cleanup and retain the source file only until
  import verification finishes.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "enabled": true,
  "mesh": {
    "refPath": "/Game/SHAR_MCP_Validation_Static_6e1b507e/SM_MCP_Imported_6e1b507e.SM_MCP_Imported_6e1b507e"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`is_nanite_enabled` changed to the opposite Boolean and then returned to its
original value after the inverse call.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Static-mesh mutations are persistent editor changes; use disposable assets
  and verify every structural reader after mutation.
- The setting controls Nanite data generation and may require a build or save
  before derived data is available.
- The imported source, duplicated cube, merged mesh, and complete validation
  folder were removed after verification.
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
shar-unreal-mcp describe editor_toolset.toolsets.static_mesh.StaticMeshTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `enabled`

- Required: **yes**
- Type: `boolean`
- Purpose:

True to enable Nanite, False to disable it.

### `mesh`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.static_mesh.StaticMeshTools \
  editor_toolset.toolsets.static_mesh.StaticMeshTools.set_nanite_enabled \
  --arguments '
{
  "enabled": false,
  "mesh": {}
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
