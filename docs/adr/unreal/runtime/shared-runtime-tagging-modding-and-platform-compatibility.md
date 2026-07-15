# Shared runtime tagging, modding, and platform compatibility

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Runtime content identity across quality presets and platforms

## Context

Graphics presets, local mods, desktop builds, and Android builds all need to
address the same runtime content. Stable semantic identities are required to
prevent duplicated platform-specific gameplay data, incompatible mods, and
quality-level branches that drift from parity.

## Decision

Runtime content uses stable semantic tags and package contracts so every
supported graphics preset, local mod, platform family, and architecture shares
one gameplay identity model.

Platform adapters may select native rendering, input, packaging, filesystem,
and device-profile behavior, but they do not own separate mission, progression,
save, package, or mod identities. Graphics presets select visual quality only.

A mod targets semantic identities and declares any genuine platform capability
requirements explicitly. A package must not infer compatibility from a physical
asset location, filename, graphics preset, processor architecture, or operating
system path.

A community server mod uses the same semantic identities through a stable,
transport-neutral server-adapter contract. The base product does not supply a
multiplayer campaign, matchmaking, server browser, hosted network, moderation,
or server persistence. Those capabilities belong to the community package and
operator.

## Consequences

- `Low`, `Medium`, `High`, `Epic`, and `Ultra` share stable semantic tags and
  package identities.
- Windows, Linux, macOS, Android, x64, and ARM64 adapters consume the same
  gameplay and mod contracts.
- Platform adapters map shared data into native Unreal facilities instead of
  duplicating domain data.
- Android's forced `Low` policy does not create Android-specific gameplay or mod
  variants.
- A save produced on one supported platform uses the same logical schema as a
  save produced on another platform under the
  [portable save storage and lifecycle](portable-save-storage-and-lifecycle.md)
  decision; physical storage and account boundaries remain platform-specific.
- Platform-specific capabilities fail explicitly rather than silently changing
  package meaning.
- Community server packages declare protocol, authority, persistence,
  package-set, and platform capabilities explicitly.
- Base campaign saves and achievements remain separate from server-owned state.

## Rejected alternatives

- Duplicating gameplay data for each platform, architecture, or graphics preset.
- Using filenames, package locations, or local paths as public mod identities.
- Defining incompatible mod contracts for desktop, mobile, x64, or ARM64 builds.
- Allowing graphics presets or device profiles to change mission, progression,
  save, physics, or simulation semantics.
- Shipping an official multiplayer campaign or hosted server network as base
  product scope.
- Letting a server mod reinterpret local campaign saves or base achievements.
