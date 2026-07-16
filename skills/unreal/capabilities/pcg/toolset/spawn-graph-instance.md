# Spawn graph instance

[Return to the central Unreal MCP index](../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Effects, physics, and procedural
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
PCGToolset.PCGToolset.SpawnGraphInstance
```

Toolset:

```text
PCGToolset.PCGToolset
```

## What this tool does

Spawns a PCG Volume with associated Graph Instance into the scene, optionally
with Graph Param overrides.

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
[TODO]
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
[TODO]
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
[FILL_ME]
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
[TODO]
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
[TODO]
<!-- END MANUAL FIELD: known-caveats -->

### Manual guidance reviewed revision

<!-- BEGIN MANUAL FIELD: manual-review-revision -->
[REVIEW_REQUIRED]
<!-- END MANUAL FIELD: manual-review-revision -->

<!-- markdownlint-disable-next-line MD013 -->
- Current revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual guidance status: **Review required**

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

### `graph`

- Required: **yes**
- Type: `object`
- Purpose:

The PCG graph to spawn an instance of

### `jsonParams`

- Required: **yes**
- Type: `string`
- Purpose:

(Optional) JSON string representing JsonObject for the params to set. MUST be
in format: {{"property_1_name": "property_1_value"}, ...} The default values
for the graph params will be used if not set.

### `name`

- Required: **yes**
- Type: `string`
- Purpose:

The name of the PCGVolume actor to spawn in the scene.

### `transform`

- Required: **yes**
- Type: `object`
- Purpose:

The transform to use for the new PCGVolume actor. Place at the origin unless
there is a reason not to and use default scale3D of {"x": 25,"y": 25,"z": 10}

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  PCGToolset.PCGToolset \
  PCGToolset.PCGToolset.SpawnGraphInstance \
  --arguments '
{
  "graph": {},
  "jsonParams": "<value>",
  "name": "<value>",
  "transform": {}
}
'
```

## Expected output

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

UStruct with information of the created actor and the corresponding graph
instance

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
