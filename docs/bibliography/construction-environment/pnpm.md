# pnpm

This non-governing record documents the managed JavaScript package manager used
to provision validation dependencies. It does not apply pnpm licensing to the
packages it installs, the files those packages validate, or SHAR-authored
source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Managed pnpm 11.13.0 package metadata,
  executable output, package-manager authority, lockfile use, current upstream
  release, Node.js engine floor, repository role, and MIT license were verified.
  The complete installed package graph retains component-specific notices.
- Counsel review: Not performed.
- Legal scope: Upstream licensing and the current non-distributed development
  dependency posture; no legal opinion is asserted.
- As-of date: 2026-07-14.
- Distribution posture: Root-managed development and dependency-provisioning
  tool; not included in the current SHAR distributed payload.
- Subject class: Open-source JavaScript package manager.

## Covered Material

| Component | Function | Relationship |
| :-------- | :------- | :----------- |
| `pnpm` CLI | Frozen installs and lockfile enforcement | Root authority |
| pnpm package | Package-manager implementation | Managed dependency |
| `pnpm-lock.yaml` | Exact resolution evidence | Root authority |
| Node.js | Runtime | Executes pnpm |
| Installed packages | Validation dependencies | Independent licenses |

pnpm is separate from npm. The reviewed Node.js archive bundles npm 11.17.0,
while pnpm 11.13.0 is installed and governed independently by the workspace
root. Neither package manager's license replaces the licenses of installed
packages.

## Repository Use And Scope

The managed pnpm CLI provisions the root JavaScript validation graph from the
exact `package.json` and `pnpm-lock.yaml` authorities. Bootstrap invokes it with
a frozen lockfile and disabled lifecycle scripts under command-owned cache,
store, module, and virtual-store paths. SHAR validation then consumes managed
CSpell and markdownlint-cli2 packages from that graph.

pnpm is not a generated-game runtime dependency and is not directly distributed
with SHAR. Its role is to create and verify the operator-side package state used
by repository validation.

## Provenance And Version History

The root package authority declares `pnpm@11.13.0`. Installed package metadata
and executable output report pnpm 11.13.0. The npm registry and official GitHub
release identify 11.13.0 as the current release, published 13 July 2026.

The package metadata requires Node.js `>=22.13`. The managed Node.js v26.5.0
runtime satisfies that floor. The reviewed pnpm package reports the MIT license,
the canonical pnpm repository, and this package integrity:

```text
sha512-iNlHJNjy5sGGdEpVhMblnsrIaex7oV6ctM1ijo3HBmggskgdjuO1HqjaMjpzeAaKpYxVajcg0yt8IKBR0Ig2Og==
```

The managed compatibility entry point has SHA-256
`67b035e322203961795e8e34ca63a08c37a4386eda94107fb3d28f3246d882ad`.
That executable hash identifies the observed local artifact; it is not a
substitute for package integrity, source provenance, or future currentness
checks.

## Authorship, Ownership, And Attribution

Zoltan Kochan, Rico Sta. Cruz, and other pnpm contributors retain applicable
rights in pnpm-authored material. Node.js, registry, package, and transitive
dependency authors retain independent rights in their respective components.
SHAR source, documentation, and configuration retain their independent rights.

## License Or Terms Basis

The installed package metadata and official pnpm license identify the project as
MIT-licensed. Redistribution of pnpm requires preservation of the applicable
copyright and permission notice. Packages installed by pnpm remain governed by
their own licenses and notices; a lockfile does not consolidate or replace them.

Registry terms, package signatures, provenance records, and dependency licenses
must be evaluated for the exact artifacts used. The package manager's MIT
license does not authorize redistribution of every package in its store or
module tree.

## Distribution, Modification, And Compatibility

Running pnpm does not relicense package manifests, lockfiles, source, or checked
documentation. Any distributed pnpm executable, package cache, content-addressed
store, `node_modules` tree, or container image must preserve the notices and
licenses of every included component.

pnpm 11.13.0 is compatible with the reviewed Node.js v26.5.0 runtime through its
published `>=22.13` engine floor. A later pnpm major or Node.js release requires
fresh bootstrap, lockfile, command-runner, and validation evidence rather than
an assumption of compatibility.

## Compliance Posture

- Keep one exact root `packageManager` pin and one reviewed lockfile authority.
- Use frozen installation and fail when package metadata, the pin, and the
  installed CLI disagree.
- Keep package state inside command-owned dependency and cache paths.
- Do not confuse pnpm with Node.js's bundled npm or infer one version from the
  other.
- Keep lifecycle scripts disabled unless a separately reviewed package requires
  them and the command authority owns the exception.
- Preserve package-specific licenses and notices before distributing a pnpm
  store, module tree, validator bundle, or container image.
- Reverify pnpm release currentness, Node.js compatibility, package integrity,
  and lockfile behavior before upgrading.

## Source References

- pnpm contributors (n.d.) *pnpm official website*. Available at:
  <https://pnpm.io/> (Accessed: 14 July 2026).
- pnpm contributors (2026) *pnpm v11.13.0 release*. Identifies the release and
  publication date. Available at:
  <https://github.com/pnpm/pnpm/releases/tag/v11.13.0> (Accessed: 14 July 2026).
- pnpm contributors (2026) *pnpm npm package metadata*. Identifies 11.13.0 as
  the current package, its Node.js engine floor, package integrity, repository,
  and MIT license. Available at: <https://www.npmjs.com/package/pnpm> (Accessed:
  14 July 2026).
- pnpm contributors (n.d.) *pnpm official GitHub repository*. Available at:
  <https://github.com/pnpm/pnpm> (Accessed: 14 July 2026).
- pnpm contributors (n.d.) *pnpm license*. Available at:
  <https://github.com/pnpm/pnpm/blob/main/LICENSE> (Accessed: 14 July 2026).
- SHAR managed command authority (2026), root `package.json`, `pnpm-lock.yaml`,
  bootstrap and command-runner configuration, managed pnpm 11.13.0 package
  metadata, executable output, checksum lock, and validator package graph.
