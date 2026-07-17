# Copy bindings

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Review required**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.sequencer.SequencerTools.copy_bindings
```

Toolset:

```text
animation_toolset.toolsets.sequencer.SequencerTools
```

## What this tool does

Copy one or more bindings to the Sequencer clipboard.

Returns a paste token that can be passed to paste_bindings, or an empty string
to consume from the clipboard. The token also lands in the editor clipboard for
interactive use.

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
Use this tool to copy bindings reviewed folders, bindings, tracks, or sections
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
- Bound the copied object set explicitly; the returned token can contain a
  large Unreal object-text payload.
- Define an exact inverse or whole disposable-asset cleanup before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "bindings": [
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
The disposable SHAR sequence returned a non-empty opaque Unreal object-text
token containing 13361 characters. The copied object set was explicit, and the
token was consumed successfully by the matching paste operation in the same
editor session.
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

### `bindings`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The bindings to copy.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.sequencer.SequencerTools \
  animation_toolset.toolsets.sequencer.SequencerTools.copy_bindings \
  --arguments '
{
  "bindings": []
}
'
```

## Expected output

The paste token string.

### `returnValue`

- Required: **yes**
- Type: `string`
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
