# Coverage.py

This non-governing record documents the Python code-coverage engine used through
pytest-cov and does not apply Coverage.py's license to measured source, tests,
or generated report content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository configuration and authoritative
  upstream sources verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-12.
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
HTML reports through pytest-cov.

Coverage.py observes executed Python code. It does not establish semantic
correctness, absence of untested states, security, legal compliance, or adequacy
of assertions.

## Provenance And Version History

Coverage runs use the release resolved as compatible with the selected Python
interpreter and pytest-cov plugin. The actual version, configuration, data-file
schema, and report behavior must be preserved with the validation evidence for
each run.

An installed version may lag because of Python or plugin compatibility, a
stability hold, delayed review, packaging availability, or human oversight.
Bibliography prose does not override the actual environment evidence.

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
- Preserve Apache-2.0 and NOTICE obligations when redistributing Coverage.py.
- Review generated reports for confidential or third-party material before
  publication.
- Treat coverage percentages as bounded measurements, not legal or quality
  certification.
- Reverify current compatibility, license, and security information before
  bundling.

## Source References

- Coverage.py contributors (n.d.) *Coverage.py documentation*. Available at:
  <https://coverage.readthedocs.io/en/latest/> (Accessed: 12 July 2026).
- Coverage.py contributors (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/coveragepy/coveragepy> (Accessed: 12 July 2026).
- Coverage.py contributors (n.d.) *Apache License 2.0*. Available at:
  <https://raw.githubusercontent.com/coveragepy/coveragepy/main/LICENSE.txt>
  (Accessed: 12 July 2026).
- Coverage.py contributors (n.d.) *NOTICE*. Available at:
  <https://raw.githubusercontent.com/coveragepy/coveragepy/main/NOTICE.txt>
  (Accessed: 12 July 2026).
- SHAR canonical Python authority (2026), coverage run, report, XML, and HTML
  configuration.
