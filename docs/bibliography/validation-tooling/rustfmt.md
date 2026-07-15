# rustfmt

This non-governing record documents rustfmt as canonical formatting tooling and
does not apply rustfmt licensing to formatted SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The repository's dated nightly pin,
  managed rustfmt 1.9.0-nightly component, compiler commit identity, invocation,
  formatting authority, official source repository, documentation, and licensing
  posture were verified. Edition interaction and formatted output remain
  run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source Rust source-code formatter.

## Covered Material

The rustfmt tool used through the repository's canonical formatting and
validation flow.

## Repository Use And Scope

rustfmt enforces canonical formatting for applicable Rust source under
repository-controlled configuration. Repository authority requires using the
canonical validator for final evidence rather than substituting ad hoc direct
formatter commands. rustfmt is not a runtime dependency of the generated game.

## Provenance And Version History

The repository pins `nightly-2026-07-10` and requires the managed runtime to
move
with that file. The reviewed runtime reported rustc 1.99.0-nightly and rustfmt
1.9.0-nightly from compiler commit `af3d95584`, dated 9 July 2026.

The dated nightly pin is the reproducibility authority. Rustup documents that
nightly releases are ordinarily produced every night, may omit optional
components, and can be installed by exact date. A validation record must
preserve
the actual compiler commit, component version, target, and configuration rather
than treating the floating `nightly` label as sufficient evidence.

## Authorship, Ownership, And Attribution

The Rust Project Developers and rustfmt contributors retain applicable upstream
rights. The tool name is used nominatively for provenance.

## License Or Terms Basis

The reviewed rustfmt repository contains MIT and Apache License 2.0 license
files. The complete current upstream license and notice set controls. A generic
dual-license summary must not replace exact distribution evidence.

## Distribution, Modification, And Compatibility

Running rustfmt does not relicense formatted source. A redistributed formatter
binary, source copy, or Rust toolchain must retain all applicable licenses,
notices, and third-party component records.

## Compliance Posture

Canonical validation and the dated Rust toolchain configuration confirm rustfmt
use. Keep the toolchain file and managed runtime synchronized, preserve the
exact
compiler commit and formatter component identity, and continue using the
repository's canonical validation flow rather than direct ad hoc formatting.

## Source References

- Rust Project (n.d.) *Rustup channels*. Documents stable, beta, nightly, dated
  toolchain pins, and component availability. Available at:
  <https://rust-lang.github.io/rustup/concepts/channels.html> (Accessed: 14 July
  2026).
- Rust Project (n.d.) *rustfmt*. Available at:
  <https://github.com/rust-lang/rustfmt> (Accessed: 14 July 2026).
- Rust Project Developers (n.d.) *rustfmt LICENSE-MIT*. Available at:
  <https://github.com/rust-lang/rustfmt/blob/main/LICENSE-MIT> (Accessed: 14
  July 2026).
- SHAR repository and managed runtime (2026), `nightly-2026-07-10`, rustc
  1.99.0-nightly, rustfmt 1.9.0-nightly, commit `af3d95584`, and canonical
  validation configuration.
