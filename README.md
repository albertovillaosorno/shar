# SHAR

SHAR rebuilds a lawful local copy of *The Simpsons: Hit & Run* as a native
Unreal Engine game. This repository does not include the original game or its
assets.

The project is still in development. The build workflow below is the intended
supported user flow; some `tools/build/` commands are not implemented yet.

## How to install

1. Buy or use a lawful copy of *The Simpsons: Hit & Run* and install it
   normally.
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

3. Install **CPython 3.14.6** from the official Python release page:
   <https://www.python.org/downloads/release/python-3146/>.

The supported workflow uses that exact version. Newer Python releases may work,
but they are not guaranteed to be compatible with the repository pin.

4. Install **Unreal Engine 5.8.1** from
   [Unreal Engine](https://www.unrealengine.com/).

With the Epic Games Launcher, open **Unreal Engine > Library**, select the
**plus (+)** beside **Engine Versions**, open the new tile's version dropdown,
choose **5.8.1** when that exact hotfix is available, then select **Install**.

The launcher does not always expose every historical hotfix as a separate
choice. **5.8.1 is the preferred repository target.** Another **5.8.x** may
work, but that is unsupported and at your own risk. While SHAR is actively
maintained, its tooling may move to a newer Unreal version. You are also welcome
to fork the repository or update your local copy for a newer engine yourself.

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
- Linux AMD64 / x86-64 (64-bit PC)
- macOS ARM64
- Windows ARM64
- Windows AMD64 / x86-64 (64-bit PC)

The selector will mark the exact host match with a note such as
`← this is your system`; it will not auto-select it or hide the other targets.
Select at least one target or select all of them, then choose **Save**. The
selection is written to:

```text
.cache/build/data/arch.json
```

The iOS target produces a local IPA only. There is no App Store submission or
iOS installation tutorial here; unavoidable Apple signing or build-host
requirements still apply.

**tvOS** and **macOS Intel / x86-64** are not planned SHAR targets. macOS ARM64
remains planned. If either unsupported target gets substantial issue traction
and contributors can help test it, issues and pull requests are welcome.

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

### Optional Windows desktop shortcut

After a successful Windows build, you can optionally run:

```text
python tools/build/windows_shortcut.py
```

The Windows-only helper asks whether you want to create a desktop shortcut. It
does nothing unless you approve the prompt. If automatic discovery is
ambiguous, pass the packaged executable with `--target`.

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

## Validation while Jig is in development

Jig is SHAR's canonical repository validator, but Jig is still in active
development and is not yet a public dependency that contributors can assume is
installed. The tracked `.jig/` files remain the repository's validation
configuration.

If you have a development Jig source/dependency snapshot, keep it in the
repository-owned location expected by the current config:

```text
.dependencies/jig/source
```

Do not install random global substitutes for the pinned tools. If Jig itself is
unavailable, run the external gates that can be reproduced without it:

```text
python tools/vwj/main.py check
```

VWJ reads the tracked `.jig/jig.toml` tool commands and configs, but it does
**not** pretend to implement Jig-native graph, ownership, header, or
architecture rules. You can also run the configured linters directly with
the tracked files under `.jig/`.

To install the local commit-message hook, run:

```text
python tools/vwj/main.py install-hook
```

That hook uses Jig when it is available on `PATH`; otherwise it falls back to
VWJ's portable commit-message validation. Issues and pull requests are welcome.

## Project documents

- [`TODO.md`](TODO.md) contains the current task list.
- [`ROADMAP.md`](ROADMAP.md) contains the project phases, dates, and progress.
- [`AGENTS.md`](AGENTS.md) contains guidance for AI agents.
- [`docs/adr/index.md`](docs/adr/index.md) contains architecture decisions.
- [`docs/technical/index.md`](docs/technical/index.md) contains technical
  specifications.
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
