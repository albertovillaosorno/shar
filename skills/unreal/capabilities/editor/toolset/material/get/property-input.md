# Get property input

[Return to the central Unreal MCP index](../../../../../index.md).

Generated from live MCP metadata; no engine source is copied.

- Domain: Assets and data
- Operational posture: **Expected read-only**
<!-- markdownlint-disable-next-line MD013 -->
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`

## Native identities

Tool:

```text
editor_toolset.toolsets.material.MaterialTools.get_property_input
```

Toolset:

```text
editor_toolset.toolsets.material.MaterialTools
```

## What this tool does

Returns the expression and output pin feeding a material output property.

Use to inspect what drives MP_EmissiveColor, MP_Opacity, MP_BaseColor, etc.

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
Use this tool to identify the expression driving one material output before SHAR
traces graph wiring or compares material parity.
<!-- END MANUAL FIELD: project-use-cases -->

### Project prerequisites

<!-- BEGIN MANUAL FIELD: project-prerequisites -->
- Use an exact loaded Material object path.
- Pass one live `EMaterialProperty` enum value from the schema.
- Verify the returned expression against the material expression inventory.
<!-- END MANUAL FIELD: project-prerequisites -->

### Validated argument example

<!-- BEGIN MANUAL FIELD: validated-arguments -->
```json
{
  "material": {"refPath": "/Engine/EngineMaterials/WorldGridMaterial.WorldGridMaterial"},
  "material_property": "MP_BaseColor"
}
```
<!-- END MANUAL FIELD: validated-arguments -->

### Project verification notes

<!-- BEGIN MANUAL FIELD: project-verification -->
Repeated reads mapped Base Color to `MaterialExpressionMultiply_30`, Roughness
to `MaterialExpressionClamp_6`, and Normal to `MaterialExpressionMultiply_31`.
Emissive Color and Opacity returned `expression: "None"`.
<!-- END MANUAL FIELD: project-verification -->

### Known project caveats

<!-- BEGIN MANUAL FIELD: known-caveats -->
- Disconnected outputs are serialized as the string `"None"`, not JSON `null`.
- `input_name` and `output_name` were empty for the verified material outputs.
- Invalid enum text fails JSON-to-struct conversion before invocation.
- The result reports graph wiring, not the final compiled value.
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

## Inputs

### `material`

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

## Invocation example

Replace placeholders with validated project values.

```text
shar-unreal-mcp call \
  editor_toolset.toolsets.material.MaterialTools \
  editor_toolset.toolsets.material.MaterialTools.get_property_input \
  --arguments '
{
  "material": {},
  "material_property": "MP_EmissiveColor"
}
'
```

## Expected output

A MaterialInputSource with input_name empty. Its expression field is None when
the output property is disconnected.

### `returnValue`

- Required: **yes**
- Type: `object`
- Purpose:

MaterialInputSource

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
