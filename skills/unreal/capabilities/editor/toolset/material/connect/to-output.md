# Connect to output

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Persistent mutation likely**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.material.MaterialTools.connect_to_output
```

Toolset:

```text
editor_toolset.toolsets.material.MaterialTools
```

## What this tool does

Connects an expression node's output to one of the material's output
properties.

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
Use this tool to connect one exact expression output to a reviewed material
property.
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
  "expression": {
    "refPath": "/Game/SHAR_MCP_Validation_Material_07b38dea/M_MCP_07b38dea.M_MCP_07b38dea:MaterialExpressionAdd_0"
  },
  "material_property": "MP_BaseColor",
  "output_name": ""
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
`get_property_input` for `MP_BaseColor` changed from expression `None` to the
exact Add expression reference and empty default output name.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Material, function, collection, expression, and instance references are live
  editor identities and become stale after deletion or whole-folder cleanup.
- An unconnected material property is reported with expression `None`; an
  empty output name means the expression default output.
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

### `expression`

- Required: **yes**
- Type: `object`
- Purpose:

Represents a reference to a UObject or UClass.

### `material_property`

- Required: **yes**
- Type: `string`
- Allowed values:

  - `"MP_EmissiveColor"`
  - `"MP_Opacity"`
  - `"MP_OpacityMask"`
  - `"MP_DiffuseColor"`
  - `"MP_SpecularColor"`
  - `"MP_BaseColor"`
  - `"MP_Metallic"`
  - `"MP_Specular"`
  - `"MP_Roughness"`
  - `"MP_Anisotropy"`
  - `"MP_Normal"`
  - `"MP_Tangent"`
  - `"MP_WorldPositionOffset"`
  - `"MP_WorldDisplacement_DEPRECATED"`
  - `"MP_TessellationMultiplier_DEPRECATED"`
  - `"MP_SubsurfaceColor"`
  - `"MP_CustomData0"`
  - `"MP_CustomData1"`
  - `"MP_AmbientOcclusion"`
  - `"MP_Refraction"`
  - `"MP_CustomizedUVs0"`
  - `"MP_CustomizedUVs1"`
  - `"MP_CustomizedUVs2"`
  - `"MP_CustomizedUVs3"`
  - `"MP_CustomizedUVs4"`
  - `"MP_CustomizedUVs5"`
  - `"MP_CustomizedUVs6"`
  - `"MP_CustomizedUVs7"`
  - `"MP_PixelDepthOffset"`
  - `"MP_ShadingModel"`
  - `"MP_FrontMaterial"`
  - `"MP_SurfaceThickness"`
  - `"MP_Displacement"`
  - `"MP_MaterialAttributes"`
  - `"MP_CustomOutput"`
  - `"MP_LastCustomizedUVs"`
  - `"MP_NumCustomizedUVs"`
- Purpose:

EMaterialProperty

### `output_name`

- Required: **yes**
- Type: `string`
- Purpose:

The output pin name. Pass an empty string to use the default (first) output.

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.material.MaterialTools \
  editor_toolset.toolsets.material.MaterialTools.connect_to_output \
  --arguments '
{
  "expression": {},
  "material_property": "MP_EmissiveColor",
  "output_name": "<value>"
}
'
```

## Expected output

The live interface does not declare a structured output schema.

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
