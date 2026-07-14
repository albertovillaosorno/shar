# List categories

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ConfigSettingsToolset.ConfigSettingsToolset.ListCategories
```

Toolset:

```text
ConfigSettingsToolset.ConfigSettingsToolset
```

## What this tool does

Lists the names of all categories within a settings container, sorted
alphabetically. Raises an error if the container does not exist.

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
Use this tool to narrow SHAR settings inspection to a functional area before
listing sections. The verified `Project` container exposes categories for
engine, game, platform, plugin, editor, and project configuration.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Run `ListContainers` first and use a returned container identity.
- The canonical SHAR editor and settings registry must be ready.
- Choose the narrowest container that owns the intended setting.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "containerName": "Project"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned the same sorted six-category array: `Editor`, `Engine`,
`Game`, `Platforms`, `Plugins`, and `Project`. A deliberately missing container
failed with `Container not found` instead of returning an empty array.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Category names are registered settings labels, not INI section names.
- An unknown container is a native error and produces no `returnValue`.
- Categories can change when enabled modules or plugins register or remove
  settings pages.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe ConfigSettingsToolset.ConfigSettingsToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `containerName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the container (e.g. "Project").

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  ConfigSettingsToolset.ConfigSettingsToolset \
  ConfigSettingsToolset.ConfigSettingsToolset.ListCategories \
  --arguments '
{
  "containerName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Sorted array of category names.

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
