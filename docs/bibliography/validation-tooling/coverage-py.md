# Coverage.py

This non-governing record documents the Python code-coverage engine used through
pytest-cov and does not apply Coverage.py's license to measured source, tests,
or generated report content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository coverage configuration,
  installed Coverage.py 7.14.0 metadata, pytest-cov's `>=7.10.6` requirement,
  PyPI's Coverage.py 7.15.1 latest-release designation, release date, Python
  compatibility metadata, license, and NOTICE were verified. Coverage.py is not
  directly pinned, and no declared compatibility hold explains the observed
  transitive lag.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: Development and validation dependency.
- Subject class: Open-source Python coverage-measurement and reporting tool.

## Covered Material

Coverage.py's statement and branch measurement, data-file format, source
analysis, report generation, configuration surface, and Python tracing support
used by the repository's canonical test profile.

pytest and pytest-cov are separate subjects. Generated terminal, XML, and HTML
reports are outputs, not upstream Coverage.py source code.

## Repository Use And Scope

The canonical Python configuration enables branch coverage, identifies measured
source roots, excludes test and generated boundaries where specified, stores
coverage state under ignored cache directories, and generates terminal, XML, and
HTML reports through pytest-cov. It pins pytest-cov but does not directly pin
Coverage.py. The reviewed shared runtime resolves Coverage.py 7.14.0, so run
evidence must retain that actual transitive identity rather than infer it from
the pytest-cov pin.

Coverage.py observes executed Python code. It does not establish semantic
correctness, absence of untested states, security, legal compliance, or adequacy
of assertions.

## Provenance And Version History

PyPI identifies Coverage.py 7.15.1 as the latest release, published 12 July
2026, and declares support for Python 3.10 or newer, including Python 3.14. The
reviewed shared runtime instead resolves Coverage.py 7.14.0. pytest-cov 7.1.0
requires `coverage[toml]>=7.10.6`, so both releases satisfy its declared lower
bound and no package-metadata constraint explains the observed one-patch lag.

These are dated environment and upstream-currentness observations, not a
permanent version requirement. Because the canonical dependency declaration does
not directly pin Coverage.py, the actual resolved version, package hashes,
configuration, data-file schema, and report behavior must be preserved with each
run. A transitive release may lag because of a resolver snapshot, stability hold,
delayed refresh, packaging availability, or human oversight. Bibliography prose
does not override the actual environment evidence.

## Authorship, Ownership, And Attribution

Ned Batchelder and Coverage.py contributors retain applicable rights in the
project and documentation. Authors of measured code, tests, and dependencies
retain their respective rights. Generated reports may incorporate factual
metrics and limited source-related information from the measured project.

## License Or Terms Basis

The official Coverage.py repository identifies the project as licensed under
Apache License 2.0 and includes a NOTICE file. Redistribution must follow the
license and preserve applicable notices. The Apache license does not apply to
measured SHAR source merely because Coverage.py analyzed it.

## Distribution, Modification, And Compatibility

External execution does not relicense measured code. A bundled validation
environment must preserve Coverage.py's Apache-2.0 license, NOTICE material, and
all notices for bundled dependencies.

Generated reports require a separate publication decision. They may expose
repository paths, source filenames, source excerpts, internal test taxonomy, or
other information not intended for publication or distribution. HTML report
assets and copied source
views must be reviewed before redistribution.

## Compliance Posture

- Keep coverage data and reports in ignored cache directories by default.
- Record exact tool, Python, plugin, and configuration identities per run.
- Refresh the transitive Coverage.py resolution or document a narrow stability
  hold before representing the shared runtime as current.
- Preserve Apache-2.0 and NOTICE obligations when redistributing Coverage.py.
- Review generated reports for confidential or third-party material before
  publication.
- Treat coverage percentages as bounded measurements, not legal or quality
  certification.
- Reverify current compatibility, license, and security information before
  bundling.

## Source References

- Python Package Index (2026) *Coverage.py 7.15.1*. Identified as the latest
  release, published 12 July 2026, requiring Python 3.10 or newer. Available at:
  <https://pypi.org/project/coverage/> (Accessed: 14 July 2026).
- Coverage.py contributors (n.d.) *Coverage.py documentation*. Available at:
  <https://coverage.readthedocs.io/en/latest/> (Accessed: 14 July 2026).
- Coverage.py contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/coveragepy/coveragepy> (Accessed: 14 July 2026).
- Coverage.py contributors (n.d.) *Apache License 2.0*. Available at:
  <https://raw.githubusercontent.com/coveragepy/coveragepy/main/LICENSE.txt>
  (Accessed: 14 July 2026).
- Coverage.py contributors (n.d.) *NOTICE*. Available at:
  <https://raw.githubusercontent.com/coveragepy/coveragepy/main/NOTICE.txt>
  (Accessed: 14 July 2026).
- SHAR canonical Python authority and shared runtime (2026), coverage run,
  report, XML, and HTML configuration; installed Coverage.py 7.14.0;
  pytest-cov 7.1.0 metadata requiring `coverage[toml]>=7.10.6`; Coverage.py is
  not directly pinned.
