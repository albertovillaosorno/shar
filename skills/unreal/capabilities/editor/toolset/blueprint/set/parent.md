# Set parent

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.blueprint.BlueprintTools.set_parent
```

Toolset:

```text
editor_toolset.toolsets.blueprint.BlueprintTools
```

## What this tool does

Reparents a Blueprint to a new parent class.

The Blueprint must be recompiled after reparenting.

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
Use this mutation to reparent one reviewed SHAR Blueprint to an exact native or
Blueprint class after direct-parent pre-state and a restoration path have been
captured.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Read the current direct parent with `get_parent`.
- Confirm the target class is compatible with the Blueprint's intended role.
- Capture the original parent class as the exact inverse.
- Use one disposable or explicitly approved Blueprint.
- Define strict compilation and fresh parent reads after both mutation and
  restoration.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "blueprint": {
    "refPath": "/Game/SHAR_MCP_Validation/BP_MCP_EventParentLifecycle.BP_MCP_EventParentLifecycle"
  },
  "parent_class": {
    "refPath": "/Script/Engine.Pawn"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The disposable Blueprint initially reported `/Script/Engine.Actor`. Reparenting
to `/Script/Engine.Pawn` returned `null`; `get_parent` returned the Pawn class,
and strict compilation succeeded. Reparenting back to `/Script/Engine.Actor`
returned `null`; the original direct parent was restored and strict compilation
succeeded again. A final parent read still returned Actor before asset cleanup.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The tool has no structured return value; use `get_parent` independently.
- Reparenting can change inherited functions, events, defaults, and components.
- A class path can be valid yet semantically incompatible with the Blueprint.
- Structural references may become stale after reparenting and compilation.
- Always retain the original parent class and verify restoration explicitly.
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

### `blueprint`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `parent_class`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.blueprint.BlueprintTools \
  editor_toolset.toolsets.blueprint.BlueprintTools.set_parent \
  --arguments '
{
  "blueprint": {},
  "parent_class": {}
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
