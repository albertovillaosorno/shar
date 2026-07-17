# Paste tracks

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.sequencer.SequencerTools.paste_tracks
```

Toolset:

```text
animation_toolset.toolsets.sequencer.SequencerTools
```

## What this tool does

Paste tracks from the clipboard onto the given bindings.

## When to use it

Use this skill when the requested outcome matches its purpose.
Choose it only when it is the most specific available action.
Do not substitute it for a narrower read or mutation capability.

## Technical execution posture

The native identity does not establish side effects. Review the live schema and
editor context before invocation.

## Human-authored guidance

Edit only between matching manual-field markers.
Regeneration preserves those contents and refreshes everything else.
A revision mismatch marks preserved guidance for human review.

### SHAR-specific use cases

<!-- BEGIN MANUAL FIELD: project-use-cases -->
Use this tool to paste tracks reviewed folders, bindings, tracks, or sections
while assembling SHAR mission, dialogue, camera, and cinematic sequences.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live SequencerTools schema.
- Open the exact disposable or recoverable Level Sequence and capture the
  independent reader state before mutation.
- Resolve every nested object from the same sequence and rediscover it after
  structural edits that can stale references.
- Use only the opaque token returned by the matching copy tool in the same
  editor session.
- Define an exact inverse or whole disposable-asset cleanup before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "paste_token": "<object-text token returned by copy_tracks in this editor session>",
  "sequence": {
    "refPath": "/Game/SHAR_MCP_Validation_Batch50_555102d0/LS_MCP_Batch50_555102d0.LS_MCP_Batch50_555102d0"
  },
  "target_bindings": [
    {
      "bindingId": "1E2C1575-4979-01F9-A760-FE9778875794",
      "sequence": {
        "refPath": "/Game/SHAR_MCP_Validation_Batch50_555102d0/LS_MCP_Batch50_555102d0.LS_MCP_Batch50_555102d0"
      }
    }
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
The matching copy token was pasted into the disposable SHAR sequence.
`get_tracks_on_binding` changed from 3 items to 4 items, and the paste call
returned the exact newly created object references.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Sequence, binding, folder, track, and section references are live editor
  identities and can become stale after structural edits or closing Sequencer.
- Copy tokens are opaque Unreal object-text payloads, can be thousands of
  characters long, and must not be stored as repository data.
- Paste tokens are session-scoped; verify the returned objects and inventory
  rather than assuming names remain unique.
- This validation used one disposable sequence and whole-folder cleanup after
  the complete batch.
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
shar-unreal-mcp describe animation_toolset.toolsets.sequencer.SequencerTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `parent_folder`

- Required: **no**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `paste_token`

- Required: **yes**
- Type: `string`
- Purpose:

Token from copy_tracks, or empty string.

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `target_bindings`

- Required: **yes**
- Type: `array<object>`
- Purpose:

Bindings to paste the tracks onto. Pass an empty list to paste as sequence-
level tracks.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.sequencer.SequencerTools \
  animation_toolset.toolsets.sequencer.SequencerTools.paste_tracks \
  --arguments '
{
  "paste_token": "<value>",
  "sequence": {},
  "target_bindings": []
}
'
```

## Expected output

List of newly-created tracks.

### `returnValue`

- Required: **yes**
- Type: `array<object>`
- Purpose:

`returnValue` has no prose; confirm its meaning with `describe`.

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
