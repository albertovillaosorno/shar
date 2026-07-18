# Set node expanded

[Return to the central Unreal MCP index](../../../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Animation and cinematics
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_node_expanded
```

Toolset:

```text
animation_toolset.toolsets.outliner.SequencerOutlinerTools
```

## What this tool does

Expand or collapse outliner nodes.

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
Use this tool to change the expanded state of live Outliner nodes only when the
caller holds the native live view-model object inside the same Unreal invocation
boundary.
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
  "nodes": [
    {
      "type": "FPossessableModel"
    }
  ],
  "expanded": true
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
- Mutation must remain blocked until a stable node identity or a same-call
  native composition route exists.
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
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `expanded`

- Required: **yes**
- Type: `boolean`
- Purpose:

True to expand, False to collapse.

### `nodes`

- Required: **yes**
- Type: `array<object>`
- Purpose:

The nodes to expand or collapse.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  animation_toolset.toolsets.outliner.SequencerOutlinerTools \
  animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_node_expanded \
  --arguments '
{
  "expanded": false,
  "nodes": []
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
