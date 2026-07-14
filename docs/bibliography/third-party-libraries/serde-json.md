# Serde JSON

This non-governing record documents a direct Rust dependency and does not apply
Serde JSON licensing to SHAR-authored schemas, manifests, or data.

## Review Status And Scope

- Review status: Verified.
- Evidence status: Verified — Exact resolved version, transitive graph, and
  license expressions confirmed from the repository lockfile and official
  registry metadata.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Open-source JSON library for Rust.

## Covered Material

The `serde_json` crate and its exact resolved package graph.

## Repository Use And Scope

`serde_json` is a direct dependency of the FBX, P3D, and pipeline crates. It is
used to parse, validate, or emit structured JSON documents at repository-owned
boundaries. It does not own or license the schemas, manifests, package indexes,
or factual data processed by those crates.

## Provenance And Version History

The `src/fbx`, `src/p3d`, and `src/pipeline` manifests request major version 1.
`Cargo.lock` resolves `serde_json` to exactly 1.0.150 (published 2026-05-21).
Its resolved transitive graph is `itoa` 1.0.18, `memchr` 2.8.3, `serde`
1.0.228, `serde_core` 1.0.228, and `zmij` 1.0.21. The exact versions and
checksums remain controlled by `Cargo.lock` and must be preserved with release
evidence.

## Authorship, Ownership, And Attribution

Serde JSON contributors retain applicable upstream rights. SHAR contributors
retain rights in independently authored repository code and schemas subject to
the repository license.

## License Or Terms Basis

The crates.io metadata for `serde_json` 1.0.150 states `MIT OR Apache-2.0`, so
a redistributor may satisfy either license at its option. The resolved
transitive dependencies are not uniformly dual-licensed: `serde` 1.0.228,
`serde_core` 1.0.228, and `itoa` 1.0.18 are `MIT OR Apache-2.0`; `memchr`
2.8.3 is `Unlicense OR MIT`; and `zmij` 1.0.21 is `MIT` only. Any release
notice inventory must therefore carry an MIT notice path for every component
and preserve the exact upstream license files.

## Distribution, Modification, And Compatibility

Using Serde JSON does not relicense JSON documents or SHAR source. Any
redistributed binary or source package must account for Serde JSON and its
transitive dependencies in the distribution notice inventory.

## Compliance Posture

Use is verified against the lockfile. Preserve exact versions, checksums,
license files, and dependency notices for each release, including the
`Unlicense OR MIT` posture of `memchr` and the `MIT`-only posture of `zmij`.

## Source References

- Serde contributors (n.d.) *Serde JSON documentation*. Available at:
  <https://docs.rs/serde_json/> (Accessed: 13 July 2026).
- Serde contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/serde-rs/json> (Accessed: 13 July 2026).
- crates.io (2026) *serde_json 1.0.150 registry metadata* (license
  `MIT OR Apache-2.0`; published 2026-05-21). Available at:
  <https://crates.io/api/v1/crates/serde_json/1.0.150> (Accessed: 13 July 2026).
- crates.io (2026) *memchr and zmij registry metadata*. Available at:
  <https://crates.io/api/v1/crates/memchr> and
  <https://crates.io/api/v1/crates/zmij> (Accessed: 13 July 2026).
- SHAR repository (2026) `src/fbx/Cargo.toml`, `src/p3d/Cargo.toml`,
  `src/pipeline/Cargo.toml`, and `Cargo.lock`.
