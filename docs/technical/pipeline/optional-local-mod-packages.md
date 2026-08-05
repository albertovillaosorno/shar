# Optional local mod packages

- Status: Active
- Last reviewed: 2026-08-05

## Governing decision

- Current owning ADR: [Voice and language mod
  packages](../../adr/modding/voice-language-modding-suite.md)

## Purpose

The extraction pipeline supports two independently supplied optional LMLM
packages without changing the minimum source-installation contract. The current
local installation snapshot may contain all supported official language data,
other lawful local additions, both optional packages, either package, or no
optional package.

The repository provides interoperability support only. It does not include,
mirror, retrieve, distribute, or claim authorship of either package.

## Repository model

Optional packages are discovered only as direct files in `game/mods/`. Stable
local aliases select behavior:

<!-- markdownlint-disable MD013 -->
| Alias | Supported package | Creator | Latest tested version | Role |
| --- | --- | --- | --- | --- |
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: the source-credit table row is indivisible -->
| `m.lmlm` | *The Simpsons: Hit & Run Remastered* | Muckluck | 1.0 | Existing-file remaster overlay |
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: the source-credit table row is indivisible -->
| `j.lmlm` | *The Simpsons: Hit & Run – Versión Latino* | Jebano | 0.8 | Isolated Latin-American audio |
<!-- markdownlint-restore MD013 -->

Release titles and version strings are documentation evidence, not executable
identifiers. Updating a locally supplied package does not require code changes
when its alias and supported behavior remain unchanged.

Both package containers are parsed and structurally validated before any package
member is applied. When both aliases are present, the remaster role is evaluated
first and the Latino role second.

### Remaster role

The remaster role removes its loader-specific package wrapper and maps each
candidate member to its corresponding source-relative identity. A member is
written only when that identity already exists in the unmodified installation or
in the extraction produced from it. The resulting bytes then pass through the
same RSD, RMV, P3D, script, and other normalization stages as original files.

Members without an original identity are skipped. They are not copied into the
extraction tree and cannot become implicit base-game additions.

### Latino role

The Latino role accepts only voice RSD members and cinematic RMV or BIK members
inside the package's custom-file namespace. Voice audio becomes deterministic
PCM WAV. Cinematics contribute only their first audio stream, also as PCM WAV.

Latino output remains isolated below the generated optional-language root. It
never replaces original or remaster output. Package configuration, loading art,
launcher resources, and every other non-audio member are skipped.

## Invariants

- `game/manifest.jsonl` remains the frozen minimum installation baseline.
- The optional package set is exactly none, `m.lmlm`, `j.lmlm`, or both.
- Unknown LMLM filenames fail closed instead of receiving inferred behavior.
- Package titles and versions never select executable behavior.
- Remaster members can replace only identities present in the unmodified game.
- Remaster members that would add a new identity are skipped.
- Latino members can add only isolated voice or cinematic-audio WAV output.
- Latino output never overwrites an existing file.
- All accepted package paths remain relative, normalized, collision-free, and
  bounded by the generated extraction root.
- Generated records contain relative paths, byte counts, roles, and SHA-256
  evidence without publishing local package locations.

## Read-only preview

Use either canonical command to inspect the supported local package set before
running extraction:

```text
pipeline preview-optional-mods game extracted --no-log
pipeline dry-run-optional-mods game extracted --no-log
```

The two command names are aliases and produce byte-identical JSON. The document
uses schema `shar-schoenwald.optional-mod-preview.v1` and records every package
member in deterministic package-table order. Each member reports its alias,
role, package-relative source identity, action, reason, predicted output path,
normalized byte count, and SHA-256 digest when it would write output.

The action is one of `replace`, `add`, or `skip`. Skipped members have no output
path or digest and carry a stable policy reason. Paths remain
repository-relative
and the document never exposes the local package location or payload bytes.

Preview is read-only with respect to the game and extraction roots. Voice media
is normalized in memory. Cinematic audio is decoded only in a temporary working
directory so that its predicted byte count and digest match extraction exactly;
the temporary directory is removed before command completion.

The maximum supported local snapshot was previewed on 2026-08-05. Both aliases
reported 1,572 package members: 1,530 writes and 42 skips. Every predicted write
matched the existing extraction manifest exactly by source identity, output,
normalized byte count, and SHA-256 digest. A complete before-and-after hash of
the optional extraction tree was unchanged.

## Failure behavior

An invalid container, unsafe path, unsupported root control, duplicate identity,
case-insensitive collision, unsupported alias, symlinked package, escaped
output,
or attempted Latino overwrite fails before successful stage completion.

All packages are parsed before package writes begin. The full extraction remains
a clean generated build and is regenerated from the source installation when a
package set changes.

## Verification

The policy is covered by unit tests for no package, either alias, both
aliases,
unknown aliases, existing-only remaster replacement, skipped remaster additions,
Latino media classification, read-only empty previews, canonical CLI aliases,
and
extra-argument rejection.

The maximum supported local snapshot was extracted successfully on 2026-08-04
with all locally supplied official languages and both tested packages. The run
completed all ten extraction stages and produced 301,799 normalized files. The
optional-mod stage recorded:

- 29 remaster replacements and 3 skipped remaster members;
- 1,497 Latino voice WAV files;
- 4 Latino cinematic-audio WAV files; and
- 39 skipped Latino non-audio members.

An independent inventory comparison found zero removed base paths and zero
unauthorized remaster additions. The normalized-output audit accepted all
137,242 audited minor units after deterministic metadata classification.

## Known limits

Only the two documented roles and aliases are supported. This contract does not
install packages, provide acquisition instructions, support arbitrary loader
features, preserve package-specific configuration UI, or infer behavior for
other LMLM files.
