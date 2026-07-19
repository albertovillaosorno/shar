# Materials, textures, UVs, and shaders

- Status: Active
- Last reviewed: 2026-07-18

## Core decision

UV coordinates are embedded mesh attributes. They are never supplied as a
detached runtime asset. Semantic regions, material roles, texture roles, and
rebake evidence are typed manifest data that references mesh sections and UV
channels.

Production FBX files reference external normalized textures. Textures are not
embedded in FBX. Final Unreal materials reference final `UTexture` assets
through validated Material Instances.

## UV channel contract

<!-- markdownlint-disable MD013 -->
| Channel | Skeletal mesh | Static mesh | Contract |
| :--- | :--- | :--- | :--- |
| `UV0` | Required | Required | Primary material coordinates; non-overlapping only when the recipe requires it. |
| `UV1` | Optional | Required for baked-light fallback; otherwise reserved | Static lightmap coordinates when enabled. |
| `UV2` | Optional | Optional | Registered detail, trim, decal, or terrain blend coordinates. |
| `UV3` | Optional | Optional | Registered specialized profile only. |
| `UV4+` | Rejected by default | Rejected by default | Requires a named validation profile and runtime consumer. |
<!-- markdownlint-enable MD013 -->

Each section declares its material semantic. Material slot order is validated
but runtime code resolves semantics from the presentation definition, not a
numeric slot guessed from import order.

## Normalized texture formats

- 8-bit base color, masks, decals, UI, and ordinary emissive inputs: lossless
  PNG;
- tangent-space normals requiring full channel precision: lossless PNG or TGA;
- high-dynamic-range emission, sky, reflection, and lighting masters: OpenEXR;
- source JPEG, DDS, PSD, proprietary image containers, and editor screenshots
  are not accepted normalized production inputs;
- all inputs declare dimensions, channel roles, alpha meaning, color space, and
  SHA-256 digest in the plan.

## Texture role suffixes

| Suffix | Role | Color space | Alpha |
| :--- | :--- | :--- | :--- |
| `_BC` | Base color | sRGB | opacity only when declared |
| `_N` | Tangent-space normal | Linear | unused or declared auxiliary |
| `_ORM` | R=ambient occlusion, G=roughness, B=metallic | Linear | unused |
| `_E` | Emissive color or intensity | Declared sRGB or linear | optional mask |
| `_M` | Single-purpose mask | Linear | role-specific |
| `_LUT` | Color lookup table | Linear | format-specific |
| `_UI` | User-interface color | sRGB | transparency |

A texture cannot serve incompatible color-space roles. Packed channels require
the exact `_ORM` contract or a versioned profile. Glossiness is converted
deterministically to roughness during normalization; runtime does not invert
channels dynamically.

## Resolution profiles

Dimensions are powers of two for streamed world and 3D textures unless a
declared UI or LUT profile requires otherwise. Maximum source dimensions are
strict upper bounds, not mandatory allocations.

<!-- markdownlint-disable MD013 -->
| Profile | Base color maximum | Normal or ORM maximum | Typical use |
| :--- | :--- | :--- | :--- |
| `hero_4k` | 4096 | 4096 | Exceptional close-up character, vehicle, or landmark |
| `hero_2k` | 2048 | 2048 | Playable character, principal vehicle, major prop |
| `standard_2k` | 2048 | 2048 | Reusable world module or visible prop |
| `standard_1k` | 1024 | 1024 | Ambient character, minor vehicle, medium prop |
| `small_512` | 512 | 512 | Small prop, eye layer, decal, icon source |
| `micro_256` | 256 | 256 | Tiny mask, lookup, distant or low-detail asset |
<!-- markdownlint-enable MD013 -->

The default character body profile is `hero_2k`; eyes and small facial layers
are `small_512` or `standard_1k`. Vehicle exteriors default to `hero_2k`. Unique
major landmarks may use `hero_4k` only when texel-density evidence and memory
budgets justify it. Repeated world surfaces prefer tiling or trim materials
rather than unique 4K textures.

## Texel density defaults

- playable or close-up character: 1024 pixels per meter effective visible
  density;
- principal vehicle exterior: 512 pixels per meter;
- ordinary world module: 256 pixels per meter;
- background or large tiling surface: 128 pixels per meter plus tiling detail;
- UI uses target display pixels and DPI scaling rather than world density.

Every import plan declares the selected profile, measured density, and allowed
variance. Arbitrary per-asset density is invalid.

## Master materials

The project owns a small stable master-material library:

- `M_SHAR_Character`;
- `M_SHAR_Eye`;
- `M_SHAR_VehiclePaint`;
- `M_SHAR_VehicleGlass`;
- `M_SHAR_WorldOpaque`;
- `M_SHAR_WorldMasked`;
- `M_SHAR_Decal`;
- `M_SHAR_UI`;
- `M_SHAR_VFX`.

Imported assets create Material Instances, not unique generated master
materials. Static switches are restricted to declared feature sets. Scalar,
vector, and texture parameters use registered names and units. A new shading
model requires an ADR and a new validation profile.

## Stylized rendering

The stylized appearance is implemented through authored shape, color, material
response, edge treatment, lighting, and post-process policy. It does not require
metallic defaults, uncontrolled specular highlights, or unlit meshes. Every
opaque surface participates correctly in lighting unless its registered material
profile intentionally uses an unlit or emissive model.

## Mips, streaming, and compression

Final `UTexture` assets generate deterministic mip chains, use texture groups by
family, and participate in streaming unless the profile explicitly requires full
residency. Compression is selected by semantic role and target platform. Normal
maps use normal-map compression. Masks remain linear. UI and tiny lookup assets
may be non-streaming only when budgeted.

## Validation

Publication rejects missing required textures, wrong color space, unsupported
resolution, invalid alpha use, inconsistent material semantics, NaN UVs, out-of-
range UV indices, undeclared channels, material-slot drift, unresolved master
material parameters, or a final asset that still references staging textures.
