# Hatchling

This non-governing record documents the Python build backend declared by the
repository and does not apply Hatchling licensing to the built SHAR package or
independently authored Python source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The repository Hatchling 1.31.0 pin,
  PyPI's 1.31.0 latest-release designation, release date, package metadata, and
  trusted-publishing provenance were verified. The exact isolated build
  environment and transitive dependency graph remain artifact-specific.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: Build-time dependency for the Python MCP client package.
- Subject class: Python packaging build backend.

## Covered Material

Hatchling as the PEP 517 build backend declared by `src/mcp/pyproject.toml`,
including the wheel builder used to package the independently authored terminal
MCP client. The full Hatch project manager is outside repository use unless it
is separately invoked.

## Repository Use And Scope

The MCP package manifest declares `hatchling.build` as its build backend, pins
Hatchling 1.31.0 exactly, and uses Hatch's wheel-target configuration to select
the package directory. Hatchling is a build dependency resolved by a Python
packaging frontend; it is not a runtime dependency declared for the installed
MCP client.

The manifest remains the project authority. The version recorded here is dated
evidence of the reviewed state, not a second dependency declaration or a
permanent latest-version policy.

## Provenance And Version History

PyPI identifies Hatchling 1.31.0 as the latest release and dates it 8 July
2026. The repository's exact pin therefore matched the reviewed upstream
release on 14 July 2026. PyPI also records trusted publication, source and wheel
hashes, and provenance attestations for that release.

The manifest may later constrain another release or lag upstream because of
compatibility, delayed review, or human oversight. The manifest,
build-isolation environment, package index metadata, hashes, attestations, and
build logs establish the component identity for a particular artifact.

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
- Python Package Index (2026) *Hatchling 1.31.0*. Identified as the latest
  release, published 8 July 2026, with package hashes and trusted-publishing
  provenance. Available at: <https://pypi.org/project/hatchling/> (Accessed: 14
  July 2026).
- Hatch contributors (n.d.) *Hatchling history*. Available at:
  <https://hatch.pypa.io/latest/history/hatchling/> (Accessed: 14 July 2026).
- Hatch contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/pypa/hatch> (Accessed: 14 July 2026).
- SHAR repository (2026) `src/mcp/pyproject.toml`, pinning
  `hatchling==1.31.0`.
