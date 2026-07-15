# Node.js

This non-governing record documents a validation runtime and does not apply
Node.js licensing to scripts, Markdown, or packages executed by it.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The managed Node.js v26.5.0 and bundled
  npm 11.17.0 identities, executable and archive hashes, Node.js support
  schedule, npm current-major engine constraint, repository role, source
  repositories, and license inventories were verified. The complete package
  graph remains environment-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source JavaScript runtime and bundled dependency set.

## Covered Material

The Node.js runtime used to execute repository-managed JavaScript validation
tooling, including markdownlint-cli2. The independently installed package
manager is documented separately in [pnpm](pnpm.md); it is not bundled with
Node.js or governed by the bundled npm version.

## Repository Use And Scope

Canonical validation resolves Node.js only through the managed runtime and uses
it for Markdown and spelling validation. The reviewed runtime reports Node.js
v26.5.0 and bundled npm 11.17.0. The managed pnpm 11.13.0 package manager runs
on the same Node.js runtime but has its own exact pin, package metadata,
lockfile role, license, and bibliography record. Node.js is a development
prerequisite for those gates, not a generated-game runtime dependency.

## Provenance And Version History

The official release schedule marks Node.js v26 Current, v24 and v22 as LTS
lines, and v25 end-of-life. It recommends Active LTS or Maintenance LTS for
production applications. The managed v26.5.0 runtime matches the latest Current
release as of 14 July 2026; that currentness does not make it the production-LTS
choice.

The official Node.js v26.5.0 Windows x64 archive has SHA-256
`d3b2277dbcccfdf24ef6302928f64f484cff1d77a6d3caa3a28f4d20ce9158f6`.
The managed `node.exe` has SHA-256
`119d6fa70e6ae1b15b90688ab6bcc8e3a2819acea021af196895cab1843645af`.
The archive bundles npm 11.17.0.

The npm registry identifies 12.0.1 as the current npm release and requires
Node.js `^22.22.2`, `^24.15.0`, or `>=26.0.0`, so the managed Node.js 26 runtime
is eligible for that major. The registry identifies 11.18.0 as the newest npm
11 release. Bundled npm 11.17.0 is therefore one compatible patch release behind
the newest npm 11 and one major behind the registry's current release.

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
licenses. The npm package metadata reports the Artistic License 2.0 for npm;
Node.js licensing must not be substituted for npm or its package graph. pnpm
reports the MIT license but remains a separately installed subject whose terms
do not replace npm, Node.js, or installed-package notices. The exact source
revisions or binary distributions and their complete license inventories
control; `MIT` alone is not a sufficient description of every bundled component.

## Distribution, Modification, And Compatibility

Running validation through Node.js does not relicense checked files. Any bundled
runtime or package cache must preserve the Node.js license and the notices for
all included libraries and packages.

## Compliance Posture

Keep the managed Node.js runtime aligned with the governed current release, or
document a narrow, time-bounded compatibility hold when a later release appears.
For production application guidance, preserve Node.js's separate LTS-only
recommendation instead of treating the operator toolchain as production policy.

Upgrade bundled npm 11.17.0 to the newest compatible npm 11 release or adopt npm
12 only through an explicit managed-package authority that preserves its engine
constraint and reproducible package graph. Keep pnpm under its separate exact
pin and do not infer its version, license, or package state from bundled npm.
Record the exact runtime, support status, npm identity, pnpm identity, engine
constraints, and package versions for reproducible validation. Do not distribute
an unidentified Node.js, npm, or pnpm package cache as SHAR-authored material.

## Source References

- OpenJS Foundation and Node.js contributors (n.d.) *Node.js*. Available at:
  <https://nodejs.org/> (Accessed: 14 July 2026).
- OpenJS Foundation and Node.js contributors (2026) *Node.js Releases*.
  Identifies v26.5.0 as the latest release, v26 as Current, v25 as end-of-life,
  and v24 and v22 as LTS lines. Available at:
  <https://nodejs.org/en/about/previous-releases> (Accessed: 14 July 2026).
- OpenJS Foundation and Node.js contributors (2026) *Node.js v26.5.0 binary
  distribution and checksums*. Publishes the Windows x64 archive and its
  SHA-256 identity. Available at: <https://nodejs.org/dist/v26.5.0/> (Accessed:
  14 July 2026).
- npm contributors (2026) *npm registry package metadata*. Identifies npm 12.0.1
  as `latest`, npm 11.18.0 as the newest npm 11 release, and their respective
  Node.js engine ranges and Artistic License 2.0. Available at:
  <https://www.npmjs.com/package/npm> (Accessed: 14 July 2026).
- OpenJS Foundation and Node.js contributors (n.d.) *Official GitHub
  repository*. Available at: <https://github.com/nodejs/node> (Accessed: 14 July
  2026).
- OpenJS Foundation and Node.js contributors (n.d.) *Node.js License And
  Third-Party Notices*. Available at:
  <https://github.com/nodejs/node/blob/main/LICENSE> (Accessed: 14 July 2026).
- SHAR managed runtime and command authority (2026), Node.js v26.5.0, bundled
  npm 11.17.0, reviewed archive and executable hashes, runtime mapping, Markdown
  validation, spelling validation, and npm registry queries for current and
  compatible release evidence.
