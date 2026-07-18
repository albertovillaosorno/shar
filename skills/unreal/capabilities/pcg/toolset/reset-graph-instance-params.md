# Reset graph instance params

[Return to the central Unreal MCP index](../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.ResetGraphInstanceParams
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Resets the given graph instance params back to the graph's default values.
Actor MUST have a graph instance.

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
Use this tool to return selected PCG instance parameters to graph defaults.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  exact live toolset schema before mutation.
- Use a unique disposable graph and define whole-folder cleanup before
  invocation.
- Use `GetGraphStructure`, `GetGraphSchema`, `GetNodeInfo`, or the matching
  instance reader as the independent postcondition.
- Capture the complete parameter schema or instance property bag and account
  for lower-camel serialization.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "pCGVolume": {
    "refPath": "/Temp/Untitled_1.Untitled_1:PersistentLevel.PCG_MCP_Volume_c297c180_hrxrsc188znr1_1555303606"
  },
  "paramNames": [
    "DensityScale"
  ]
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`GetGraphInstanceParams` changed again after reset, proving that
`DensityScale` returned to the graph-provided state.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- PCG graph, node, comment, and volume references are transient editor
  identities and become stale after deletion.
- The reproduced lifecycle used one disposable folder and removed both graphs,
  the Dataflow companion graph, and the PCG Volume afterward.
- Submitted parameter names are exposed through readers in lower camel case,
  while reset accepted the authored display name.
- Instance parameters are stored on a transient PCG Volume; remove the exact
  actor before deleting its graph.
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
shar-unreal-mcp describe PCGToolset.PCGToolset
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `pCGVolume`

- Required: **yes**
- Type: `object`
- Purpose:

The actor to reset graph instance params for.

### `paramNames`

- Required: **yes**
- Type: `array<string>`
- Purpose:

Names of the parameters to reset.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PCGToolset.PCGToolset \
  PCGToolset.PCGToolset.ResetGraphInstanceParams \
  --arguments '
{
  "pCGVolume": {},
  "paramNames": []
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `boolean`
- Purpose:

boolean representing success/failed

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
