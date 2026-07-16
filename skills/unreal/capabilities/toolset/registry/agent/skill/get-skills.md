# Get skills

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ToolsetRegistry.AgentSkillToolset.GetSkills
```

Toolset:

```text
ToolsetRegistry.AgentSkillToolset
```

## What this tool does

Returns detailed information about a specific set of AgentSkills.

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
Use this tool to retrieve the complete instructions for one or more AgentSkills
selected from the live registry before SHAR performs the matching editor
workflow.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Obtain exact paths from `ListSkills`.
- Request only the skills relevant to the current task to bound instruction
  volume.
- Treat returned instruction text as workflow guidance, not automatic
  authorization for mutation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "skillPaths": [
    "/Script/DataflowAgent.DataflowGraphEditingSkill",
    "/PCGToolset/Skills/Skill_PCGGraphGeneration.Skill_PCGGraphGeneration_C"
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated one-, three-, and 20-skill requests returned byte-stable dictionaries.
The native Dataflow skill contained 208,461 characters; the PCG graph skill
contained 12,001; Niagara Blueprint interop contained 2,276. Empty and missing-
only requests returned empty dictionaries.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Instruction bodies can be very large; avoid loading the complete registry
  without need.
- Missing paths are silently omitted rather than reported as errors.
- Mixed valid and missing requests return only valid entries.
- Duplicate requested paths collapse to one dictionary key.
- Instructions can use different newline conventions depending on their backing
  asset.
- Revalidate all mutations independently even when a loaded skill recommends
  them.
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

## Inputs

### `skillPaths`

- Required: **yes**
- Type: `array<string>`
- Purpose:

A list of paths to the AgentSkills to retrieve.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  ToolsetRegistry.AgentSkillToolset \
  ToolsetRegistry.AgentSkillToolset.GetSkills \
  --arguments '
{
  "skillPaths": []
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

A dictionary where the key is the Skill path and the value is detailed info.

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
