# pytest

This non-governing record documents the Python test framework used by the
repository and does not apply pytest's license to SHAR tests, fixtures, source
code, generated reports, or test inputs.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The canonical pytest 9.1.1 pin, PyPI's
  9.1.1 latest-release designation, release date, Python compatibility metadata,
  repository use, and upstream license were verified. The exact installed wheel,
  transitive graph, and loaded plugin set remain environment-specific.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: Development and validation dependency.
- Subject class: Open-source Python test framework and plugin host.

## Covered Material

The pytest test runner, assertion-rewriting machinery, fixture system, test
collection behavior, configuration surface, and plugin-loading boundary used by
SHAR's Python validation and MCP regression tests.

Third-party pytest plugins are separate subjects. In particular, pytest-cov and
Coverage.py are not covered by pytest's license merely because they execute in a
pytest process.

## Repository Use And Scope

The canonical Python configuration pins pytest 9.1.1 as a development
dependency and defines strict collection, warning, marker, cache, coverage, and
failure behavior. SHAR test modules use pytest fixtures, plain assertions,
exception matching, output capture, and deterministic loopback fixtures.

pytest is not a runtime dependency of the Rust tools, Unreal product, or the
installed MCP client unless a package expressly includes test tooling. Running
pytest does not transfer ownership of checked tests or fixtures to the pytest
project.

## Provenance And Version History

PyPI identifies pytest 9.1.1 as the latest release, published 19 June 2026,
and declares support for Python 3.10 or newer, including Python 3.14. The
canonical dependency pin therefore matched the reviewed latest release on 14
July 2026.

The exact version for each run is established by the canonical dependency
manifest, installed package metadata, lock or environment evidence, and captured
command output. A later installed version may lag upstream because of
interpreter or plugin compatibility, a deliberate stability hold, unavailable
packaging, delayed review, or human oversight. This record does not make 9.1.1
a permanent latest-version label.

## Authorship, Ownership, And Attribution

Holger Krekel, the pytest-dev team, and other contributors retain applicable
rights in pytest and its documentation. Plugin authors and dependency authors
retain rights in their respective components. SHAR contributors retain rights in
independently authored tests and fixtures subject to the repository license.

## License Or Terms Basis

The official pytest documentation and repository identify pytest as distributed
under the MIT License. Redistribution of pytest source or binaries requires
preservation of the applicable copyright and permission notice.

The MIT grant for pytest does not automatically cover plug-ins, Python itself,
package-manager metadata, bundled dependencies, test data, or generated coverage
artifacts. The exact distribution and dependency graph control.

## Distribution, Modification, And Compatibility

Invoking pytest as an external test runner does not relicense SHAR source or
test files. A distributed validator environment, virtual environment, wheel
cache, or bundled test harness must preserve the licenses and notices for pytest
and every included dependency.

pytest's plugin architecture is an execution boundary, not a single license
family. Each loaded plugin must be inventoried independently, and version
compatibility must be verified against the actual pytest and Python releases.

## Compliance Posture

- Keep the canonical dependency declaration and test configuration
  authoritative.
- Record the exact pytest, Python, and plugin identities for reproducible runs.
- Treat every plugin as a separately licensed component.
- Keep generated caches and reports outside tracked source unless explicitly
  reviewed and intended for publication.
- Do not infer that successful test execution proves legal compliance,
  correctness outside the tested scope, or production fitness.
- Reverify upstream release, license, and security information before bundling.

## Source References

- Python Package Index (2026) *pytest 9.1.1*. Identified as the latest release,
  published 19 June 2026, requiring Python 3.10 or newer. Available at:
  <https://pypi.org/project/pytest/> (Accessed: 14 July 2026).
- pytest-dev contributors (n.d.) *pytest documentation*. Available at:
  <https://docs.pytest.org/en/stable/> (Accessed: 14 July 2026).
- pytest-dev contributors (n.d.) *License*. Available at:
  <https://docs.pytest.org/en/stable/license.html> (Accessed: 14 July 2026).
- pytest-dev contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/pytest-dev/pytest> (Accessed: 14 July 2026).
- SHAR canonical Python authority (2026), pinning `pytest==9.1.1`, plus
  repository configuration and tests under `src/mcp/tests/`.
