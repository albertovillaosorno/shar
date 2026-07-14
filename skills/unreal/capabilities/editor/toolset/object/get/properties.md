# Get properties

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.object.ObjectTools.get_properties
```

Toolset:

```text
editor_toolset.toolsets.object.ObjectTools
```

## What this tool does

Returns the values of one or more properties on an object.

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
Use this tool to read a narrow set of effective SHAR UObject properties when a
more specific toolset is unavailable or when an independent reflection check is
needed. The verified fixture reads map, gameplay-class, and split-screen values
from the Maps settings default object.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Resolve a valid object reference first.
- Discover readable names with `list_properties` and request only the needed
  subset.
- Parse `returnValue` as JSON.
- Treat the read as effective editor state, which can include inherited defaults.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "instance": {
    "refPath": "/Script/EngineSettings.Default__GameMapsSettings"
  },
  "properties": [
    "GameDefaultMap",
    "bUseSplitscreen"
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated reads were byte-identical and returned the OpenWorld map soft path
plus `true` for split screen. A five-property read matched
`GetSectionPropertyValues` after normalizing key casing, while
`DefaultEngine.ini` independently confirmed the explicit game-map package.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `returnValue` is a JSON-encoded string.
- Property lookup is case-insensitive, but returned keys preserve the requested
  spelling and order.
- An empty property list returns `{}`.
- If any requested property cannot be read, the entire call fails.
- Values can include Unreal defaults that are not authored in project INI files.
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

### `instance`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `properties`

- Required: **yes**
- Type: `array<string>`
- Purpose:

The names of the properties to query.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.object.ObjectTools \
  editor_toolset.toolsets.object.ObjectTools.get_properties \
  --arguments '
{
  "instance": {},
  "properties": []
}
'
```

## Expected output

A JSON formatted string of the properties and their values.

### `returnValue`

- Required: **yes**
- Type: `string`
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
