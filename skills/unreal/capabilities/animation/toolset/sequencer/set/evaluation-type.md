# Set evaluation type

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.sequencer.SequencerTools.set_evaluation_type
```

Toolset:

```text
animation_toolset.toolsets.sequencer.SequencerTools
```

## What this tool does

Set the evaluation type of a sequence.

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
Use this tool to set evaluation type for a reviewed SHAR mission, dialogue,
camera, or cinematic Level Sequence.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live SequencerTools schema.
- Open the exact disposable or recoverable Level Sequence and capture the
  independent reader state before mutation.
- Define an exact inverse or whole disposable-asset cleanup before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "eval_type": "WITH_SUB_FRAMES",
  "sequence": {
    "refPath": "/Game/SHAR_MCP_Validation_Batch50_555102d0/LS_MCP_Batch50_555102d0.LS_MCP_Batch50_555102d0"
  }
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
In the disposable SHAR sequence, `get_evaluation_type` changed from
<MovieSceneEvaluationType.WITH_SUB_FRAMES: 1> to
<MovieSceneEvaluationType.FRAME_LOCKED: 0> after the call. The result was read
independently from the mutation response before cleanup.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Sequence, binding, folder, track, and section references are live editor
  identities and can become stale after structural edits or closing Sequencer.
- Enum strings are interpreted by the live plugin revision; always read the
  resulting enum back instead of inferring it from the submitted text.
- In this revision, submitting `WITH_SUB_FRAMES` produced a `FRAME_LOCKED`
  read-back; treat that mismatch as a live compatibility caveat.
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

### `eval_type`

- Required: **yes**
- Type: `string`
- Purpose:

Evaluation type string (e.g. 'WITH_SUB_FRAMES', 'FRAME_LOCKED').

### `sequence`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.sequencer.SequencerTools \
  animation_toolset.toolsets.sequencer.SequencerTools.set_evaluation_type \
  --arguments '
{
  "eval_type": "<value>",
  "sequence": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
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
