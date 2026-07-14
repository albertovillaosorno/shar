# Ruff

This non-governing record documents Ruff as validation tooling and does not
apply Ruff's license to SHAR source merely because Ruff checks that source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The canonical Ruff 0.15.21 pin, PyPI's
  0.15.21 latest-release designation, release date, repository rule policy,
  upstream documentation, source repository, and license were verified. The
  exact installed executable, parser behavior, active rule inventory, and fix
  output remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source Python linter and formatter.

## Covered Material

The Ruff command-line tool and official documentation used by repository
validation.

## Repository Use And Scope

The canonical Python configuration pins Ruff 0.15.21, enables preview behavior,
selects every available lint rule, and delegates canonical formatting to Ruff
with only documented conflict-pair exclusions. Ruff validates and formats
applicable Python sources under repository policy. It is a development and
validation tool, not a runtime dependency of the generated game. Any
repository-managed Ruff package or executable remains upstream software.

## Provenance And Version History

PyPI identifies Ruff 0.15.21 as the latest release and dates it 9 July 2026.
The canonical dependency pin therefore matched the reviewed latest release on
14 July 2026.

The exact executable, platform artifact, invocation, parser behavior, active
preview rules, and diagnostics for a validation result must still be obtained
from the repository-managed environment, package metadata, and captured command
evidence. This dated record does not make 0.15.21 a permanent latest-version
label.

## Authorship, Ownership, And Attribution

Astral Software and Ruff contributors retain applicable upstream rights. The
Ruff name is used nominatively to identify the tool.

## License Or Terms Basis

The reviewed upstream Ruff repository carries the MIT License. A distributed
copy or substantial portion must preserve the applicable copyright and
permission notice. The exact current upstream license file and dependency
notices control.

## Distribution, Modification, And Compatibility

Running Ruff does not relicense checked source. Redistributing a Ruff package,
executable, source copy, or bundled dependency requires preservation of its
license and all applicable third-party notices.

## Compliance Posture

Canonical validation and repository configuration confirm Ruff use. Preserve
exact version and license evidence for any distributed validation-tool bundle.

## Source References

- Python Package Index (2026) *Ruff 0.15.21*. Identified as the latest release,
  published 9 July 2026. Available at: <https://pypi.org/project/ruff/>
  (Accessed: 14 July 2026).
- Astral Software (n.d.) *Ruff documentation*. Available at:
  <https://docs.astral.sh/ruff/> (Accessed: 14 July 2026).
- Astral Software and contributors (n.d.) *Ruff official GitHub repository*.
  Available at: <https://github.com/astral-sh/ruff> (Accessed: 14 July 2026).
- Astral Software and contributors (n.d.) *Ruff LICENSE*. Available at:
  <https://github.com/astral-sh/ruff/blob/main/LICENSE> (Accessed: 14 July
  2026).
- SHAR canonical Python authority (2026), pinning `ruff==0.15.21` and defining
  the all-rules preview lint and canonical formatting policy.
