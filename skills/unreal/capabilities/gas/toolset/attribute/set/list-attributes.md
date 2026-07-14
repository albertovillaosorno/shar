# List attributes

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GASToolsets.AttributeSetToolset.ListAttributes
```

Toolset:

```text
GASToolsets.AttributeSetToolset
```

## What this tool does

Returns the gameplay attributes defined on a specific AttributeSet class.

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
Use this tool to confirm the exact gameplay-attribute names available on one
AttributeSet before SHAR GameplayEffects, abilities, tests, or UI bindings
reference those attributes.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Run `FindAttributeSetClasses` first and copy its exact `className` value.
- The target class must be loaded or discoverable as an AttributeSet.
- Use the reflected class name without adding Unreal's conventional `U` prefix.
- No instance, world, map, or PIE session is required.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "className": "GASToolsetsTestAttributeSet"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated calls returned exactly `Health` and `MaxHealth`, with full names
`GASToolsetsTestAttributeSet.Health` and
`GASToolsetsTestAttributeSet.MaxHealth`. The discovery descriptor contained the
same two entries, and `ObjectTools.search_subclasses` independently resolved
`/Script/GASToolsets.GASToolsetsTestAttributeSet`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `UGASToolsetsTestAttributeSet` is rejected; use the exact discovered name
  without the `U` prefix.
- An unknown class raises a not-found error and directs callers back to class
  discovery.
- The base `AttributeSet` class is accepted and returns an empty array.
- The result reports names and ownership only; it does not return values,
  metadata, replication settings, or numeric types.
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b
<!-- END MANUAL FIELD: manual-review-revision -->

- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Current**

## Before invocation

1. Run `shar-unreal-mcp doctor` and require `ready: true`.
1. Select this skill from the central index, not from memory.
1. Refresh the live schema:

```text
shar-unreal-mcp describe GASToolsets.AttributeSetToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `className`

- Required: **yes**
- Type: `string`
- Purpose:

The UClass name to look up, e.g. "UMyHealthSet". Raises a script error if the
class is not found or is not an AttributeSet subclass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GASToolsets.AttributeSetToolset \
  GASToolsets.AttributeSetToolset.ListAttributes \
  --arguments '
{
  "className": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

A list of attribute descriptors defined on the class.

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
