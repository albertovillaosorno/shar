# Rust

This non-governing record distinguishes the Rust language and official toolchain
ecosystem from SHAR source code and from third-party crates.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository Rust use, the dated
  `nightly-2026-07-10` toolchain pin, official project identity, compiler
  source,
  and licensing posture were verified. The resolved compiler and Cargo builds,
  standard-library payload, target, installed components, and dependency graph
  remain build-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
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

## Provenance And Version History

The repository pins the rustup channel `nightly-2026-07-10`. That dated channel
is the source-level toolchain contract; it is not a permanent claim about the
newest nightly or stable Rust release.

The pin does not identify the resolved compiler commit, Cargo build, standard
library payload, installed components, host, or compilation target. Those
identities remain time-bounded build evidence. Toolchain configuration,
lockfiles, command output, and build or distribution records establish the
complete identity for a particular build.

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

Cargo manifests, lock evidence, and authored source confirm Rust use. Maintain
an exact dependency and toolchain inventory for every published or distributed
artifact, and do not infer license compatibility from the language name alone.

## Source References

- Rust Project (n.d.) *Licenses*. Available at:
  <https://www.rust-lang.org/policies/licenses> (Accessed: 14 July 2026).
- Rust Project (n.d.) *The Rust Programming Language*. Available at:
  <https://www.rust-lang.org/> (Accessed: 14 July 2026).
- Rust Project (n.d.) *Rust compiler official GitHub repository*. Available at:
  <https://github.com/rust-lang/rust> (Accessed: 14 July 2026).
- SHAR repository (2026) Cargo manifests, lockfile, Rust source, and toolchain
  configuration pinning `nightly-2026-07-10`.
