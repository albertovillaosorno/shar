# CSpell

This non-governing record documents the CSpell command-line spell checker and
its repository-curated dictionaries. It does not make CSpell an authority for
technical, factual, or legal correctness.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Official identity, installed 10.0.0
  package metadata, repository use, upstream 10.0.1 release, and the current
  distribution boundary were verified. No documented compatibility hold or
  other repository reason for retaining the older patch was identified.
- Counsel review: Not performed.
- Legal scope: Upstream licensing and the current non-distributed development
  dependency posture; no legal opinion is asserted.
- As-of date: 2026-07-14.
- Distribution posture: Development and validation dependency.
- Subject class: Open-source spelling validator for source and documentation.

## Covered Material

| Component | Function | Repository relationship |
| :-------- | :------- | :---------------------- |
| `cspell` | Command-line spelling validation | Invoked by `validate.sh` |
| CSpell libraries | Parsing, dictionaries, and configuration | Loaded by the CLI |
| Bundled dictionaries | General and technical vocabulary | Enabled selectively |
| Repository wordlists | Project-reviewed vocabulary | Authored under `docs/cspell` |
| Node.js | JavaScript runtime | Executes the CLI |

The repository wordlists are SHAR-authored configuration data. Their inclusion
in a CSpell run does not transfer ownership to CSpell or apply CSpell's license
to independently authored repository material.

## Repository Use And Scope

`validate.sh` invokes the repository-managed Node.js runtime and CSpell module
with the repository-local `cspell.json`, disabled configuration search, an
explicit file list, directive validation, and no CSpell-owned cache. The
repository validator separately enforces dictionary classification, sorted and
unique entries, and exact line-scoped suppressions for intentional invalid
sequences.

The repository-managed package observed for this record is `cspell` 10.0.0. The
official upstream repository identifies CSpell as a spell checker for code,
lists its CLI and supporting packages, and identifies the project as
MIT-licensed. The official release page identifies 10.0.1 as the latest release
and dates it 31 May 2026. The managed installation is therefore one patch release
behind the verified upstream state.

## Provenance And Version History

Repository configuration and installed package metadata establish the CSpell
10.0.0 identity and current SHAR role. The official website, repository, and
release evidence establish the 10.0.1 currentness comparison as of 14 July 2026.
Those values are dated observations, not permanent latest-version labels or an
undocumented compatibility range.

A successful spelling run proves observed execution with 10.0.0; it does not
make that patch current or establish a reason for retaining it. Version currency
remains unresolved until the package is upgraded or a separately documented
compatibility hold supplies evidence for the older patch.

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
- The repository-managed CSpell 10.0.0 installation executes the current
  spelling contract, but it is not represented as current while upstream 10.0.1
  remains the verified latest release and no compatibility hold is documented.

## Compliance Posture

- Keep `cspell.json` as the local configuration authority.
- Keep genuine English, technical, and named-entity vocabulary separated.
- Do not use broad ignore lists to hide spelling errors.
- Use exact line-scoped suppressions for intentional invalid sequences.
- Reverify package, dictionary, and dependency notices before redistribution.
- Treat spelling acceptance as editorial evidence, not factual or legal proof.

## Source References

- Street Side Software and contributors (n.d.) *CSpell official website*.
  Available at: <https://cspell.org/> (Accessed: 14 July 2026).
- Street Side Software and contributors (2026) *CSpell releases*. Version
  10.0.1 is identified as the latest release and dated 31 May 2026. Available
  at: <https://github.com/streetsidesoftware/cspell/releases> (Accessed: 14
  July 2026).
- Street Side Software and contributors (n.d.) *CSpell official GitHub
  repository*. Available at: <https://github.com/streetsidesoftware/cspell>
  (Accessed: 14 July 2026).
- SHAR repository (2026) `cspell.json`, `docs/cspell`, `validate.sh`, and
  repository-managed `cspell` 10.0.0 package metadata.
