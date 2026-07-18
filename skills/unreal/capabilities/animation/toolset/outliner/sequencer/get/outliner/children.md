# Get outliner children

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_outliner_children
```

Toolset:

```text
animation_toolset.toolsets.outliner.SequencerOutlinerTools
```

## What this tool does

Get child nodes of an outliner node.

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
Use this tool to read child view models beneath one live Outliner node only when
the caller holds the native live view-model object inside the same Unreal
invocation boundary.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Open the intended Level Sequence and obtain a live Sequencer view-model node.
- Do not reconstruct a node from the public JSON `type` field.
- Require a same-process identity-preserving route before reading or mutating
  node state.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "node": {
    "type": "FPossessableModel"
  },
  "type_filter": ""
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
A real camera binding was selected in Sequencer. The MCP response retained only
`{"type": "FPossessableModel"}`. Passing that value to node readers produced an
empty label or `View Model is no longer valid`; the programmatic executor
serialized the node identically. The example therefore reproduces the
identity-loss boundary rather than a reusable node reference.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- `SequencerViewModelScriptingStruct` identity is not represented by the public
  schema.
- A serialized `{type: ...}` object cannot target the original native node.
- Do not retry through the programmatic executor; it uses the same serialization
  boundary.
- Read results from a reconstructed proxy are not evidence about the selected
  native node.
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
shar-unreal-mcp describe animation_toolset.toolsets.outliner.SequencerOutlinerTools
```

1. Confirm every required input against the current schema.

## Inputs

### `node`

- Required: **yes**
- Type: `object`
- Purpose:

SequencerViewModelScriptingStruct

### `type_filter`

- Required: **no**
- Type: `string`
- Purpose:

Optional node type name to filter by (e.g. "Track").

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.outliner.SequencerOutlinerTools \
  animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_outliner_children \
  --arguments '
{
  "node": {}
}
'
```

## Expected output

List of child SequencerViewModelScriptingStruct objects.

### `returnValue`

- Required: **yes**
- Type: `array<object>`
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
