# Pure3D P3D

This non-governing record documents one interoperability subject without
granting rights in proprietary code, tools, documentation, game data, assets,
names, marks, or user-supplied content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Fail-closed chunk framing, hierarchy,
  package metadata, and normalized P3D consumption are verified in repository
  code and tests; the universal chunk registry, version matrix, title-specific
  extensions, and present rights authority remain unresolved.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local interoperability research only.
- Subject class: Proprietary chunked graphics, animation, scene, and game-asset
  container.

## Covered Material

The P3D container family associated with Pure3D, including chunk framing,
hierarchy, names, images, meshes, rigs, animations, collision, intersection,
scene, and related asset records needed by SHAR.

## Repository Use And Scope

The src/p3d crate owns bounded binary parsing. The src/fbx and pipeline surfaces
consume normalized results and produce independently authored JSON, PNG, and FBX
artifacts. The repository does not distribute original P3D files.

## Provenance And Version History

Reviewed historical headers identify Pure3D and Radical entities, but the exact
version matrix, per-title extensions, chunk ownership, and present rights
authority remain unresolved. Chunk IDs observed in one title must not be
represented as universal across all Pure3D products.

## Authorship, Ownership, And Attribution

Historical developers, contributors, publishers, licensors, and any successors
retain applicable rights in upstream code, documentation, tools, marks, and
protected expression. SHAR claims rights only in independently authored
repository material to the extent supported by authorship evidence and law.

## License Or Terms Basis

No standalone public specification license or redistribution grant for this
proprietary subject has been verified. The SHAR MIT License applies only to
material the repository owner has authority to license and does not absorb
upstream expression, assets, marks, patents, trade secrets, or contracts.

## Distribution, Modification, And Compatibility

Independently observed functional facts may support compatibility work, but
successful parsing does not authorize distribution of the input, extracted
content, historical tools, or copied documentation. Copyright, contract, anti-
circumvention, trademark, patent, trade-secret, and jurisdiction questions
require separate fact-specific analysis in docs/legal.

## Compliance Posture

- Use only user-supplied local input obtained on a documented lawful basis.
- Keep original and extracted proprietary payloads outside Git and distributed
  artifacts.
- Use synthetic or independently authored fixtures for tracked regression tests.
- Preserve private hashes, acquisition dates, and version evidence without
  publishing local routes.
- Do not infer ownership, authorization, or redistribution rights from
  successful decoding.
- Record platform, title, package hash, root chunks, and unsupported chunk IDs
  for each sample.
- Keep decoded semantic models distinct from copied historical structure
  definitions.

## Source References

- [Radical Entertainment historical toolchain provenance
  record](radical-entertainment-toolchain-and-formats.md).
- Historical Radical source notices reviewed locally; source material not
  distributed.
- SHAR repository (2026) src/p3d, src/fbx, and P3D architecture records.
