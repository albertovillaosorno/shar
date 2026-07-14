# List sections

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ConfigSettingsToolset.ConfigSettingsToolset.ListSections
```

Toolset:

```text
ConfigSettingsToolset.ConfigSettingsToolset
```

## What this tool does

Lists the names of all sections within a settings category, sorted
alphabetically. Raises an error if the container or category does not exist.

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
Use this tool to locate the exact settings page that owns a SHAR value before
requesting its schema or current values. The verified `Project/Project` path
contains the map, packaging, gameplay-tag, and general project sections.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Discover the container with `ListContainers`.
- Discover the category with `ListCategories`.
- Pass the registered display labels, including spaces where present.
- The canonical SHAR editor and settings registry must be ready.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "categoryName": "Project",
  "containerName": "Project"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned the same sorted nine-section array, including `Maps`,
`General`, `Packaging`, and `SupportedPlatforms`. A deliberately missing
category failed with `Category not found: Project/...`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Returned names are settings-page labels and may contain spaces.
- A listed section can use a custom widget and therefore lack a readable
  settings object; `SupportedPlatforms` reproduced that case.
- An unknown category is an error rather than an empty collection.
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

### `categoryName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the category (e.g. "Engine").

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
  ConfigSettingsToolset.ConfigSettingsToolset.ListSections \
  --arguments '
{
  "categoryName": "<value>",
  "containerName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Sorted array of section names.

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
