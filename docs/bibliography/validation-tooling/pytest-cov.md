# pytest-cov

This non-governing record documents the pytest coverage-integration plugin used
by the canonical Python test profile and does not apply the plugin's license to
measured source, tests, Coverage.py, or generated reports.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository use and authoritative upstream sources
  verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-12.
- Distribution posture: Development and validation dependency.
- Subject class: Open-source pytest plugin for Coverage.py integration.

## Covered Material

The pytest-cov plugin, its pytest command-line options, coverage-data lifecycle,
report orchestration, and integration with Coverage.py.

pytest and Coverage.py remain separate subjects with separate authorship,
licenses, release histories, and notice requirements.

## Repository Use And Scope

The canonical Python configuration declares pytest-cov as a development
dependency and uses coverage options for source selection, branch measurement,
terminal reporting, XML reporting, HTML reporting, and a minimum coverage gate.

pytest-cov coordinates pytest and Coverage.py. It does not itself determine the
copyright or publication status of measured source, copied report excerpts, test
inputs, or generated HTML and XML artifacts.

## Provenance And Version History

pytest-cov runs use the release resolved as compatible with the selected pytest,
Coverage.py, and Python versions. The canonical dependency manifest and
installed package evidence establish the component identity for a particular
run.

Plugin behavior can change across releases, including subprocess measurement,
coverage-data handling, option forwarding, and compatibility requirements.
Versions may lag upstream for compatibility, stability, delayed review, or human
oversight and must not be inferred from this record.

## Authorship, Ownership, And Attribution

pytest-cov contributors retain applicable rights in the plugin and its
documentation. pytest contributors, Coverage.py contributors, and other
dependency authors retain independent rights in their components.

## License Or Terms Basis

The official pytest-cov repository identifies the plugin as MIT-licensed.
Redistribution requires preservation of the applicable copyright and permission
notice. Coverage.py is separately licensed under Apache License 2.0 and is not
relicensed by pytest-cov.

## Distribution, Modification, And Compatibility

Invoking pytest-cov does not relicense measured SHAR source or tests. Bundling a
Python validator or virtual environment requires a complete dependency and
notice inventory, including pytest, pytest-cov, Coverage.py, and transitive
packages.

Coverage reports may contain filenames, paths, metrics, and source excerpts.
Before publishing a report, review it for confidential paths, untracked local
information, third-party source excerpts, and generated-artifact licensing.

## Compliance Posture

- Keep pytest-cov separate from pytest and Coverage.py in the notice inventory.
- Record exact versions and configuration for each validation result.
- Store coverage data and generated reports in ignored cache locations.
- Review reports before external publication.
- Do not interpret a coverage threshold as proof of test quality or complete
  behavioral verification.
- Reverify plugin compatibility and license evidence before bundling.

## Source References

- pytest-cov contributors (n.d.) *pytest-cov documentation*. Available at:
  <https://pytest-cov.readthedocs.io/en/latest/> (Accessed: 12 July 2026).
- pytest-cov contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/pytest-dev/pytest-cov> (Accessed: 12 July 2026).
- pytest-cov contributors (n.d.) *MIT License*. Available at:
  <https://github.com/pytest-dev/pytest-cov/blob/master/LICENSE> (Accessed: 12
  July 2026).
- SHAR canonical Python authority (2026), pytest-cov dependency and coverage
  command-line configuration.
