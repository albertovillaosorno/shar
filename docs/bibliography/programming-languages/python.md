# Python

This non-governing record distinguishes Python itself from SHAR-authored scripts
and from independently licensed third-party packages.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Required Python 3.14.6 runtime, official
  license history, project identity, and CPython source verified; the exact
  local interpreter build, standard-library payload, and third-party package
  graph
  remain environment-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Programming language, runtime, and documentation ecosystem.

## Covered Material

The Python language and runtime used for bounded integration scripts, validation
support, Blender review helpers, and Unreal-facing automation.

## Repository Use And Scope

Python is used where Blender or Unreal exposes a materially better native
integration boundary and for selected repository tooling. Python does not own or
license scripts merely because they are written in the language. The runtime,
standard library, documentation, and every third-party package retain separate
rights and obligations.

## Provenance And Version History

The MCP package pins `requires-python = "==3.14.6"`, and canonical validation
uses the same current stable Python patch. Other exact runtime and package
versions for publication, distribution, or validation must be taken from the
repository-managed environment and lock evidence. Python's historical license
lineage includes multiple institutional licensors and cannot be reduced to one
copyright line for all versions.

## Authorship, Ownership, And Attribution

The Python Software Foundation and the historical licensors identified by the
official license history retain applicable upstream rights. Individual package
maintainers and contributors retain rights in their packages.

## License Or Terms Basis

Python is distributed under the Python Software Foundation License Version 2
with historical component licenses. The official history also identifies a
Zero-Clause BSD license for eligible code examples in documentation beginning
with specified releases. Those terms do not automatically apply to third-party
packages, copied prose, or independently authored SHAR scripts.

## Distribution, Modification, And Compatibility

Any bundled interpreter, standard library, wheel, package, copied example, or
embedded runtime requires component-level notice and license verification.
Merely running Python does not relicense SHAR source.

## Compliance Posture

Repository manifests and authored scripts confirm Python use. Preserve exact
runtime, package, license, notice, and source-provenance evidence for any
redistributed environment.

## Source References

- Python Software Foundation (2026) *History and License*. Available at:
  <https://docs.python.org/3/license.html> (Accessed: 13 July 2026).
- Python Software Foundation (n.d.) *Python*. Available at:
  <https://www.python.org/> (Accessed: 13 July 2026).
- Python Software Foundation (n.d.) *CPython official GitHub repository*.
  Available at: <https://github.com/python/cpython> (Accessed: 13 July 2026).
- SHAR repository (2026) `src/mcp/pyproject.toml` (pins
  `requires-python = "==3.14.6"`).
