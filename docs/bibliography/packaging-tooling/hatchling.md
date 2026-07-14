# Hatchling

This non-governing record documents the Python build backend declared by the
repository and does not apply Hatchling licensing to the built SHAR package or
independently authored Python source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository manifest and authoritative upstream
  sources verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-12.
- Distribution posture: Build-time dependency for the Python MCP client package.
- Subject class: Python packaging build backend.

## Covered Material

Hatchling as the PEP 517 build backend declared by `src/mcp/pyproject.toml`,
including the wheel builder used to package the independently authored terminal
MCP client. The full Hatch project manager is outside repository use unless it
is separately invoked.

## Repository Use And Scope

The MCP package manifest declares `hatchling.build` as its build backend and
uses Hatch's wheel-target configuration to select the package directory.
Hatchling is a build dependency resolved by a Python packaging frontend; it is
not a runtime dependency declared for the installed MCP client.

The exact build requirement is controlled by the manifest. This bibliography
record intentionally does not restate its number as a permanent project policy.

## Provenance And Version History

For a Python package build that uses Hatchling, the compatible release resolved
by the canonical manifest and build environment is the relevant evidence. The
manifest may intentionally constrain a release, and it may lag upstream because
of compatibility, delayed review, or human oversight. The manifest,
build-isolation environment, package index metadata, hashes, and build logs
establish the component identity for a particular artifact.

## Authorship, Ownership, And Attribution

Ofek Lev, PyPA project contributors, and dependency authors retain applicable
rights in Hatch and Hatchling. SHAR contributors retain rights in independently
authored package source and metadata subject to the repository license.

## License Or Terms Basis

The official Hatch documentation and repository identify the project as licensed
under the MIT License. Build dependencies and plug-ins retain their own licenses
and notices. The exact source distribution, wheel metadata, and resolved
dependency graph control.

## Distribution, Modification, And Compatibility

Using Hatchling to build a wheel does not ordinarily apply Hatchling's license
to the package being built. A distributed build environment, vendored backend,
or bundled dependency cache must preserve the applicable licenses and notices
for Hatchling and every included dependency.

## Compliance Posture

- Keep the build backend declaration in the package manifest authoritative.
- Use isolated, reproducible builds with recorded dependency hashes.
- Distinguish Hatchling from the broader Hatch command-line application.
- Do not infer runtime dependency status from build-system use.
- Preserve backend and transitive license evidence when distributing a build
  environment.
- Reverify current compatibility before changing the manifest constraint.

## Source References

- Python Packaging Authority (n.d.) *pyproject.toml specification*. Available
  at: <https://packaging.python.org/en/latest/specifications/pyproject-toml/>
  (Accessed: 12 July 2026).
- Hatch contributors (n.d.) *Hatch documentation*. Available at:
  <https://hatch.pypa.io/latest/> (Accessed: 12 July 2026).
- Hatch contributors (n.d.) *Hatchling history*. Available at:
  <https://hatch.pypa.io/latest/history/hatchling/> (Accessed: 12 July 2026).
- Hatch contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/pypa/hatch> (Accessed: 12 July 2026).
- SHAR repository (2026) `src/mcp/pyproject.toml`.
