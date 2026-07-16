# Find cue notify assets

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GASToolsets.GameplayCueToolset.FindCueNotifyAssets
```

Toolset:

```text
GASToolsets.GameplayCueToolset
```

## What this tool does

Returns all GameplayCueNotify assets found in the project via the asset
registry.

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
Use this tool to inventory GameplayCueNotify assets and their cue-tag mappings
before SHAR audits cue implementation coverage.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass an explicit parent tag or an empty string for all notify assets.
- Treat an empty result as a valid no-notify baseline.
- Compare the result with `ListCues` and `FindCueTagsWithoutNotifies`.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "parentTag": ""
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two complete calls and two `GameplayCue`-root calls returned empty arrays. The
project still contained registered tag `GameplayCue.Test`, proving notify assets
and cue tags are independent inventories.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Empty means the Asset Registry found no qualifying GameplayCueNotify assets.
- A registered cue tag can exist without a notify asset.
- Unknown parent tags raise an explicit parent-tag error.
- Results depend on mounted content and current Asset Registry state.
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
shar-unreal-mcp describe GASToolsets.GameplayCueToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `parentTag`

- Required: **yes**
- Type: `string`
- Purpose:

If non-empty, only notifies whose cue tag descends from this tag are returned.
Pass an empty string to return all notify assets in the project.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GASToolsets.GameplayCueToolset \
  GASToolsets.GameplayCueToolset.FindCueNotifyAssets \
  --arguments '
{
  "parentTag": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

A list of notify descriptors, sorted by cue tag.

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
