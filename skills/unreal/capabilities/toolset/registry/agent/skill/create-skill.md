# Create skill

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ToolsetRegistry.AgentSkillToolset.CreateSkill
```

Toolset:

```text
ToolsetRegistry.AgentSkillToolset
```

## What this tool does

Creates a new AgentSkill.

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
Use this mutation to create a bounded runtime AgentSkill asset when SHAR needs
editor-loaded project guidance whose routing description and instruction body
must be discoverable through the native AgentSkill registry.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Confirm the target folder and asset path are absent with `AssetTools.exists`.
- Confirm `ListSkills` does not already contain the intended generated class
  path.
- Provide a short routing description and put the complete instruction body in
  `details.instructions`.
- Define explicit `AssetTools.save_assets` persistence and `AssetTools.delete`
  cleanup before creating a disposable fixture.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "assetName": "MCP_AgentSkillLifecycle",
  "description": "Routes disposable SHAR AgentSkill lifecycle validation.",
  "details": {
    "instructions": "Use this disposable skill only to verify AgentSkill creation, discovery, detail retrieval, update, persistence, and cleanup in the canonical SHAR editor project."
  },
  "folderPath": "/Game/SHAR_MCP_Validation/AgentSkills"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The tool returned this generated class path:

```text
/Game/SHAR_MCP_Validation/AgentSkills/MCP_AgentSkillLifecycle.MCP_AgentSkillLifecycle_C
```

`AssetTools.exists` then returned true for the underlying asset.
`get_asset_class` returned `MCP_AgentSkillLifecycle_C`. `ListSkills` returned
the exact routing description under the generated class path, while `GetSkills`
returned the exact `details.instructions` value. Creation did not write a
`.uasset` automatically; an explicit `save_assets` call returned true and
created the package on disk.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `details` must contain the required `instructions` string.
- The return value is the generated class object path ending in `_C`; the
  underlying asset path omits the object suffix.
- The tool can create missing content folders, but it does not save the new
  package automatically.
- The AgentSkill toolset has no delete operation. Use the reviewed AssetTools
  delete capability and verify both registry removal and filesystem cleanup.
- `ListSkills` exposes descriptions; use `GetSkills` for instruction bodies.
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

### `assetName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the skill to create i.e. MySkill.

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

### `folderPath`

- Required: **yes**
- Type: `string`
- Purpose:

The folder in which to create the skill. i.e. /Game/Skills/.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  ToolsetRegistry.AgentSkillToolset \
  ToolsetRegistry.AgentSkillToolset.CreateSkill \
  --arguments '
{
  "assetName": "<value>",
  "description": "<value>",
  "details": {},
  "folderPath": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

The path for the created Skill class. Empty if unsuccessful.

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
