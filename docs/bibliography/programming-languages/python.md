# Python

This non-governing record distinguishes Python itself from SHAR-authored scripts
and from independently licensed third-party packages.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The repository Python 3.14.6 pin,
  managed CPython 3.14.6 Windows AMD64 build, compiler identity, executable
  digest, OpenSSL identity, official current maintenance release, license
  history, project identity, and CPython source were verified. The complete
  standard-library payload and third-party package graph remain environment-
  specific.
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
upstream Python release. Python 3.14.6 was published on 10 June 2026 as the
sixth
maintenance release of Python 3.14.

The managed interpreter reports CPython 3.14.6 final for `win-amd64`, compiled
with MSC 19.44 as a 64-bit AMD64 build dated 23 June 2026. It reports OpenSSL
3.5.7 and executable SHA-256
`0e88c01f0bef4c1216d0f3e990662128163e5e932c2fdac75777084cd4b769e3`.
The Python downloads page identifies 3.14.6 as the current source and platform
release on 14 July 2026.

These values are dated runtime evidence, not a permanent latest-version label or
an unbounded compatibility range. They still do not inventory every standard-
library file or installed third-party package. Publication, distribution, and
validation evidence must preserve those identities from the managed environment
and package-resolution evidence. Python's historical license lineage includes
multiple institutional licensors and cannot be reduced to one copyright line for
all versions.

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

- Python Software Foundation (2026) *Python 3.14.6*. Released 10 June 2026 as
  the sixth Python 3.14 maintenance release, with Sigstore and SBOM evidence for
  supported artifacts. Available at:
  <https://www.python.org/downloads/release/python-3146/> (Accessed: 14 July
  2026).
- Python Software Foundation (2026) *Download Python*. Identifies Python 3.14.6
  as the latest source and platform release on 14 July 2026. Available at:
  <https://www.python.org/downloads/> (Accessed: 14 July 2026).
- Python Software Foundation (2026) *History and License for Python 3.14.6*.
  Available at: <https://docs.python.org/release/3.14.6/license.html> (Accessed:
  14 July 2026).
- Python Software Foundation (n.d.) *Python*. Available at:
  <https://www.python.org/> (Accessed: 14 July 2026).
- Python Software Foundation (n.d.) *CPython official GitHub repository*.
  Available at: <https://github.com/python/cpython> (Accessed: 14 July 2026).
- SHAR repository and managed runtime (2026), MCP package metadata pinning
  `requires-python = "==3.14.6"`; CPython 3.14.6 final for Windows AMD64;
  MSC 19.44 compiler identity; 23 June 2026 build date; OpenSSL 3.5.7; and
  reviewed executable SHA-256.
