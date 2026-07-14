# pytest

This non-governing record documents the Python test framework used by the
repository and does not apply pytest's license to SHAR tests, fixtures, source
code, generated reports, or test inputs.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository use and authoritative upstream sources
  verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-12.
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

The canonical Python configuration declares pytest as a development dependency
and defines strict collection, warning, marker, cache, coverage, and failure
behavior. SHAR test modules use pytest fixtures, plain assertions, exception
matching, output capture, and deterministic loopback fixtures.

pytest is not a runtime dependency of the Rust tools, Unreal product, or the
installed MCP client unless a package expressly includes test tooling. Running
pytest does not transfer ownership of checked tests or fixtures to the pytest
project.

## Provenance And Version History

pytest runs use the release resolved as compatible with the selected Python
interpreter, plugin set, repository policies, and reproducibility requirements.
The exact version for each run is established by the canonical dependency
manifest, installed package metadata, lock or environment evidence, and captured
command output.

An installed version may lag upstream because of interpreter compatibility,
plugin compatibility, a deliberate stability hold, unavailable packaging,
delayed review, or human oversight. This record therefore does not represent a
version number as permanently current.

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

- pytest-dev contributors (n.d.) *pytest documentation*. Available at:
  <https://docs.pytest.org/en/stable/> (Accessed: 12 July 2026).
- pytest-dev contributors (n.d.) *License*. Available at:
  <https://docs.pytest.org/en/stable/license.html> (Accessed: 12 July 2026).
- pytest-dev contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/pytest-dev/pytest> (Accessed: 12 July 2026).
- SHAR repository and canonical Python authority (2026), pytest dependency,
  configuration, and tests under `src/mcp/tests/`.
