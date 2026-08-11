# SHAR

SHAR rebuilds a lawful local copy of *The Simpsons: Hit & Run* as a native
Unreal Engine game. This repository does not include the original game or its
assets.

The project is still in development. The build workflow below is the intended
supported user flow; some `tools/build/` commands are not implemented yet.

## How to install

1. Buy or use a lawful copy of *The Simpsons: Hit & Run* and install it normally.
2. Copy the **contents** of the installed game directly into this repository's
   `game/` directory.

The supported layout is:

```text
game/Simpsons.exe
game/art/...
game/scripts/...
...
```

Do not add another directory layer. This is unsupported:

```text
game/SomeFolder/Simpsons.exe
```

3. Install **CPython 3.14.6**.

The supported workflow uses that exact version. Newer Python releases may work,
but they are not guaranteed to be compatible with the repository pin.

4. Install **Unreal Engine 5.8.1**.

Other 5.8.x releases may work, but they are unsupported and may require manual
changes.

5. From the repository root, install the project dependencies:

```text
python tools/build/dependencies.py
```

The supported bootstrap keeps project dependencies in repository-owned
locations instead of modifying global packages.

6. Check the installation:

```text
python tools/build/check.py
```

A successful check writes:

```text
.cache/build/data/check.json
```

That file records the validated game, Python, Unreal Engine, and host paths used
by later build steps. You can edit it manually, but manual overrides are
unsupported and are revalidated before use.

7. Choose the build targets:

```text
python tools/build/arch.py
```

A small checklist window will offer the supported targets:

- Android ARM64 — APK
- iOS ARM64 — IPA
- Linux ARM64
- Linux AMD64
- macOS ARM64
- Windows ARM64
- Windows AMD64

Select at least one target or select all of them, then choose **Save**. The
selection is written to:

```text
.cache/build/data/arch.json
```

The iOS target produces a local IPA only. There is no App Store submission or
iOS installation tutorial here; unavoidable Apple signing or build-host
requirements still apply.

8. Build the selected targets:

```text
python tools/build/run.py
```

Successful builds are published under:

```text
dist/<ARCH>/
```

Each directory contains the minimal native deliverable for that target. Copy the
builds wherever you want; once you no longer need the source workspace, you can
delete the repository copy after preserving anything you want from `dist/`.

## One-command flow

The optional convenience command will run the supported steps in order and keep
the same saved JSON decisions:

```text
python tools/build/auto.py
```

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

## Project documents

- [`TODO.md`](TODO.md) contains the current task list.
- [`ROADMAP.md`](ROADMAP.md) contains the project phases, dates, and progress.
- [`AGENTS.md`](AGENTS.md) contains guidance for AI agents.
- [`docs/adr/index.md`](docs/adr/index.md) contains architecture decisions.
- [`docs/technical/index.md`](docs/technical/index.md) contains technical specifications.
- [`docs/legal/index.md`](docs/legal/index.md) contains legal research and
scope notes.
- [`skills/`](skills/) contains task guidance and Unreal MCP documentation.

## Legal

SHAR is an independent interoperability and reimplementation project. It is not
affiliated with or endorsed by the original publishers, developers, licensors,
platform holders, or Epic Games. You are responsible for obtaining a lawful
copy of the game and complying with applicable licenses and local law.

Repository-owned material is available under the MIT License in
[`LICENSE-MIT`](LICENSE-MIT).
