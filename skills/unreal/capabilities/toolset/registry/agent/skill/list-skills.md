# List skills

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ToolsetRegistry.AgentSkillToolset.ListSkills
```

Toolset:

```text
ToolsetRegistry.AgentSkillToolset
```

## What this tool does

Gets a summary of all AgentSkills in the project.

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
Use this tool to inventory Unreal AgentSkill assets and native skills before
SHAR loads detailed instructions for a relevant editor workflow.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Ensure the editor and Toolset Registry are ready.
- Treat returned paths as the authoritative inputs for `GetSkills`.
- Use descriptions only for routing; load details before following a skill.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned the same 20-entry dictionary. The inventory included one
native Dataflow skill, nine Niagara and Editor Python skills, and ten PCG
Blueprint skills. Each key was an exact skill path and each value was a routing
description.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The result combines native, Python-backed, and Blueprint-backed skills.
- Skill paths use different valid forms, including `/Script/...`, object paths,
  and generated `_C` paths.
- Descriptions can be multiline and are routing summaries rather than full
  instructions.
- Loaded plugins and engine revision determine the inventory.
- Use exact returned paths with `GetSkills`; do not reconstruct them from names.
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
shar-unreal-mcp describe ToolsetRegistry.AgentSkillToolset
```

1. Confirm every required input against the current schema.

## Inputs

This tool accepts no input fields.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  ToolsetRegistry.AgentSkillToolset \
  ToolsetRegistry.AgentSkillToolset.ListSkills \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

A dictionary where the key is the Skill path and the value is a description.

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
