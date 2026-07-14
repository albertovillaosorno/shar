# Faithful material normalization

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Material rendering

## Context

Source materials express observable layering, blending, texture, and shading
relationships that must survive normalization. Preset-specific ad hoc materials
would duplicate behavior and allow graphics quality levels to drift.

## Decision

Material translation preserves observable layering, blend, texture, and shading
behavior through repository-owned native material rules shared by all supported
graphics presets.

## Consequences

- Material translation preserves layering, blend, texture, and shading semantics
  through one repository-owned native rule set.
- Low through Ultra can vary visual fidelity without creating competing
  gameplay or material identities.
- Unsupported layering or blend evidence fails instead of being flattened
  silently.

## Rejected alternatives

- Baking layered behavior into flattened textures without equivalent evidence.
- Maintaining unrelated material logic for each graphics preset.
- Treating manual shader adjustment as the authoritative conversion process.
