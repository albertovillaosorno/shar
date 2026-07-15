# Clippy

This non-governing record documents Clippy as Rust linting tooling and does not
apply Clippy licensing to checked SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The tracked nightly selection, managed
  stable and nightly Clippy components, compiler identities, invocation, lint
  policy, official Rust documentation, source repository, and licensing
  evidence were verified. The active lint inventory, target, compile graph, and
  diagnostics remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-15.
- Subject class: Open-source Rust linting tool.

## Covered Material

The Rust Clippy lints used by repository validation.

## Repository Use And Scope

Clippy enforces correctness, style, complexity, and safety-oriented diagnostics
for authored Rust source. It is a development and validation tool, not a runtime
dependency of the generated game. Narrow repository lint exceptions remain SHAR
decisions and do not modify upstream terms.

The root dependency bootstrap installs Clippy for exact stable Rust 1.97.0 and
reconciles Clippy on the separate floating nightly during every dependency
bootstrap. The tracked SHAR validation manifest selects `nightly`; canonical
validation owns the enabled lint policy and target graph.

## Provenance And Version History

The reviewed stable component reported Clippy 0.1.97 from compiler commit
`2d8144b78`, dated 7 July 2026. The reviewed SHAR nightly component reported
Clippy 0.1.99 from compiler commit `da80ed070`, dated 14 July 2026.

Those exact values identify the reviewed managed artifacts only. They are not
permanent requirements or a claim that a floating `nightly` label reproduces the
same diagnostics indefinitely.

Rustup documents that nightly toolchains may omit optional components such as
Clippy and that a requested component set can resolve to an older
component-complete nightly. Validation evidence must therefore preserve the
compiler commit, Clippy version, host, target, enabled lint policy, compile
graph, and diagnostics for each run.

## Authorship, Ownership, And Attribution

The Rust Project Developers and Clippy contributors retain applicable upstream
rights. Individual diagnostics and documentation remain subject to upstream
notices.

## License Or Terms Basis

The reviewed Clippy repository contains MIT and Apache License 2.0 license
files. The complete current upstream license and notice set controls. A generic
dual-license summary does not replace exact distribution evidence.

## Distribution, Modification, And Compatibility

Running Clippy does not relicense checked source. Redistributing Clippy or a
Rust toolchain requires preservation of applicable licenses, notices, and
third-party component records.

## Compliance Posture

- Keep stable and nightly Clippy components synchronized with their governed
  compiler channels.
- Preserve the exact compiler commit, Clippy version, host, target, lint policy,
  compile graph, and diagnostic evidence for each run.
- Treat the floating channel as an update policy, not a reproducible identity.
- Keep exceptions narrow, explained, and test-covered.

## Source References

- Rust Project (n.d.) *Rustup channels*. Documents nightly channels, updates,
  version-specific toolchains, and optional-component availability. Available
  at: <https://rust-lang.github.io/rustup/concepts/channels.html> (Accessed: 15
  July 2026).
- Rust Project (n.d.) *Clippy documentation*. Available at:
  <https://doc.rust-lang.org/clippy/> (Accessed: 15 July 2026).
- Rust Project (n.d.) *Clippy official GitHub repository*. Available at:
  <https://github.com/rust-lang/rust-clippy> (Accessed: 15 July 2026).
- Rust Project Developers (n.d.) *Clippy LICENSE-MIT*. Available at:
  <https://raw.githubusercontent.com/rust-lang/rust-clippy/master/LICENSE-MIT>
  (Accessed: 15 July 2026).
- SHAR repository and managed command authority (2026), validation manifest
  selecting `nightly`, root bootstrap authority, Clippy 0.1.97, Clippy 0.1.99,
  compiler commits `2d8144b78` and `da80ed070`, lint attributes, and canonical
  validation output.
