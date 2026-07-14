# Python

This non-governing record distinguishes Python itself from SHAR-authored scripts
and from independently licensed third-party packages.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The repository Python 3.14.6 pin,
  official 3.14.6 maintenance release, license history, project identity, and
  CPython source were verified. The exact interpreter build, standard-library
  payload, platform architecture, and third-party package graph remain
  environment-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
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

The MCP package pins `requires-python = "==3.14.6"`; that exact patch is the
repository contract for the package, not a permanent claim about the newest
upstream Python release. Python 3.14.6 was published on 10 June 2026 as the sixth
maintenance release of Python 3.14.

The pin does not identify an interpreter build, architecture, standard-library
payload, or installed package graph. Publication, distribution, and validation
evidence must take those identities from the repository-managed environment and
lock evidence. Python's historical license lineage includes multiple
institutional licensors and cannot be reduced to one copyright line for all
versions.

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

- Python Software Foundation (2026) *Python 3.14.6*. Released 10 June 2026.
  Available at: <https://www.python.org/downloads/release/python-3146/>
  (Accessed: 14 July 2026).
- Python Software Foundation (2026) *History and License for Python 3.14.6*.
  Available at: <https://docs.python.org/release/3.14.6/license.html> (Accessed:
  14 July 2026).
- Python Software Foundation (n.d.) *Python*. Available at:
  <https://www.python.org/> (Accessed: 14 July 2026).
- Python Software Foundation (n.d.) *CPython official GitHub repository*.
  Available at: <https://github.com/python/cpython> (Accessed: 14 July 2026).
- SHAR repository (2026) MCP package metadata pinning
  `requires-python = "==3.14.6"`.
