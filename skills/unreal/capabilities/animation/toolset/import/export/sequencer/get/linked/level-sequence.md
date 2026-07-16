# Get linked level sequence

[Return to the central Unreal MCP index](../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.import_export.SequencerImportExportTools.get_linked_level_sequence
```

Toolset:

```text
animation_toolset.toolsets.import_export.SequencerImportExportTools
```

## What this tool does

Get the content path of the LevelSequence linked to an AnimSequence.

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
Use this tool to detect the LevelSequence linked to a SHAR AnimSequence before
synchronization, re-export, ownership, or cleanup decisions.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a valid AnimSequence asset.
- Treat an empty string as a valid unlinked state.
- Do not infer a link from skeleton compatibility or asset naming.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "anim_sequence": {
        "refPath": (
            "/Engine/Tutorial/SubEditors/TutorialAssets/Character/"
            "Tutorial_Walk_Fwd.Tutorial_Walk_Fwd"
        )
    }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Tutorial_Walk_Fwd and Tutorial_Idle both returned `""`, proving a valid unlinked
AnimSequence state. Passing a disposable LevelSequence failed with the explicit
AnimSequence type error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The return value is a plain content-path string.
- `""` is the valid sentinel for no linked LevelSequence.
- Direct `link_anim_sequence` currently reports its documented binding as
  missing.
- Export with `create_link=true` returned `false` in the compatible disposable
  fixture, so positive link output remains upstream-blocked.
- Engine tutorial assets were read only and never modified.
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
shar-unreal-mcp describe animation_toolset.toolsets.import_export.SequencerImportExportTools
```

1. Confirm every required input against the current schema.

## Inputs

### `anim_sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.import_export.SequencerImportExportTools \
  animation_toolset.toolsets.import_export.SequencerImportExportTools.get_linked_level_sequence \
  --arguments '
{
  "anim_sequence": {}
}
'
```

## Expected output

Content path of the linked LevelSequence, or empty string if no link exists.

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
