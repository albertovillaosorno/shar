# rustfmt

This non-governing record documents rustfmt as canonical formatting tooling and
does not apply rustfmt licensing to formatted SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The tracked nightly selection, managed
  stable and nightly rustfmt components, compiler identities, invocation,
  formatting authority, official source repository, documentation, and
  licensing posture were verified. Edition interaction and formatted output
  remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-15.
- Subject class: Open-source Rust source-code formatter.

## Covered Material

The rustfmt tool used through the repository's canonical formatting and
validation flow.

## Repository Use And Scope

rustfmt enforces canonical formatting for applicable Rust source under
repository-controlled configuration. Repository authority requires using the
canonical validator for final evidence rather than substituting ad hoc direct
formatter commands. rustfmt is not a runtime dependency of the generated game.

The root dependency bootstrap installs rustfmt for exact stable Rust 1.97.0 and
reconciles rustfmt on the separate floating nightly during every dependency
bootstrap. The tracked SHAR validation manifest selects `nightly` and adds the
repository-owned formatter override `fn_params_layout=Vertical`.

## Provenance And Version History

The reviewed stable component reported rustfmt 1.9.0-stable from compiler commit
`2d8144b78`, dated 7 July 2026. The reviewed SHAR nightly component reported
rustfmt 1.9.0-nightly from compiler commit `da80ed070`, dated 14 July 2026.

Those exact values identify the reviewed managed artifacts only. They are not
permanent requirements or a claim that a floating `nightly` label reproduces the
same formatter indefinitely.

Rustup documents that nightly toolchains may omit optional components and that a
requested component set can resolve to an older component-complete nightly.
Validation evidence must therefore preserve the actual compiler commit, rustfmt
component version, host, target, formatter configuration, and validation result.

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

- Keep stable and nightly rustfmt components synchronized with their governed
  compiler channels.
- Preserve the exact compiler commit, formatter version, host, target,
  configuration, and output evidence for each run.
- Treat the floating channel as an update policy, not a reproducible identity.
- Continue using canonical validation rather than direct ad hoc formatting for
  repository acceptance evidence.

## Source References

- Rust Project (n.d.) *Rustup channels*. Documents stable, beta, nightly,
  updates, version-specific toolchains, and optional-component availability.
  Available at: <https://rust-lang.github.io/rustup/concepts/channels.html>
  (Accessed: 15 July 2026).
- Rust Project (n.d.) *rustfmt*. Available at:
  <https://github.com/rust-lang/rustfmt> (Accessed: 15 July 2026).
- Rust Project Developers (n.d.) *rustfmt LICENSE-MIT*. Available at:
  <https://github.com/rust-lang/rustfmt/blob/main/LICENSE-MIT> (Accessed: 15
  July 2026).
- SHAR repository and managed command authority (2026), validation manifest
  selecting `nightly`, root bootstrap authority, rustfmt 1.9.0-stable,
  rustfmt 1.9.0-nightly, compiler commits `2d8144b78` and `da80ed070`, formatter
  configuration, and canonical validation output.
