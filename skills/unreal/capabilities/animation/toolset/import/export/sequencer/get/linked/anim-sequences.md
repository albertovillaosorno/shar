# Get linked anim sequences

[Return to the central Unreal MCP index](../../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.import_export.SequencerImportExportTools.get_linked_anim_sequences
```

Toolset:

```text
animation_toolset.toolsets.import_export.SequencerImportExportTools
```

## What this tool does

Get content paths of all AnimSequences linked to a LevelSequence.

Linked AnimSequences auto-update when the LevelSequence changes. They are
created via export_anim_sequence with create_link=True.

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
Use this tool to detect AnimSequences currently linked to a SHAR LevelSequence
before export, rebake, synchronization, or cleanup decisions.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Pass a valid LevelSequence asset.
- Treat an empty array as a valid unlinked state.
- Do not infer links from skeletal animation sections alone.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```python
{
    "sequence": {
        "refPath": (
            "/Game/LS_SHAR_MCP_UnlinkedReadProbe_1."
            "LS_SHAR_MCP_UnlinkedReadProbe_1"
        )
    }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two disposable LevelSequences returned `[]` for linked AnimSequences. Passing
the engine Tutorial AnimSequences as the sequence argument failed with the
explicit LevelSequence type error.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The return value is an already parsed array.
- `[]` means no link metadata exists; it does not mean the sequence has no
  animation tracks.
- Direct `link_anim_sequence` currently rejects its documented binding argument.
- `export_anim_sequence(create_link=true)` returned `false` in the compatible
  disposable fixture, so populated-link output remains upstream-blocked.
- Disposable sequence paths must not be persisted.
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
shar-unreal-mcp describe animation_toolset.toolsets.import_export.SequencerImportExportTools
```

1. Confirm every required input against the current schema.

## Inputs

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.import_export.SequencerImportExportTools \
  animation_toolset.toolsets.import_export.SequencerImportExportTools.get_linked_anim_sequences \
  --arguments '
{
  "sequence": {}
}
'
```

## Expected output

List of content paths (e.g. '/Game/Anim/Test_Anim') for each linked
AnimSequence. Empty list if no links exist.

### `returnValue`

- Required: **yes**
- Type: `array<string>`
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
