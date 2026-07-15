# CSpell

This non-governing record documents the CSpell command-line spell checker and
its repository-curated dictionaries. It does not make CSpell an authority for
technical, factual, or legal correctness.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Official identity, installed 10.0.1
  package metadata, repository use, current upstream release, and the current
  distribution boundary were verified. The complete transitive package and
  bundled-dictionary notice inventory remains distribution-specific.
- Counsel review: Not performed.
- Legal scope: Upstream licensing and the current non-distributed development
  dependency posture; no legal opinion is asserted.
- As-of date: 2026-07-14.
- Distribution posture: Development and validation dependency.
- Subject class: Open-source spelling validator for source and documentation.

## Covered Material

| Component | Function | Relationship |
| :-------- | :------- | :----------- |
| `cspell` | Spelling validation | Invoked by `validate.sh` |
| CSpell libraries | Parsing and configuration | CLI dependencies |
| Bundled dictionaries | Vocabulary | Enabled selectively |
| Repository wordlists | Project-reviewed vocabulary | `docs/cspell` authority |
| Node.js | Runtime | Executes the CLI |

The repository wordlists are SHAR-authored configuration data. Their inclusion
in a CSpell run does not transfer ownership to CSpell or apply CSpell's license
to independently authored repository material.

## Repository Use And Scope

`validate.sh` invokes the managed Node.js runtime and CSpell module with the
repository-local `cspell.json`, disabled configuration search, an explicit file
list, directive validation, and no CSpell-owned cache. The repository validator
separately enforces dictionary classification, sorted and unique entries, and
exact line-scoped suppressions for intentional invalid sequences.

The managed package observed for this record is `cspell` 10.0.1. Its package
metadata requires Node.js `>=22.18.0`, reports the MIT license, and identifies
the canonical CSpell repository. The managed Node.js v26.5.0 runtime satisfies
that engine floor. The npm registry and official upstream release evidence also
identify 10.0.1 as the current release as of 14 July 2026.

## Provenance And Version History

Root package authority, the lockfile, installed package metadata, and
executable output establish the CSpell 10.0.1 identity used by SHAR validation.
The official
website, repository, release page, and npm registry establish the dated
currentness comparison.

A successful spelling run proves observed execution with 10.0.1 and the reviewed
configuration. It does not prove factual correctness, establish the completeness
of every bundled dictionary notice, or make a dated version permanently current.

## Authorship, Ownership, And Attribution

CSpell and its upstream packages are authored by their respective upstream
authors and contributors. Bundled or optional dictionaries may have separate
authors and notices. SHAR owns its independently authored `cspell.json` and
curated wordlists; loading those files into CSpell does not transfer ownership
or apply CSpell's license to unrelated repository material.

## License Or Terms Basis

The installed `cspell` package metadata reports the MIT license, and the
canonical upstream repository presents an MIT license. Bundled dictionaries,
transitive packages, optional extensions, and separately installed add-on
dictionaries may have independent notices and must be reviewed from the exact
installed package graph before redistribution.

External validation does not relicense checked source, documentation, or
wordlists. Distribution of Node.js, CSpell, bundled dictionaries, or a complete
package tree requires preservation of the applicable notices for the exact
artifacts distributed.

## Distribution, Modification, And Compatibility

- SHAR does not bundle Node.js, CSpell, its package tree, or third-party add-on
  dictionaries in the repository or current distributed payload.
- The repository distributes only its own `cspell.json` and curated wordlists.
- A CSpell package, transitive dependency, and bundled-dictionary notice
  inventory is outside the current SHAR distribution boundary and is required
  when executable or package contents are redistributed.
- The managed CSpell 10.0.1 package matches the current reviewed release and its
  Node.js engine requirement is satisfied by the managed runtime.

## Compliance Posture

- Keep `cspell.json` as the local configuration authority.
- Keep genuine English, technical, and named-entity vocabulary separated.
- Do not use broad ignore lists to hide spelling errors.
- Use exact line-scoped suppressions for intentional invalid sequences.
- Reverify package, dictionary, dependency notices, and release currentness
  before redistribution or a managed upgrade.
- Treat spelling acceptance as editorial evidence, not factual or legal proof.

## Source References

- Street Side Software and contributors (n.d.) *CSpell official website*.
  Available at: <https://cspell.org/> (Accessed: 14 July 2026).
- Street Side Software and contributors (2026) *CSpell releases*. Version
  10.0.1 is identified as the latest release and dated 31 May 2026. Available
  at: <https://github.com/streetsidesoftware/cspell/releases> (Accessed: 14
  July 2026).
- Street Side Software and contributors (2026) *CSpell npm package metadata*.
  Identifies `cspell` 10.0.1, its Node.js engine floor, package integrity,
  repository, and MIT license. Available at:
  <https://www.npmjs.com/package/cspell> (Accessed: 14 July 2026).
- Street Side Software and contributors (n.d.) *CSpell official GitHub
  repository*. Available at: <https://github.com/streetsidesoftware/cspell>
  (Accessed: 14 July 2026).
- SHAR managed command authority (2026), `cspell.json`, `docs/cspell`,
  `validate.sh`, root package authority, lockfile, and managed `cspell` 10.0.1
  package metadata and executable output.
