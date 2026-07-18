# Remove cue tag

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Gameplay and AI
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
GASToolsets.GameplayCueToolset.RemoveCueTag
```

Toolset:

```text
GASToolsets.GameplayCueToolset
```

## What this tool does

Removes a gameplay cue tag from the project.

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
Use this tool to remove one reviewed SHAR Gameplay Cue tag after deleting or
migrating its notify assets.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live tag or Gameplay Cue toolset schema.
- Capture the complete matching tag or cue inventory before mutation and use a
  unique fully qualified validation name.
- Use `ListCues`, `GetCueInfo`, and AssetTools inventory reads as independent
  postconditions.
- Delete associated disposable notify assets before removing the cue tag.
- Snapshot gameplay-tag configuration files before mutation and restore the
  exact pre-state during cleanup.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "cueTag": "GameplayCue.MCP.Validation.Round9e31d6a7"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
After deleting the disposable notify asset, `RemoveCueTag` returned true,
`ListCues` no longer contained the validation cue, and the cue inventory
returned to its original count of one.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Gameplay tags are persistent project configuration, not transient
  editor-only state; always capture and verify the exact configuration
  boundary.
- Remove notify assets before the tag so no orphaned cue asset remains.
- Removing the final validation cue can leave an empty validation-created
  config file that must be cleaned only when it did not exist before the test.
- The reproduced lifecycle restored the original tag and cue inventories and
  left no config or asset residue.
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `cueTag`

- Required: **yes**
- Type: `string`
- Purpose:

The fully-qualified cue tag to remove (e.g. "GameplayCue.Character.Death").

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  GASToolsets.GameplayCueToolset \
  GASToolsets.GameplayCueToolset.RemoveCueTag \
  --arguments '
{
  "cueTag": "<value>"
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

True if the tag was removed successfully.

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
