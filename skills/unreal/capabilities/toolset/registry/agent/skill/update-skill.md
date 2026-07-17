# Update skill

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ToolsetRegistry.AgentSkillToolset.UpdateSkill
```

Toolset:

```text
ToolsetRegistry.AgentSkillToolset
```

## What this tool does

Updates an existing AgentSkill.

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
Use this mutation to replace the routing description and instruction body of
one existing SHAR runtime AgentSkill after its exact generated class path and
current registry values have been captured.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain the exact generated class path from `CreateSkill` or `ListSkills`.
- Capture the current description with `ListSkills` and instructions with
  `GetSkills` before mutation.
- Provide both required replacement values: `description` and
  `details.instructions`.
- Plan an explicit `AssetTools.save_assets` call and an independent registry
  round-trip after the update.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "description": "Routes updated disposable SHAR AgentSkill lifecycle validation.",
  "details": {
    "instructions": "Use this updated disposable skill to verify that AgentSkill descriptions and instruction bodies round-trip through UpdateSkill, ListSkills, GetSkills, explicit save, and bounded cleanup."
  },
  "skillPath": "/Game/SHAR_MCP_Validation/AgentSkills/MCP_AgentSkillLifecycle.MCP_AgentSkillLifecycle_C"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The update returned true. A fresh `ListSkills` call returned the replacement
routing description, and `GetSkills` returned the replacement instruction body
under the same generated class path. `save_assets` then returned true, and the
persisted `.uasset` SHA-256 changed from its post-create value. Deleting the
bounded validation folder removed the asset, unregistered the skill, made
`AssetTools.exists` return false, and caused `GetSkills` for the removed path to
return an empty dictionary.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `skillPath` must be the full generated class object path ending in `_C`.
- `description` and `details.instructions` are both required on every call.
- Registry reads reflect the update before the package is saved; call
  `AssetTools.save_assets` to persist it to disk.
- A true boolean response is insufficient verification. Re-read both the
  description and instruction body independently.
- The AgentSkill toolset has no delete operation; cleanup requires AssetTools.
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
shar-unreal-mcp describe ToolsetRegistry.AgentSkillToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `description`

- Required: **yes**
- Type: `string`
- Purpose:

The brief description of the skill.

### `details`

- Required: **yes**
- Type: `object`
- Purpose:

Detailed information about how to use the skill.

### `skillPath`

- Required: **yes**
- Type: `string`
- Purpose:

The full path to the skill to modify i.e. /Game/Skills/MySkill.MySkill_C.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  ToolsetRegistry.AgentSkillToolset \
  ToolsetRegistry.AgentSkillToolset.UpdateSkill \
  --arguments '
{
  "description": "<value>",
  "details": {},
  "skillPath": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

True if the skill was updated.

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
