# Rust

This non-governing record distinguishes the Rust language and official toolchain
ecosystem from SHAR source code and from third-party crates.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository Rust use, the governed stable
  and nightly channels, resolved compiler and Cargo identities, required
  components, official project identity, source repositories, and licensing
  posture were verified. The standard-library payload, target-specific output,
  and dependency graph remain build-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-15.
- Subject class: Programming language and official toolchain ecosystem.

## Covered Material

The Rust language, compiler toolchain, rustup toolchain manager, Cargo package
and build tool, standard library, official components, and project documentation
relevant to SHAR.

## Repository Use And Scope

Rust is the primary implementation language for deterministic parsers,
extractors, manifests, conversion tooling, validation components, and pipeline
orchestration. SHAR's authored Rust source is independently licensed under the
repository license. The language, compiler, libraries, tools, and packages
retain their respective upstream terms.

The root dependency authority installs exact stable Rust 1.97.0 for default
operator commands and reconciles a separate component-complete floating nightly.
The tracked SHAR validation manifest selects `nightly`; it does not select or
publish a dated nightly override.

## Provenance And Version History

The reviewed default toolchain reported rustc 1.97.0, commit `2d8144b78`, dated
7 July 2026, and Cargo 1.97.0, commit `c980f4866`, dated 30 June 2026. Official
Rust release notes identify Rust 1.97.0 as released on 9 July 2026.

The reviewed SHAR nightly reported rustc 1.99.0-nightly, commit `da80ed070`,
dated 14 July 2026, and Cargo 1.99.0-nightly, commit `59800466c`, dated 7 July
2026. Those exact values are dated execution evidence, not permanent version
requirements or a claim that a floating channel is reproducible by name alone.

Rustup documents that stable releases follow the release train, nightly builds
are produced nightly, optional components may be unavailable, and a requested
component set can resolve to an older component-complete nightly. Validation
records must therefore preserve the actual compiler commit, Cargo build, host,
target, components, configuration, and dependency graph for each run.

## Authorship, Ownership, And Attribution

The Rust Project and contributors retain applicable rights in official Rust
material. Each third-party crate has its own authorship, ownership, license,
notice, and dependency history.

## License Or Terms Basis

Official Rust projects are generally offered under a dual Apache License 2.0 and
MIT posture, subject to the exact license files in each upstream repository.
That summary does not license every crate, compiler distribution, documentation
page, bundled component, or generated binary. Package-level evidence controls.

## Distribution, Modification, And Compatibility

Publishing Rust source does not necessarily redistribute the compiler.
Published source and distributed binaries must inventory direct and transitive
dependencies, preserve required notices, and satisfy any source, attribution,
patent, or reciprocal conditions attached to the components actually delivered.

## Compliance Posture

- Keep the exact stable toolchain and the component-complete floating nightly
  independently governed.
- Preserve the resolved compiler, Cargo, host, target, component, and lockfile
  identities for every validation or distribution record.
- Do not convert one observed version into a permanent minimum or unbounded
  compatibility range.
- Do not infer crate-license compatibility from the Rust language name.

## Source References

- Rust Project (2026) *Rust Release Notes: Version 1.97.0*. Identifies release
  1.97.0, compiler commit `2d8144b78`, and the 9 July 2026 release date.
  Available at:
  <https://doc.rust-lang.org/stable/releases.html#version-1970-2026-07-09>
  (Accessed: 15 July 2026).
- Rust Project (n.d.) *Rustup channels*. Documents stable, beta, nightly,
  version-specific toolchains, updates, and optional-component availability.
  Available at: <https://rust-lang.github.io/rustup/concepts/channels.html>
  (Accessed: 15 July 2026).
- Rust Project (n.d.) *Licenses*. Available at:
  <https://www.rust-lang.org/policies/licenses> (Accessed: 15 July 2026).
- Rust Project (n.d.) *Rust compiler official GitHub repository*. Available at:
  <https://github.com/rust-lang/rust> (Accessed: 15 July 2026).
- SHAR repository and managed command authority (2026), Cargo manifests,
  lockfile, Rust source, validation manifest selecting `nightly`, exact stable
  1.97.0 bootstrap authority, and managed stable and nightly executable output.
