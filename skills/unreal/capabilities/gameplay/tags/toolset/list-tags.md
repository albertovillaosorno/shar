# List tags

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GameplayTagsToolset.GameplayTagsToolset.ListTags
```

Toolset:

```text
GameplayTagsToolset.GameplayTagsToolset
```

## What this tool does

Returns gameplay tags registered in the project.

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
Use this tool to inventory registered Gameplay Tags before SHAR inspects tag
metadata, searches references, or prepares a separately authorized tag change.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass an explicit parent tag or an empty string for the complete registry.
- Treat returned fully qualified names as authoritative inputs for other
  Gameplay Tags tools.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "parentTag": "GameplayCue"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two complete calls returned 31 sorted tags across Enhanced Input, Gameplay Cue,
Input User Settings, Niagara, StateTree test, and generic test namespaces.
`GameplayCue` returned only `GameplayCue.Test`; using leaf parent
`GameplayCue.Test` returned an empty array.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The parent itself is not included; only descendants are returned.
- A valid leaf parent therefore returns an empty array.
- Unknown parent strings also return an empty array rather than an error.
- Results include engine and plugin tags, not only project-authored tags.
- Registry contents change with loaded plugins and tag-source configuration.
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
shar-unreal-mcp describe GameplayTagsToolset.GameplayTagsToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `parentTag`

- Required: **yes**
- Type: `string`
- Purpose:

If non-empty, only tags that are descendants of this tag are returned. For
example, passing "Character.State" returns "Character.State.Dead",
"Character.State.Stunned", etc. Pass an empty string to return all tags.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GameplayTagsToolset.GameplayTagsToolset \
  GameplayTagsToolset.GameplayTagsToolset.ListTags \
  --arguments '
{
  "parentTag": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<string>`
- Purpose:

A sorted list of fully-qualified tag names.

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
