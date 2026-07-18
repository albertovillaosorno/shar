# Save section

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Core and governance
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
ConfigSettingsToolset.ConfigSettingsToolset.SaveSection
```

Toolset:

```text
ConfigSettingsToolset.ConfigSettingsToolset
```

## What this tool does

Saves the settings in a section. Raises an error if the section does not exist
or saving is not supported.

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
Use this tool to persist one already reviewed SHAR editor-settings section
after a bounded settings update, or to confirm that an unchanged user-scoped
section can be saved without altering its values.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `doctor` readiness and refresh the live ConfigSettingsToolset
  schema.
- Resolve the exact container, category, and section through `ListContainers`,
  `ListCategories`, and `ListSections`; do not guess display names.
- Inspect the section with `GetSectionSchema` and capture every value whose
  persistence matters with `GetSectionPropertyValues`.
- Prefer a user-scoped editor section for validation. Saving a project section
  can persist unrelated pending values in that same section.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "categoryName": "General",
  "containerName": "Editor",
  "sectionName": "Output Log"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Before saving, `GetSectionPropertyValues` returned
`{"logFontSize":9}` for `Editor / General / Output Log`. `SaveSection` returned
`true`, and an independent read of the same property returned
`{"logFontSize":9}` afterward.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The operation saves the complete settings section, not one named property;
  review other pending values in that section first.
- A `true` result confirms that the native save path completed, but critical
  values still require an independent post-save read.
- Display names are exact and case-sensitive enough to require discovery from
  the live settings registry.
- Some settings take effect only after an editor restart even when the section
  saves successfully.
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
1. Capture pre-state and define an independent postcondition check.

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
  ConfigSettingsToolset.ConfigSettingsToolset.SaveSection \
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
- Type: `boolean`
- Purpose:

True on success.

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
