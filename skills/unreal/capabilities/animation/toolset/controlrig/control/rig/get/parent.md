# Get parent

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.controlrig.ControlRigTools.get_parent
```

Toolset:

```text
animation_toolset.toolsets.controlrig.ControlRigTools
```

## What this tool does

Get the parent of a hierarchy element.

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
Use this tool to resolve the immediate parent of one typed Control Rig hierarchy
element.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use a hierarchy key returned by an inventory or traversal read.
- Treat root and missing elements carefully because they share the same
  sentinel.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "control_rig": {"refPath": "/Game/CR_SHAR_MCP_HierarchyFixture_1.CR_SHAR_MCP_HierarchyFixture_1"},
  "item": {"type": "Control", "name": "MainControl"}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two cycles returned AimNull as MainControl parent, RootBone as AimNull parent,
and `{type: None, name: None}` for RootBone.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Root elements and missing elements both return the invalid `None` key.
- Verify existence separately before interpreting the sentinel.
- Only the immediate parent is returned.
- Wrong Control Rig asset types fail during translation.
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
shar-unreal-mcp describe animation_toolset.toolsets.controlrig.ControlRigTools
```

1. Confirm every required input against the current schema.

## Inputs

### `control_rig`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `item`

- Required: **yes**
- Type: `object`
- Purpose:

RigElementKey

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.controlrig.ControlRigTools \
  animation_toolset.toolsets.controlrig.ControlRigTools.get_parent \
  --arguments '
{
  "control_rig": {},
  "item": {}
}
'
```

## Expected output

The parent's RigElementKey, or an invalid key if item is at root.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

RigElementKey

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
