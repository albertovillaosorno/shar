# TOML

This non-governing record documents a configuration-language specification
without granting rights in repository configuration, package metadata, or
third-party parsers.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository use and the official TOML specification
  verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Build, package, and toolchain configuration.
- Subject class: Open configuration-file language.

## Covered Material

TOML syntax used by Cargo manifests, Rust toolchain configuration, Python
`pyproject.toml`, and repository configuration surfaces, including tables,
arrays, strings, numbers, dates, comments, dotted keys, and ordering-sensitive
human review.

## Repository Use And Scope

SHAR reads and writes TOML through tool-specific implementations. A valid TOML
file may still violate Cargo, Python packaging, repository, or security policy.
The TOML syntax does not define the meaning of application-specific keys.

## Provenance And Version History

The TOML project publishes a versioned specification and language tests. Parser
behavior, supported specification versions, duplicate-key handling, date
semantics, and application-specific extensions must be recorded for each tool.

## Authorship, Ownership, And Attribution

TOML contributors retain applicable rights in the specification and reference
materials. Parser authors retain rights in their implementations. Repository
configuration authors retain rights in independently authored configuration.

## License Or Terms Basis

The official TOML repository is MIT-licensed. That license does not apply to
configuration content merely because it uses TOML syntax, and it does not cover
every parser or application consuming TOML.

## Distribution, Modification, And Compatibility

Syntactic validity is not semantic validity. Configuration may expose private
paths, package sources, credentials, machine-specific state, or incompatible
version constraints. Such content requires independent review.

## Compliance Posture

- Record the TOML specification and parser versions used by consequential tools.
- Reject duplicate or ambiguous configuration according to the consuming tool.
- Keep secrets and machine-specific private paths out of tracked TOML.
- Treat application schemas and dependency constraints as separate authorities.
- Revalidate after parser or toolchain upgrades.

## Technical Baseline And SHAR Profile

### Public baseline

TOML defines a general configuration syntax. Cargo manifests, Cargo
configuration, Python `pyproject.toml`, and tool-specific tables apply separate
application schemas on top of that syntax. A syntactically valid TOML document
may therefore still be invalid for the owning tool.

The version shown in a public specification or supported by an installed parser
is time-bounded evidence, not a permanent repository requirement. This record
preserves observed parser and tool identities, reproducible manifests, and any
explicit compatibility hold established by repository authority.

### SHAR profile

The repository uses TOML through Rust package and workspace manifests, Python
packaging metadata, and tool configuration. Each file is governed first by the
schema of its owning tool. Unknown keys, duplicate keys, table placement,
dependency syntax, feature resolution, build-system declarations, and tool
namespaces must be validated by that owner rather than inferred from core TOML
syntax alone.

### Use-specific evidence limits

Before treating a tracked TOML surface as fully validated, inventory the file,
identify its owning application schema, record the parser or tool version used
by validation, and test repository-specific tightening, migration,
unknown-field, duplicate-field, and misplaced-field behavior.

### Verified sources

- TOML contributors, *TOML v1.0.0*. <https://toml.io/en/v1.0.0>
- Rust Project, *The Cargo Book: The Manifest Format*.
  <https://doc.rust-lang.org/cargo/reference/manifest.html>
- Python Packaging Authority, *pyproject.toml specification*.
  <https://packaging.python.org/en/latest/specifications/pyproject-toml/>
- SHAR repository manifests and `src/mcp/pyproject.toml`.

## Source References

- TOML contributors (n.d.) *TOML specification*. Available at:
  <https://toml.io/en/> (Accessed: 12 July 2026).
- TOML contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/toml-lang/toml> (Accessed: 12 July 2026).
- SHAR repository (2026), Cargo manifests, toolchain TOML, and Python package
  metadata.
