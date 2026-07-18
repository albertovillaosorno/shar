# Get registry info

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
DataRegistryToolset.DataRegistryTools.GetRegistryInfo
```

Toolset:

```text
DataRegistryToolset.DataRegistryTools
```

## What this tool does

Returns detailed information about a specific registry.

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
Use this tool to inspect one active Data Registry definition before SHAR
gameplay systems consume registry-backed data.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Call `ListRegistries` first and use an exact returned registry name.
- Require the registry subsystem to be initialized in the current editor
  session.
- Treat an empty registry inventory as a hard prerequisite failure.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "registryName": "MCPValidationMissingRegistry"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The live project returned `[]` from `ListRegistries`. The synthetic name
`MCPValidationMissingRegistry` failed closed with `Registry not found.
Available: (none)`. Generic DataAsset creation also rejected
`/Script/DataRegistry.DataRegistry` because the class cannot be stored through
that constructor.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The example is a validated negative diagnostic, not a registry name to reuse.
- These tools query registered runtime state; an asset path is not a substitute
  for `registryName`.
- Do not accept an empty or failed response when no registry was listed.
- Reading metadata does not initialize or register the registry.
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
shar-unreal-mcp describe DataRegistryToolset.DataRegistryTools
```

1. Confirm every required input against the current schema.

## Inputs

### `registryName`

- Required: **yes**
- Type: `string`
- Purpose:

The registry name.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  DataRegistryToolset.DataRegistryTools \
  DataRegistryToolset.DataRegistryTools.GetRegistryInfo \
  --arguments '
{
  "registryName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Detailed info including description and id format.

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
