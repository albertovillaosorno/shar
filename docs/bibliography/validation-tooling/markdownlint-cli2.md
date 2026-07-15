# markdownlint-cli2 And markdownlint

This non-governing record documents the Markdown command-line validator and its
rule engine without applying their licenses to checked documentation, reference
definitions, configuration files, or authored prose.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Managed markdownlint-cli2 0.23.0,
  markdownlint 0.41.0, their exact package relationship, official package
  metadata, repository use, and upstream licenses were verified. The CLI
  matches the current release. Its exact transitive engine remains one patch
  behind the standalone markdownlint 0.41.1 release.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: Development and validation dependencies.
- Subject class: Open-source Markdown command-line interface and lint library.

## Covered Material

This related validation family contains distinct components:

| Component | Function | Relationship |
| :-------- | :------- | :----------- |
| `markdownlint-cli2` | Markdown command interface | Invoked by `validate.sh` |
| `markdownlint` | CommonMark rule engine | Exact CLI dependency |
| Node.js | Runtime | Executes the validators |
| Parser packages | Parsing and globbing | Transitive graph |

The CLI, rule engine, runtime, and transitive packages retain their own release,
authorship, security, and license evidence even when installed together.

## Repository Use And Scope

`validate.sh` invokes the managed Node.js runtime and `markdownlint-cli2` module
with `--no-globs`, an explicit canonical configuration path, and an enumerated
Markdown file list. The canonical configuration enables the complete rule set
and pins deterministic style options.

The managed CLI package is markdownlint-cli2 0.23.0. Its package metadata
requires Node.js `>=22`, reports the MIT license, and declares markdownlint
0.41.0 as an exact dependency. The installed lockfile and package tree resolve
that exact engine version. The managed Node.js v26.5.0 runtime satisfies both
packages' engine floor.

The checked Markdown remains authored repository material. Running the validator
does not transfer ownership, create a derivative license from the tool, or make
the tool an authority for legal conclusions in the prose.

The bibliography template uses brace-delimited prompts instead of unresolved
reference labels. MD052 therefore remains enabled without a file-wide
suppression, while MD053 continues to reject unused reference definitions. Real
links, definitions, inline code, and task-list markers retain normal Markdown
syntax.

## Provenance And Version History

Root package authority, the lockfile, installed package metadata, and
executable output establish markdownlint-cli2 0.23.0 with markdownlint 0.41.0.
The npm registry identifies markdownlint-cli2 0.23.0 as current and dates it
1 July 2026. The registry identifies markdownlint 0.41.1 as current and dates
it 13 July 2026.
The managed CLI is therefore current, while its exact upstream-declared engine
is one patch behind the standalone rule-engine package.

The engine difference is not an undocumented repository hold or an accidental
loose resolution: markdownlint-cli2 0.23.0 declares `markdownlint: 0.41.0`
exactly. An independent override could alter a tested upstream package graph and
must not be introduced merely to erase a currentness finding. Recheck the
engine when a later CLI release changes its dependency, or document and test an
explicit override decision separately.

Exact run identities remain established by installed package metadata, lockfile
evidence, runtime output, canonical configuration, and validation logs. Rule
behavior, aliases, parser behavior, automatic fixes, configuration schemas, and
exit-code handling may change across releases.

## Authorship, Ownership, And Attribution

David Anson and contributors retain applicable rights in markdownlint-cli2,
markdownlint, and their documentation. Node.js, parser, globbing, schema, and
other dependency contributors retain independent rights in their components.
SHAR contributors retain rights in independently authored Markdown and
configuration subject to the repository license.

## License Or Terms Basis

The installed markdownlint-cli2 and markdownlint package metadata and their
official repositories identify both projects as MIT-licensed. Redistribution
requires preservation of the applicable copyright and permission notices for
each project. The complete installed Node.js package graph may contain
additional licenses and notices that must be inventoried from the exact lockfile
and package contents.

## Distribution, Modification, And Compatibility

External lint execution does not relicense checked Markdown. A distributed
validator bundle, Node.js runtime, `node_modules` tree, container image, or
cache must preserve license and notice obligations for every included component.

Inline Markdownlint directives are source-level validator instructions. They are
not license notices, legal waivers, or evidence that a rule is inapplicable to
other files. Exceptions must remain minimal, rule-specific, and locally earned.

The CLI documents three exit classes: successful lint with no errors, completed
lint with reported errors, and execution failure. Repository automation must not
collapse those outcomes into one success state.

## Compliance Posture

- Keep the canonical external configuration authoritative.
- Record exact CLI, rule-engine, Node.js, and dependency identities for each
  run.
- Treat markdownlint-cli2 0.23.0 as current for this dated review.
- Track markdownlint 0.41.0 as the CLI's exact dependency and recheck the
  standalone 0.41.1 drift when the CLI or lockfile changes.
- Do not override the exact engine dependency without a separately tested and
  documented compatibility decision.
- Keep all rules enabled globally and use only justified file- or line-local
  exceptions.
- Preserve the `MD052` template exception only while unresolved reference labels
  are intentional template content.
- Do not distribute an unidentified `node_modules` tree or portable runtime.
- Review automatic fixes rather than treating generated edits as legally or
  semantically authoritative.
- Reverify licenses, schemas, release compatibility, and security information
  before bundling or upgrading.

## Source References

- Anson, D. and contributors (2026) *markdownlint-cli2 npm package metadata*.
  Identifies 0.23.0 as the current package, its Node.js engine floor, package
  integrity, repository, and MIT license. Available at:
  <https://www.npmjs.com/package/markdownlint-cli2> (Accessed: 14 July 2026).
- Anson, D. and contributors (2026) *markdownlint npm package metadata*.
  Identifies 0.41.1 as the current standalone package and its publication date,
  Node.js engine floor, package integrity, repository, and MIT license.
  Available at: <https://www.npmjs.com/package/markdownlint> (Accessed: 14 July
  2026).
- Anson, D. and contributors (n.d.) *markdownlint-cli2 official GitHub
  repository*. Available at: <https://github.com/DavidAnson/markdownlint-cli2>
  (Accessed: 14 July 2026).
- Anson, D. and contributors (n.d.) *markdownlint official GitHub repository*.
  Available at: <https://github.com/DavidAnson/markdownlint> (Accessed: 14 July
  2026).
- Anson, D. and contributors (n.d.) *MD052: Reference links and images should
  use a label that is defined*. Available at:
  <https://github.com/DavidAnson/markdownlint/blob/main/doc/md052.md> (Accessed:
  14 July 2026).
- Anson, D. and contributors (n.d.) *MD053: Link and image reference definitions
  should be needed*. Available at:
  <https://raw.githubusercontent.com/DavidAnson/markdownlint/main/doc/md053.md>
  (Accessed: 14 July 2026).
- SHAR managed command authority (2026), `validate.sh`, canonical Markdown
  configuration, root package authority, lockfile, installed
  markdownlint-cli2 0.23.0 and markdownlint 0.41.0 package metadata, executable
  output, and `docs/bibliography/template.md`.
