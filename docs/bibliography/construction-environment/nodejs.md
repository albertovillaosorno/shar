# Node.js

This non-governing record documents a validation runtime and does not apply
Node.js licensing to scripts, Markdown, or packages executed by it.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The managed Node.js v25.9.0 and npm
  11.12.1 identities, repository role, official release schedule, source
  repository, and license inventory were verified. Node.js v25 is end-of-life;
  the package graph and exact binary provenance remain environment-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source JavaScript runtime and bundled dependency set.

## Covered Material

The Node.js runtime used to execute repository-managed JavaScript validation
tooling, including markdownlint-cli2.

## Repository Use And Scope

Canonical validation resolves Node.js only through the managed runtime and uses
it for Markdown and spelling validation. The reviewed runtime reports Node.js
v25.9.0 and npm 11.12.1. Node.js is a development prerequisite for those gates,
not a generated-game runtime dependency.

## Provenance And Version History

The official release schedule marks Node.js v25 end-of-life, v26 Current, and
v24 and v22 as LTS lines. It also recommends Active LTS or Maintenance LTS for
production applications. The managed v25.9.0 runtime is therefore outside the
supported lines as of 14 July 2026.

Validation evidence must preserve the exact Node.js binary, major line, support
status, architecture, npm identity, and package graph. Node.js also bundles
externally maintained libraries whose license texts are included in the project
license inventory.

## Authorship, Ownership, And Attribution

Node.js contributors, the OpenJS Foundation, and third-party component authors
retain applicable rights. SHAR source and documentation retain their independent
rights.

## License Or Terms Basis

The Node.js project license applies the MIT terms to Node.js-authored portions
and then lists externally maintained libraries with their own notices and
licenses. The exact source revision or binary distribution and its complete
license inventory control; `MIT` alone is not a sufficient description of every
bundled component.

## Distribution, Modification, And Compatibility

Running validation through Node.js does not relicense checked files. Any bundled
runtime or package cache must preserve the Node.js license and the notices for
all included libraries and packages.

## Compliance Posture

Replace the end-of-life v25 runtime with a supported line or record a narrow,
time-bounded compatibility blocker. Record the exact runtime, support status,
npm, and package versions for reproducible validation. Do not distribute an
unidentified Node.js or package cache as SHAR-authored material.

## Source References

- OpenJS Foundation and Node.js contributors (n.d.) *Node.js*. Available at:
  <https://nodejs.org/> (Accessed: 14 July 2026).
- OpenJS Foundation and Node.js contributors (2026) *Node.js Releases*.
  Identifies v25 as end-of-life, v26 as Current, and v24 and v22 as LTS lines.
  Available at: <https://nodejs.org/en/about/previous-releases> (Accessed: 14
  July 2026).
- OpenJS Foundation and Node.js contributors (n.d.) *Official GitHub
  repository*. Available at: <https://github.com/nodejs/node> (Accessed: 14 July
  2026).
- OpenJS Foundation and Node.js contributors (n.d.) *Node.js License And
  Third-Party Notices*. Available at:
  <https://github.com/nodejs/node/blob/main/LICENSE> (Accessed: 14 July 2026).
- SHAR managed runtime and command authority (2026), Node.js v25.9.0, npm
  11.12.1, runtime mapping, Markdown validation, and spelling validation.
