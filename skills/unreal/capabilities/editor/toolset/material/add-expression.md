# Add expression

[Return to the central Unreal MCP index](../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.material.MaterialTools.add_expression
```

Toolset:

```text
editor_toolset.toolsets.material.MaterialTools
```

## What this tool does

Adds a new expression node to a Material or MaterialFunction graph.

Use list_expression_classes to discover available types.

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
Use this tool to add an exact reflected expression class to a reviewed SHAR
material or material function.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Require `shar-unreal-mcp doctor` to report `ready: true` and refresh the
  live MaterialTools or MaterialInstanceTools schema.
- Use disposable or explicitly task-owned assets and capture the matching
  asset, graph, parameter, or property reader before mutation.
- Resolve expression classes, pin names, output names, and nested expression
  references from current MaterialTools readers.
- Define whole-folder asset cleanup before invocation.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "expression_class": {
    "refPath": "/Script/Engine.MaterialExpressionConstant3Vector"
  },
  "material_or_function": {
    "refPath": "/Game/SHAR_MCP_Validation_Material_07b38dea/M_MCP_07b38dea.M_MCP_07b38dea"
  },
  "x": 0,
  "y": 0
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`get_expressions` changed from an empty list to seven exact nested references:
two Constant3Vector nodes, Add, scalar, vector, texture, and static-switch
parameter expressions.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Material, function, collection, expression, and instance references are live
  editor identities and become stale after deletion or whole-folder cleanup.
- Discover classes with `list_expression_classes`; class names alone do not
  prove compatibility with the target material or function.
- The reproduced lifecycle used one disposable content folder and removed
  every created asset after verification.
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
shar-unreal-mcp describe editor_toolset.toolsets.material.MaterialTools
```

1. Confirm every required input against the current schema.
1. Capture pre-state and define an independent postcondition check.

## Inputs

### `expression_class`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `material_or_function`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `x`

- Required: **no**
- Type: `integer`
- Default: `0`
- Purpose:

Horizontal position in the graph editor.

### `y`

- Required: **no**
- Type: `integer`
- Default: `0`
- Purpose:

Vertical position in the graph editor.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.material.MaterialTools \
  editor_toolset.toolsets.material.MaterialTools.add_expression \
  --arguments '
{
  "expression_class": {},
  "material_or_function": {}
}
'
```

## Expected output

The newly created MaterialExpression node.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

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
