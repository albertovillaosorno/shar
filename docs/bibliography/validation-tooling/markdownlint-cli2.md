# markdownlint-cli2 And markdownlint

This non-governing record documents the Markdown command-line validator and its
rule engine without applying their licenses to checked documentation, reference
definitions, configuration files, or authored prose.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository use, installed
  markdownlint-cli2 0.22.1 and markdownlint 0.40.0 package metadata, official tag
  histories, and upstream licenses were verified. The rule engine matched its
  reviewed latest tag, while the CLI remained one minor release behind 0.23.0;
  no documented compatibility hold was identified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: Development and validation dependencies.
- Subject class: Open-source Markdown command-line interface and lint library.

## Covered Material

This related validation family contains distinct components:

| Component                   | Function                                   | Repository relationship          |
| :-------------------------- | :----------------------------------------- | :------------------------------- |
| `markdownlint-cli2`         | Configuration-based command-line interface | Invoked by `validate.sh`         |
| `markdownlint`              | Markdown/CommonMark rule engine            | Loaded by the CLI package        |
| Node.js                     | JavaScript runtime                         | Executes the CLI and rule engine |
| Parser and utility packages | Parsing, globbing, schemas, and support    | Transitive dependency graph      |

The CLI, rule engine, runtime, and transitive packages retain their own release,
authorship, security, and license evidence even when installed together.

## Repository Use And Scope

`validate.sh` invokes the repository-managed portable Node.js runtime and
`markdownlint-cli2` module with `--no-globs`, an explicit canonical
configuration path, and an enumerated Markdown file list. The canonical
configuration enables the complete rule set and pins deterministic style
options.

The checked Markdown remains authored repository material. Running the validator
does not transfer ownership, create a derivative license from the tool, or make
the tool an authority for legal conclusions in the prose.

The bibliography template uses brace-delimited prompts instead of unresolved
reference labels. MD052 therefore remains enabled without a file-wide
suppression, while MD053 continues to reject unused reference definitions. Real
links, definitions, inline code, and task-list markers retain normal Markdown
syntax.

## Provenance And Version History

The reviewed managed package tree contained markdownlint-cli2 0.22.1 and
markdownlint 0.40.0. The official tag histories identify markdownlint-cli2
0.23.0, dated 1 July 2026, and markdownlint 0.40.0, dated 4 December 2025, as
their newest tags on 14 July 2026. The rule engine therefore matched the reviewed
upstream tag, while the CLI lagged by one minor release.

Exact run identities remain established by installed package metadata, lock or
package-tree evidence, runtime output, canonical configuration, and validation
logs. Rule behavior, aliases, parser behavior, automatic fixes, configuration
schemas, and exit-code handling may change across releases. An older component
may be justified by runtime compatibility or a deliberate stability hold, but
no such reason was located for the observed CLI lag.

## Authorship, Ownership, And Attribution

David Anson and contributors retain applicable rights in markdownlint-cli2,
markdownlint, and their documentation. Node.js, parser, globbing, schema, and
other dependency contributors retain independent rights in their components.
SHAR contributors retain rights in independently authored Markdown and
configuration subject to the repository license.

## License Or Terms Basis

The official markdownlint-cli2 and markdownlint repositories identify both
projects as MIT-licensed. Redistribution requires preservation of the applicable
copyright and permission notices for each project. The complete installed
Node.js package graph may contain additional licenses and notices that must be
inventoried from the exact lockfile and package contents.

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
- Upgrade markdownlint-cli2 or document the exact compatibility hold before
  treating the managed CLI as current.
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

- Anson, D. and contributors (2026) *markdownlint-cli2 tags*. The newest tag is
  `v0.23.0`, dated 1 July 2026. Available at:
  <https://github.com/DavidAnson/markdownlint-cli2/tags> (Accessed: 14 July
  2026).
- Anson, D. and contributors (n.d.) *markdownlint-cli2 official GitHub
  repository*. Available at: <https://github.com/DavidAnson/markdownlint-cli2>
  (Accessed: 14 July 2026).
- Anson, D. and contributors (2025) *markdownlint tags*. The newest tag is
  `v0.40.0`, dated 4 December 2025. Available at:
  <https://github.com/DavidAnson/markdownlint/tags> (Accessed: 14 July 2026).
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
- SHAR repository (2026) `validate.sh`, canonical Markdown configuration,
  installed markdownlint-cli2 0.22.1 and markdownlint 0.40.0 package metadata,
  and `docs/bibliography/template.md`.
