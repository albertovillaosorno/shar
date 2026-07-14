# Search subclasses

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.object.ObjectTools.search_subclasses
```

Toolset:

```text
editor_toolset.toolsets.object.ObjectTools
```

## What this tool does

Finds all subclasses of a given class.

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
Use this tool to resolve a loaded Unreal class from a narrow partial name before
SHAR performs reflection or class-specific inspection. The verified query finds
the class that backs the project Maps settings object.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- `base_class.refPath` must resolve to a valid UClass.
- The live schema requires `class_name`, even though its prose describes the
  filter as optional.
- Use a selective substring to avoid an unnecessarily broad class enumeration.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "base_class": {
    "refPath": "/Script/CoreUObject.Object"
  },
  "class_name": "GameMapsSettings"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned exactly
`/Script/EngineSettings.GameMapsSettings`. A lowercase filter returned the same
result, and `get_class` independently returned that class for
`Default__GameMapsSettings`. A deliberately missing substring returned `[]`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The substring filter is case-insensitive.
- No match is an empty array rather than an error.
- `class_name` is required by the current live schema.
- An empty or very broad filter can produce a large editor-session class set.
- Enabled modules and plugins can change the available subclass results.
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
shar-unreal-mcp describe editor_toolset.toolsets.object.ObjectTools
```

1. Confirm every required input against the current schema.

## Inputs

### `base_class`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `class_name`

- Required: **yes**
- Type: `string`
- Purpose:

Optional case-insensitive substring filter on the class path.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.object.ObjectTools \
  editor_toolset.toolsets.object.ObjectTools.search_subclasses \
  --arguments '
{
  "base_class": {},
  "class_name": "<value>"
}
'
```

## Expected output

A list of subclasses matching the filter.

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
