# Find attribute set classes

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GASToolsets.AttributeSetToolset.FindAttributeSetClasses
```

Toolset:

```text
GASToolsets.AttributeSetToolset
```

## What this tool does

Returns all AttributeSet subclasses found in the project, including their
attributes. Covers both native C++ subclasses and Blueprint subclasses
discovered via the asset registry.

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
Use this tool before SHAR gameplay-ability work to inventory the AttributeSet
classes visible to the editor and review their declared gameplay attributes
before effects, abilities, or UI bindings depend on them.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- The canonical SHAR editor must be ready with GameplayAbilities and
  GASToolsets loaded.
- Native classes must be loaded; Blueprint subclasses depend on current Asset
  Registry discovery.
- No map, actor selection, or PIE session is required.
- Distinguish engine or plugin test classes from SHAR-authored classes.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two repeated calls returned the same two descriptors in class-name order:
`AbilitySystemTestAttributeSet` with 16 attributes and
`GASToolsetsTestAttributeSet` with two. `ListAttributes` reproduced each
attribute list. `ObjectTools.search_subclasses` independently resolved the
classes under `/Script/GameplayAbilities` and `/Script/GASToolsets`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The current editor exposes only two native test AttributeSets; both have an
  empty `assetPath`, and no SHAR Blueprint AttributeSet was discovered.
- Class descriptors are sorted by class name, but attributes retain reflected
  declaration order rather than alphabetical order.
- Results can change when plugins, native modules, or Blueprint registry data
  change.
- An empty `assetPath` identifies a native class, not a failed lookup.
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
shar-unreal-mcp describe GASToolsets.AttributeSetToolset
```

1. Confirm every required input against the current schema.

## Inputs

This tool accepts no input fields.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GASToolsets.AttributeSetToolset \
  GASToolsets.AttributeSetToolset.FindAttributeSetClasses \
  --arguments '
{}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

A list of attribute set descriptors sorted by class name.

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
