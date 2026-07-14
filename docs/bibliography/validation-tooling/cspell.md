# CSpell

This non-governing record documents the CSpell command-line spell checker and
its repository-curated dictionaries. It does not make CSpell an authority for
technical, factual, or legal correctness.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Official identity, installed package metadata,
  repository use, and current distribution boundary verified.
- Counsel review: Not performed.
- Legal scope: Upstream licensing and the current non-distributed development
  dependency posture; no legal opinion is asserted.
- As-of date: 2026-07-13.
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

The installed package observed for this record is `cspell` 10.0.0. The official
upstream repository identifies CSpell as a spell checker for code, lists its CLI
and supporting packages, and identifies the project as MIT-licensed. The
upstream release page showed 10.0.1 as the latest release on the review date;
the local installation therefore was one patch release behind that observed
upstream state.

## Provenance And Version History

Repository configuration, installed package metadata, and the observed local
validation environment establish the CSpell 10.0.0 identity and current SHAR
role. The official website, repository, and release evidence establish upstream
identity and the 10.0.1 currentness comparison as of 13 July 2026. Those values
are dated observations, not permanent latest-version labels or an undocumented
compatibility range.

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
- The observed local CSpell 10.0.0 installation is accepted for the current
  validation contract; compatibility remains enforced by validation results,
  not by an undocumented version range.

## Compliance Posture

- Keep `cspell.json` as the local configuration authority.
- Keep genuine English, technical, and named-entity vocabulary separated.
- Do not use broad ignore lists to hide spelling errors.
- Use exact line-scoped suppressions for intentional invalid sequences.
- Reverify package, dictionary, and dependency notices before redistribution.
- Treat spelling acceptance as editorial evidence, not factual or legal proof.

## Source References

- Street Side Software and contributors (n.d.) *CSpell official website*.
  Available at: <https://cspell.org/> (Accessed: 13 July 2026).
- Street Side Software and contributors (n.d.) *CSpell official GitHub
  repository*. Available at: <https://github.com/streetsidesoftware/cspell>
  (Accessed: 13 July 2026).
- SHAR repository (2026) `cspell.json`, `docs/cspell`, `validate.sh`, and
  installed `cspell` package metadata.
