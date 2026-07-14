# Get section property values

[Return to the central Unreal MCP index](../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ConfigSettingsToolset.ConfigSettingsToolset.GetSectionPropertyValues
```

Toolset:

```text
ConfigSettingsToolset.ConfigSettingsToolset
```

## What this tool does

Returns the current values of the specified properties as a JSON object. Raises
an error if the section does not exist, has no settings object, or any
requested property cannot be read.

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
Use this tool to verify the effective SHAR map and gameplay-class defaults
before PIE, packaging, or map-import work. It reads both authored overrides and
values inherited from Unreal defaults without modifying the section.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Discover the section and property names with the preceding list and schema
  tools.
- Request only names present in the current section schema.
- Parse the returned string as JSON.
- The section must expose a backing settings object.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "categoryName": "Project",
  "containerName": "Project",
  "propertyNames": [
    "editorStartupMap",
    "gameDefaultMap",
    "globalDefaultGameMode",
    "gameInstanceClass",
    "bUseSplitscreen"
  ],
  "sectionName": "Maps"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned byte-identical values: both map references resolved to
`/Engine/Maps/Templates/OpenWorld.OpenWorld`, the game mode to
`/Script/Engine.GameModeBase`, the game instance to
`/Script/Engine.GameInstance`, and split screen to `true`. ObjectTools returned
the same five values, while `DefaultEngine.ini` independently confirmed the
explicit game-map package.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON-encoded string.
- Effective values include inherited engine defaults, not only keys authored in
  the project's INI files.
- An empty `propertyNames` array returns `{}`.
- Any unreadable property makes the whole call fail; a deliberately missing
  property reproduced that behavior.
- Soft object paths include the object name after the package path.
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

### `propertyNames`

- Required: **yes**
- Type: `array<string>`
- Purpose:

The names of the properties to read.

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
  ConfigSettingsToolset.ConfigSettingsToolset.GetSectionPropertyValues \
  --arguments '
{
  "categoryName": "<value>",
  "containerName": "<value>",
  "propertyNames": [],
  "sectionName": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

JSON object mapping property names to their current values, or empty string on
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
