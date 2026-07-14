# Clippy

This non-governing record documents Clippy as Rust linting tooling and does not
apply Clippy licensing to checked SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The repository's dated nightly pin,
  managed Clippy 0.1.98 component, compiler commit identity, invocation, lint
  policy, official Rust documentation, source repository, and licensing evidence
  were verified. The active lint inventory, target, compile graph, and
  diagnostics remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source Rust linting tool.

## Covered Material

The Rust Clippy lints used by repository validation.

## Repository Use And Scope

Clippy enforces correctness, style, complexity, and safety-oriented diagnostics
for authored Rust source. It is a development and validation tool, not a runtime
dependency of the generated game. Narrow repository lint exceptions remain SHAR
decisions and do not modify upstream terms.

## Provenance And Version History

The repository pins `nightly-2026-07-10` and requires the managed runtime to move
with that file. The reviewed runtime reported rustc 1.99.0-nightly and Clippy
0.1.98 from compiler commit `af3d95584`, dated 9 July 2026.

Rustup documents that nightly toolchains may omit optional components such as
Clippy and can be pinned by exact date. Validation evidence must therefore retain
the compiler commit, Clippy component version, target, enabled lint policy,
compile graph, and diagnostics rather than relying on a floating channel label.

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

Canonical validation and the dated Rust toolchain configuration confirm Clippy
use. Keep the toolchain file and managed runtime synchronized, keep exceptions
narrow, explained, and test-covered, and preserve the exact compiler commit,
component version, target, lint inventory, and compile graph for every published
or distributed validation artifact.

## Source References

- Rust Project (n.d.) *Rustup channels*. Documents nightly channels, dated
  toolchain pins, and optional-component availability. Available at:
  <https://rust-lang.github.io/rustup/concepts/channels.html> (Accessed: 14 July
  2026).
- Rust Project (n.d.) *Clippy documentation*. Available at:
  <https://doc.rust-lang.org/clippy/> (Accessed: 14 July 2026).
- Rust Project (n.d.) *Clippy official GitHub repository*. Available at:
  <https://github.com/rust-lang/rust-clippy> (Accessed: 14 July 2026).
- Rust Project Developers (n.d.) *Clippy LICENSE-MIT*. Available at:
  <https://raw.githubusercontent.com/rust-lang/rust-clippy/master/LICENSE-MIT>
  (Accessed: 14 July 2026).
- SHAR repository and managed runtime (2026), `nightly-2026-07-10`, rustc
  1.99.0-nightly, Clippy 0.1.98, commit `af3d95584`, lint attributes, and
  canonical validation policy.
