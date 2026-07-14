# Texture evidence

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Faithful material normalization](../../../adr/pipeline/unreal/texture-superposition-shader.md)

## Purpose

This specification explains how supported texture evidence is normalized and
bound before scene serialization.

## Repository model

Texture adapters validate decoded image evidence, create stable identities, and
associate textures with canonical materials. The scene writer receives validated
bytes and metadata only; temporary review exports are not an input authority.

## Invariants

- Texture identity is independent of a local storage route.
- Embedded bytes match validated texture evidence.
- Material bindings reference known canonical textures.

## Failure behavior

- Missing bytes, conflicting metadata, unsupported encoding, and ambiguous
  bindings reject the texture capability.

## Verification

- Evidence tests compare identifiers and content hashes.
- Binding tests cover shared and missing textures.
- Writer read-back verifies embedded resources and connections.
