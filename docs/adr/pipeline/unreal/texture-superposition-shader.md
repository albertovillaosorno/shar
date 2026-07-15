# Faithful material normalization

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Material rendering

## Context

Source materials express observable layering, blending, texture, and shading
relationships that must survive normalization. Preset-specific ad hoc materials
would duplicate behavior and allow graphics quality levels to drift.

## Decision

Material translation consumes canonical semantic-region and modern texture
evidence prepared during FBX conversion. Neutral base-color textures preserve
intended pigment relationships without baking one campaign level's illumination
into the asset. Native material rules then preserve observable layering, blend,
texture, and shading behavior across all supported graphics presets and respond
to the active environment lighting and time-of-day state.

Authored shading maps are bound when validated. Derived normal, specular,
roughness, metallic, glossiness, emissive, or ambient-occlusion evidence is used
only when a versioned deterministic recipe owns its generation and verification.

## Consequences

- Semantic base-color regions remain stable and independently modifiable.
- Campaign and dynamic lighting alter environmental appearance without requiring
  a separate baked texture set for each time of day.
- Low through Ultra can vary visual fidelity without creating competing
  gameplay, texture, or material identities.
- Unsupported layering, color-space, map-role, or blend evidence fails instead
  of being flattened or guessed silently.

## Rejected alternatives

- Baking layered behavior into flattened textures without equivalent evidence.
- Maintaining unrelated material logic for each graphics preset.
- Treating manual shader adjustment as the authoritative conversion process.
