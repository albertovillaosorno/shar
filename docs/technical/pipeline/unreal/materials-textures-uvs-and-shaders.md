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

- **Channel:** `UV0`
  - **Skeletal mesh:** Required
  - **Static mesh:** Required
  - **Contract:** Primary material coordinates; non-overlapping only when the
    recipe requires it.
- **Channel:** `UV1`
  - **Skeletal mesh:** Optional
  - **Static mesh:** Required for baked-light fallback; otherwise reserved
  - **Contract:** Static lightmap coordinates when enabled.
- **Channel:** `UV2`
  - **Skeletal mesh:** Optional
  - **Static mesh:** Optional
  - **Contract:** Registered detail, trim, decal, or terrain blend coordinates.
- **Channel:** `UV3`
  - **Skeletal mesh:** Optional
  - **Static mesh:** Optional
  - **Contract:** Registered specialized profile only.
- **Channel:** `UV4+`
  - **Skeletal mesh:** Rejected by default
  - **Static mesh:** Rejected by default
  - **Contract:** Requires a named validation profile and runtime consumer.

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

- **Profile:** `hero_4k`
  - **Base color maximum:** 4096
  - **Normal or ORM maximum:** 4096
  - **Typical use:** Exceptional close-up character, vehicle, or landmark
- **Profile:** `hero_2k`
  - **Base color maximum:** 2048
  - **Normal or ORM maximum:** 2048
  - **Typical use:** Playable character, principal vehicle, major prop
- **Profile:** `standard_2k`
  - **Base color maximum:** 2048
  - **Normal or ORM maximum:** 2048
  - **Typical use:** Reusable world module or visible prop
- **Profile:** `standard_1k`
  - **Base color maximum:** 1024
  - **Normal or ORM maximum:** 1024
  - **Typical use:** Ambient character, minor vehicle, medium prop
- **Profile:** `small_512`
  - **Base color maximum:** 512
  - **Normal or ORM maximum:** 512
  - **Typical use:** Small prop, eye layer, decal, icon source
- **Profile:** `micro_256`
  - **Base color maximum:** 256
  - **Normal or ORM maximum:** 256
  - **Typical use:** Tiny mask, lookup, distant or low-detail asset

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

Regular world surfaces preserve the original renderer's `PddiCullNone` state.
`M_SHAR_WorldOpaque` and `M_SHAR_WorldMasked` therefore render both polygon
faces. This is a raster-state translation, not permission to duplicate faces,
reverse authored winding, or repair source normals. Shadow and special-purpose
passes keep their independently validated native culling policy.

The `shar.world-package-collection.v8` artifact records both exact base
`material_bindings` and exact `material_slots` emitted by the binary writer for
each world FBX. Every binding row carries an independent `binding_sha256`
for its runtime-addressable source shader target and a separate
`presentation_sha256` for its initial visual state, plus canonical PNG identity,
decoded diffuse RGBA8 tint, and six overlapping surface-semantic flags.

A base binding may fan out to multiple FBX slots when geometry identity adds
effective surface semantics. Each `material_slots` row therefore carries the
exact `slot_name`, `source_material_name`, base binding and presentation hashes,
an effective `slot_presentation_sha256`, and the effective semantic flags.
Unreal projection must address the exact slot, join source state through
`source_material_name`, and may deduplicate native Material Instances only by
the effective presentation hash without merging runtime targets. It must not
infer shader meaning from slot names or reparse source P3D packages.

Normalized world diffuse PNGs cross the native editor boundary as base-color
`Texture2D` assets only. The texture importer does not create Materials, choose
blend policy, or infer special surface behavior. Those decisions belong to the
catalog-driven master-material and Material Instance projection after byte and
identity verification.

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
maps use normal-map compression. Masks remain linear.

UI and tiny lookup assets
may be non-streaming only when budgeted.

## Validation

Publication rejects missing required textures, wrong color space, unsupported
resolution, invalid alpha use, inconsistent material semantics, NaN UVs, out-of-
range UV indices, undeclared channels, material-slot drift, unresolved master
material parameters, or a final asset that still references staging textures.
