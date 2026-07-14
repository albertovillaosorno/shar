# Serde

This non-governing record documents a direct Rust dependency and does not apply
Serde's license to independently authored SHAR source or serialized data.

## Review Status And Scope

- Review status: Verified.
- Evidence status: Verified — Exact resolved versions, feature selection, and
  license expression confirmed from the repository lockfile and official
  registry metadata.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Open-source Rust serialization framework.

## Covered Material

The `serde`, `serde_core`, and `serde_derive` package family used through the
workspace dependency graph.

## Repository Use And Scope

`src/fbx/Cargo.toml` declares Serde version 1 with the `derive` feature. SHAR
uses Serde traits and derives to express typed serialization boundaries. Serde
is a library dependency, not an author or owner of repository data, schemas, or
independently authored Rust types.

## Provenance And Version History

`src/fbx/Cargo.toml` requests major version 1 with the `derive` feature.
`Cargo.lock` resolves the family to `serde` 1.0.228, `serde_core` 1.0.228, and
`serde_derive` 1.0.228 (the 1.0.228 release was published 2025-09-27). The
`derive` path pulls in `proc-macro2` 1.0.106, `quote` 1.0.46, `syn` 2.0.118,
and `unicode-ident` 1.0.24. The exact versions and checksums remain controlled
by `Cargo.lock` and must be preserved with build or distribution evidence.

## Authorship, Ownership, And Attribution

Serde contributors retain applicable rights in the framework and derive
implementation. SHAR contributors retain rights in independently authored
repository code subject to the repository license.

## License Or Terms Basis

The crates.io metadata for `serde` 1.0.228 states `MIT OR Apache-2.0`, so a
redistributor may satisfy either license at its option; `serde_core`,
`serde_derive`, `proc-macro2`, and `quote` carry the same expression. One
derive-path transitive dependency is not plain dual-licensed: `unicode-ident`
1.0.24 is `(MIT OR Apache-2.0) AND Unicode-3.0`, which additionally requires
the Unicode license notice. The exact upstream license files and notices
control.

## Distribution, Modification, And Compatibility

Compiling against Serde does not relicense SHAR source or serialized output.
Source or binary distribution must preserve any notices required for the Serde
components and all other delivered dependencies.

## Compliance Posture

Serde use is verified against the lockfile. Preserve the exact resolved
versions, checksums, selected features, license files, and dependency inventory
for every published or distributed artifact, including the Unicode-3.0 notice
required by `unicode-ident`.

## Source References

- Serde contributors (n.d.) *Serde*. Available at: <https://serde.rs/>
  (Accessed: 13 July 2026).
- Serde contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/serde-rs/serde> (Accessed: 13 July 2026).
- crates.io (2026) *serde 1.0.228 registry metadata* (license
  `MIT OR Apache-2.0`; published 2025-09-27) and *unicode-ident registry
  metadata* (license `(MIT OR Apache-2.0) AND Unicode-3.0`). Available at:
  <https://crates.io/api/v1/crates/serde/1.0.228> and
  <https://crates.io/api/v1/crates/unicode-ident> (Accessed: 13 July 2026).
- SHAR repository (2026) `src/fbx/Cargo.toml` and `Cargo.lock`.
