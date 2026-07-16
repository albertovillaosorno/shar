# Get active tags

[Return to the central Unreal MCP index](../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GASToolsets.AbilitySystemInspectorToolset.GetActiveTags
```

Toolset:

```text
GASToolsets.AbilitySystemInspectorToolset
```

## What this tool does

Returns the gameplay tags currently owned by the actor's AbilitySystemComponent
(includes loose tags, effect-granted tags, etc.).

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
Use this tool to inspect gameplay tags currently owned by a SHAR actor before
tag-gated ability, effect, interaction, or UI decisions.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a live Actor that owns an AbilitySystemComponent.
- Resolve the actor from the current editor world rather than persisting its
  path.
- Treat an empty array as valid only after proving the component exists.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
  "actor": {
    "refPath": (
      "/Temp/Untitled_1.Untitled_1:PersistentLevel."
      "AbilitySystemTestPawn_1"
    )
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Three disposable AbilitySystemTestPawn actors returned `[]` for active tags. The
same read on a plain Actor raised the explicit missing-component error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The return value is an already parsed array, not JSON text.
- A valid AbilitySystemComponent can legitimately return `[]`.
- A plain Actor raises `does not have an AbilitySystemComponent`.
- Spawnable actor object paths are temporary and change between sequences.
- The inventory can include loose tags and effect-granted tags when present.
- Re-read after effects, loose-tag changes, or ability-system initialization.
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
shar-unreal-mcp describe GASToolsets.AbilitySystemInspectorToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `actor`

- Required: **yes**
- Type: `object`
- Purpose:

The target actor.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GASToolsets.AbilitySystemInspectorToolset \
  GASToolsets.AbilitySystemInspectorToolset.GetActiveTags \
  --arguments '
{
  "actor": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

A sorted list of gameplay tag name strings. Raises a script error if Actor is
null or has no ASC.

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
