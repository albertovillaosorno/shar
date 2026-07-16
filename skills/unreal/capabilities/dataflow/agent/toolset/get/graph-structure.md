# Get graph structure

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
DataflowAgent.DataflowAgentToolset.GetGraphStructure
```

Toolset:

```text
DataflowAgent.DataflowAgentToolset
```

## What this tool does

Returns the complete structure of a Dataflow graph including all nodes and
connections.

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
Use this tool to inspect a Dataflow asset's node inventory, positions, pin
types, GUIDs, and explicit connections during SHAR graph review.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact indexed `/Script/DataflowEngine.Dataflow` object path.
- Parse the returned JSON string.
- Treat node GUIDs and names as graph-local identities.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "graph": {"refPath": "/GeometryCollectionPlugin/DataflowTemplates/DF_GC_Template_Source.DF_GC_Template_Source"}
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Two calls returned graph `DF_GC_Template_Source` with three nodes and five
connections. The nodes were Create Geometry Collection From Sources, Uniform
Fracture, and Geometry Collection Terminal, with stable GUIDs, positions,
input/output pin names, and types. The Groom empty template independently
returned four nodes and four connections.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- The return value is JSON text and requires a second parse.
- Structure contains authored nodes and links, not evaluated runtime data.
- Node GUIDs, names, and positions can change after graph edits or duplication.
- Pin GUIDs are separate from human-readable pin names.
- Missing graph refs fail during parameter translation.
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
shar-unreal-mcp describe DataflowAgent.DataflowAgentToolset
```

1. Confirm every required input against the current schema.

## Inputs

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

The Dataflow asset to inspect

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  DataflowAgent.DataflowAgentToolset \
  DataflowAgent.DataflowAgentToolset.GetGraphStructure \
  --arguments '
{
  "graph": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `string`
- Purpose:

JSON string with graph structure (nodes + connections), or error message on
failure

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
