# Node.js

This non-governing record documents a validation runtime and does not apply
Node.js licensing to scripts, Markdown, or packages executed by it.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository role, official release
  policy, source repository, and license inventory verified; the exact local
  runtime and package graph remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Open-source JavaScript runtime and bundled dependency set.

## Covered Material

The Node.js runtime used to execute repository-managed JavaScript validation
tooling, including markdownlint-cli2.

## Repository Use And Scope

`validate.sh` records the Node.js version as toolchain identity and invokes a
Node.js-based Markdown validator. Node.js is a development prerequisite for that
gate, not a generated-game runtime dependency.

## Provenance And Version History

Validation evidence must preserve the exact Node.js binary, major line, support
status, architecture, and package graph. The current Node.js policy
distinguishes Current, Active LTS, Maintenance LTS, and end-of-life lines and
recommends LTS
lines for production use. Node.js also bundles externally maintained libraries
whose license texts are included in the project license inventory.

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

Record the exact runtime and package versions for reproducible validation. Do
not distribute an unidentified Node.js or package cache as SHAR-authored
material.

## Source References

- OpenJS Foundation and Node.js contributors (n.d.) *Node.js*. Available at:
  <https://nodejs.org/> (Accessed: 13 July 2026).
- OpenJS Foundation and Node.js contributors (2026) *Node.js Releases*.
  Available at: <https://nodejs.org/en/about/previous-releases> (Accessed: 13
  July 2026).
- OpenJS Foundation and Node.js contributors (n.d.) *Official GitHub
  repository*. Available at: <https://github.com/nodejs/node> (Accessed: 13 July
  2026).
- OpenJS Foundation and Node.js contributors (n.d.) *Node.js License And
  Third-Party Notices*. Available at:
  <https://github.com/nodejs/node/blob/main/LICENSE> (Accessed: 13 July 2026).
- SHAR repository (2026) `validate.sh` and Markdown validation tooling.
