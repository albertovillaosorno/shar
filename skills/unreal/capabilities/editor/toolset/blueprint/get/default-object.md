# Get default object

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.get_default_object
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Returns the Class Default Object (CDO) for a Blueprint's.

ObjectTools list/set/get property will get the CDO automatically, so this
should primarily be used before calling tools that want to operate on the
object inside the blueprint (like ActorTools).

Note: The Blueprint must be compiled for the CDO to reflect the latest state.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

Use the returned structured evidence directly, but still confirm the live
schema because names do not prove side effects.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to obtain a compiled Blueprint class default object before SHAR
component or object-property inspection.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact existing Blueprint asset ref.
- Compile the Blueprint before relying on its generated layout.
- Keep the Blueprint loaded while consuming returned nested refs.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "blueprint": {
    "refPath": "/Game/B_SHAR_MCP_Blueprint_ReadFixture.B_SHAR_MCP_Blueprint_ReadFixture"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned the same `Default__B_SHAR_MCP_Blueprint_ReadFixture_C` ref.
Filtered ActorTools inspection then found exactly one Niagara Component on that
CDO. Cleanup deleted the disposable Blueprint.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The CDO is not a placed runtime actor.
- Recompilation or deletion can invalidate the CDO and component refs.
- Missing Blueprint refs fail during parameter translation.
- An uncompiled Blueprint can expose stale generated state.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

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

## Inputs

### `blueprint`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.get_default_object \
  --arguments '
{
  "blueprint": {}
}
'
```

## Expected output

The Class Default Object for the Blueprint's generated class.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Verification

- Check the returned `isError` state and structured output.
- Compare returned identities and counts with the requested scope.
- Treat transport success as insufficient evidence by itself.
- Confirm the response belongs to the open editor project.
- Reject evidence derived from stale discovery state.

## Common failure modes

- The skill may be stale; run `describe` and regenerate the catalog.
- A required editor object or asset may not be loaded.
- Placeholder values are not valid project identities.
- Native validation may reject semantically invalid JSON values.
