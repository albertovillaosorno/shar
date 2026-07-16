# Get section schema

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ConfigSettingsToolset.ConfigSettingsToolset.GetSectionSchema
```

Toolset:

```text
ConfigSettingsToolset.ConfigSettingsToolset
```

## What this tool does

Returns a JSON Schema describing the user-visible properties of a settings
section. The schema maps each property name to its type, description, and
constraints. Raises an error if the section does not exist or has no backing
settings object (e.g. uses a custom widget instead).

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
Use this tool to discover property names and value shapes before reading or
changing SHAR project settings. The verified `Project/Project/Maps` schema
covers startup maps, default maps, game classes, split-screen behavior, and
map-to-game-mode mappings.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Discover the exact container, category, and section through the list tools.
- The section must have a backing settings object.
- Parse the returned string as JSON before selecting property names.
- Do not infer property spelling from the settings-page display text.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "categoryName": "Project",
  "containerName": "Project",
  "sectionName": "Maps"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two consecutive calls returned equivalent decoded property maps with 17
entries. The schema described `bUseSplitscreen` as Boolean and represented soft
references with nested `refPath` fields. `GetSectionPropertyValues` then read
five schema-declared names successfully, while `SupportedPlatforms` raised the
documented no-settings-object error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON-encoded string, not an already decoded object.
- The decoded value is a top-level property map rather than a conventional
  schema object containing a `properties` member.
- Some listed sections use custom widgets; `SupportedPlatforms` failed with
  `Section has no settings object`.
- Property keys use the toolset's lower-camel serialization names.
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

### `sectionName`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the section (e.g. "General").

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  ConfigSettingsToolset.ConfigSettingsToolset \
  ConfigSettingsToolset.ConfigSettingsToolset.GetSectionSchema \
  --arguments '
{
  "categoryName": "<value>",
  "containerName": "<value>",
  "sectionName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

JSON Schema string describing the section's properties, or empty string on
failure.

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
