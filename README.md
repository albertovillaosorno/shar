# SHAR

SHAR is a faithful Unreal Engine 5 port workspace for rebuilding a lawful local
copy of *The Simpsons: Hit & Run* as a native, playable, and moddable project.

This is a technology migration, not a content redesign. The base project keeps
the original missions, world layout, models, textures, audio, cinematics, UI,
progression, and general gameplay structure. Assets may receive only the
technical conversion required to work correctly in Unreal Engine.

The project was previously archived because manually rebuilding the world and
correcting hundreds of assets was outside the work the author wanted to
continue.
Development now follows a narrower plan: reproduce the original game faithfully
in Unreal, finish the runtime, and provide a clean foundation for mods. Creative
changes and visual replacements belong in optional mods rather than the base
port.

The base port imports the original decoded world through source-authored FBX.
It does not replace the map with an Unreal Landscape, and it does not apply
heuristic map offsets, interior movements, global height raises, or UV mirrors.
Only the explicit source-to-FBX/Unreal axis conversion is retained.

The repository contains independently authored Rust, C++, and Python source
code, format decoders, manifests, conversion tools, validation rules,
architecture records, technical documentation, and an Unreal project shell. It
does not contain or distribute the original game, extracted game assets,
proprietary engine source, third-party media, or generated builds.

## Maximum supported local installation

The frozen `game/manifest.jsonl` remains the minimum installation baseline. A
local workspace may additionally contain every supported official language and
zero, one, or both optional packages below `game/mods/`:

- `m.lmlm`: *The Simpsons: Hit & Run Remastered*, created by Muckluck; latest
  tested version 1.0. It replaces only identities that exist in the original
  installation and skips every additional package member.
- `j.lmlm`: *The Simpsons: Hit & Run – Versión Latino*, created by Jebano;
  latest tested version 0.8. It adds only Latin-American voice and cinematic
  audio and never overwrites original or remaster output.

The package filenames are stable local aliases; release names and versions are
not hardcoded into extraction behavior. The repository provides compatibility
support only, does not include download links, and does not claim authorship of
either mod. See the [optional local mod package
contract](docs/technical/pipeline/optional-local-mod-packages.md).

Before extraction, either read-only command prints the exact supported package
changes as deterministic JSON:

```text
pipeline preview-optional-mods game extracted --no-log
pipeline dry-run-optional-mods game extracted --no-log
```

Both names are aliases. They report every replacement, addition, and skip with a
relative output path, normalized byte count, and SHA-256 evidence without
modifying the game or extraction roots.

When either supported package is present, copy the `approval_token` from the
current preview into the command that may apply it:

```text
TOKEN=<approval-token>
pipeline extract-game game extracted --approve-optional-mods "$TOKEN"
pipeline extract-game-resume game extracted --approve-optional-mods "$TOKEN"
pipeline export-lmlm game extracted --approve-optional-mods "$TOKEN"
```

The token approves the exact ordered package byte set from that preview. A
missing, malformed, stale, or package-free token fails before output mutation.
The option is rejected by read-only and unrelated commands.

Resume and package-only reapplication may repeat only the token already recorded
by extraction manifest schema v3. Adding, removing, or changing a package—or
encountering an older manifest—requires a clean `extract-game` invocation.

Complete clean and resume extraction runs rebuild all ten stages below an
isolated sibling candidate. The accepted extraction root changes only after the
candidate, minor-unit manifest, and run report all succeed. A later invocation
recovers an interrupted rename before validating package approval or continuity;
a live transaction holds an exclusive file lease and cannot be mistaken for an
abandoned run. See the [recoverable extraction publication
contract](docs/technical/pipeline/recoverable-extraction-publication.md).

## Deterministic Unreal staging

After extraction, indexing, and audit succeed, the pipeline can generate the
public-safe Unreal import manifest and aggregate plan bundle with:

```text
pipeline prepare-unreal game extracted
```

A successful run atomically publishes nine ignored files under
`unreal-staging/`: the manifest, summary, bundle index, and the six canonical
asset import, asset construction, world assembly, runtime binding, validation,
and package plans. Operations distinguish verified inputs that are ready from
FBX inputs that still require conversion and normalized JSON inputs that require
a repository-owned native editor factory. When ignored `fbx-assets/` exists,
`prepare-unreal` accepts it only as a complete exact catalog with verified
binary FBX 7.7 headers, external PNG provenance when declared, byte counts,
hashes, paths, and inventory; otherwise the entire catalog is rejected before
plan publication.

This command plans and validates work; it does not mutate the Unreal project.
Native application remains a separate transactional stage launched and observed
through tested editor automation. See the
the generated plan bundle contract documented under
`docs/technical/pipeline/unreal/generated-plan-bundle.md`.

A user supplies their own lawful local installation. The tooling validates and
decodes that installation, preserves its content deterministically, imports it
into Unreal, and builds the independently authored runtime. The project is not a
complete playable release yet.

## Project documents

- [`TODO.md`](TODO.md) contains the current task list.
- [`ROADMAP.md`](ROADMAP.md) contains the project phases, dates, and progress.
- [`AGENTS.md`](AGENTS.md) contains guidance for AI agents.
- [`docs/adr/index.md`](docs/adr/index.md) contains architecture decisions.
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [`docs/technical/index.md`](docs/technical/index.md) contains technical specifications. <!-- markdownlint-disable-line MD013 -->
- [`docs/legal/index.md`](docs/legal/index.md) contains legal research and
scope notes.
- [`skills/`](skills/) contains task guidance and Unreal MCP documentation.

## Legal boundary

This is an independent interoperability and reimplementation project. It is not
affiliated with or endorsed by the original publishers, developers, licensors,
platform holders, Epic Games, or any other third party named in the repository.
Users are responsible for obtaining a lawful game copy and complying with
applicable licenses and local law.

Repository-owned material is available under the MIT License in
[`LICENSE-MIT`](LICENSE-MIT). Third-party names, software, formats, assets, and
documentation remain governed by their respective owners.
