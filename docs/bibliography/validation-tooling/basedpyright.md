# BasedPyright

This non-governing record documents BasedPyright as validation tooling and does
not apply the tool's license to Python source merely because it checks that
source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The canonical BasedPyright 1.39.9 pin,
  PyPI's 1.39.9 latest-release designation, release date, Python compatibility,
  trusted-publishing provenance, repository configuration, and upstream license
  were verified. The exact installed wheel, bundled Pyright revision, Node
  runtime, and diagnostic behavior remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source Python static type checker.

## Covered Material

The BasedPyright package, command-line tooling, and documentation used for
strict Python type analysis.

## Repository Use And Scope

The canonical Python configuration pins BasedPyright 1.39.9, selects strict
type checking, and enables additional diagnostics that reject unresolved `Any`
flows and unnecessary suppressions. BasedPyright validates authored Python
templates and integration helpers. It is a development and validation tool, not
a runtime dependency of the generated game. Any package, executable,
language-server component, or bundled dependency remains upstream material.

## Provenance And Version History

PyPI identifies BasedPyright 1.39.9 as the latest release, published 27 June
2026, and declares support for Python 3.8 or newer, including Python 3.14. The
canonical dependency pin therefore matched the reviewed latest release on 14
July 2026. PyPI also records trusted publication and package-file hashes for the
release.

The exact installed package, bundled Pyright revision, Node runtime, invocation,
and diagnostic result must still be taken from the repository-managed
environment, package metadata, and captured validation evidence. This dated
record does not make 1.39.9 a permanent latest-version label.

## Authorship, Ownership, And Attribution

BasedPyright is a fork-derived project. The reviewed license file attributes the
underlying Pyright code to Microsoft under the MIT License. Fork maintainers and
contributors may hold rights in later contributions. Preserve both upstream and
fork-specific notices rather than attributing the complete work to one entity.

## License Or Terms Basis

The reviewed upstream repository presents an MIT license file. The exact current
license, copyright notices, package metadata, and dependency notices control.
The existence of a permissive license does not remove attribution obligations or
third-party rights.

## Distribution, Modification, And Compatibility

Running BasedPyright does not relicense checked source. Redistributing the
package, executable, language server, or dependencies requires preservation of
applicable license and notice material.

## Compliance Posture

Canonical validation and repository configuration confirm BasedPyright use.
Preserve exact package metadata and verify fork-specific and inherited
attribution before redistribution.

## Source References

- Python Package Index (2026) *BasedPyright 1.39.9*. Identified as the latest
  release, published 27 June 2026, requiring Python 3.8 or newer. Available at:
  <https://pypi.org/project/basedpyright/> (Accessed: 14 July 2026).
- BasedPyright contributors (n.d.) *BasedPyright documentation*. Available at:
  <https://docs.basedpyright.com/> (Accessed: 14 July 2026).
- BasedPyright contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/DetachHead/basedpyright> (Accessed: 14 July 2026).
- BasedPyright contributors (n.d.) *LICENSE.txt*. Available at:
  <https://raw.githubusercontent.com/DetachHead/basedpyright/main/LICENSE.txt>
  (Accessed: 14 July 2026).
- SHAR canonical Python authority (2026), pinning
  `basedpyright==1.39.9` and defining strict diagnostic policy.
