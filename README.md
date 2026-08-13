# SHAR

<!-- CSpell:ignore apk ipa ios Paks resumability sideload uninst -->

SHAR rebuilds a lawful local copy of *The Simpsons: Hit & Run* as a native
Unreal Engine game. This repository does not include the original game or its
assets.

> The lightweight player flow described below is the intended final product.
> Parts are still pending. Every unfinished surface names its owning TODO so the
> README can describe the destination without pretending unfinished code exists.

## How the finished game will look

A finished desktop SHAR export is a portable game directory. A Windows
AMD64 package is expected to have this general shape:

```text
The Simpsons Hit & Run/
├── shar.exe
├── Engine/                         # only packaged Unreal runtime files required
├── shar/
│   ├── Binaries/Win64/
│   │   ├── shar-Win64-Shipping.exe
│   │   └── <runtime dependencies selected by packaging>
│   ├── Content/Paks/               # cooked game content
│   └── <other packaged runtime data>
├── mods/                           # SHAR mods: folder or .zip
├── NOTICES.txt
└── direct-access-desktop.ps1       # optional convenience helper
```

Linux and macOS use the same portable-product idea with their native executable
or application bundle. Android and iOS publish package files instead of desktop
folders.

The exact semantic contract will live in `game/manifest/dist.json`; it will not
hardcode every incidental Unreal DLL filename.

**TODO: Define the portable `dist/` layout and `game/manifest/dist.json`.**

## How to play — quick and easy, without installing heavy development tools

> This player flow is not finished yet.
>
> **TODO: Build the lightweight `src/user` exporter and cross-platform GUI.**

The most important rule is absolute: **SHAR never writes to, overwrites, cleans,
or patches the original installed-game directory.** The original installation is
read-only input. Generated state and exports belong to the extracted SHAR tool
and its own `dist/` directory.

The reconstruction/export tool itself runs on a desktop-class computer. Mobile
outputs are built from that lawful computer-side source workflow and then moved
to the phone/tablet; SHAR does not expect the original PC game installation to
exist on Android or iOS.

SHAR is open source, so you can inspect the code. Release ZIPs and generated
binaries will expose integrity hashes so users can detect unexpected
modification and perform their own security checks. A hash is evidence only: it
is never reconstruction input and it does not itself prove software is safe.

1. Buy or use a lawful copy of *The Simpsons: Hit & Run* and install it normally.
   Leave the files where the installer put them. On Windows this is commonly a
   path similar to:

   ```text
   C:/Program Files (x86)/Vivendi Universal Games/The Simpsons Hit & Run/
   ```

1. Install CPython 3.12 or newer from the official
   [Python downloads page](https://www.python.org/downloads/).

1. Download the newest `shar-v<version>.zip` from Releases.

1. Extract the ZIP anywhere you want. The final release tree is intended to look
   like this:

   ```text
   CHANGELOG.md
   shar.py
   NOTICES.md
   game.jsonl
   dist.json
   user.json                  # created/updated locally
   algorithms/
     <arch>/<os>/build.txt
     <arch>/<os>/hash.txt
   code/
     *.py
   mods/
   scripts/
     windows/shar.ps1
     macos-linux/shar.sh
   ```

   The release is produced from `src/user/`. Helper Python lives under `code/`
   to keep `shar.py` small. The release convention keeps executable/script
   construction entry code at the end of its owning `.py` file.

1. Run `shar.py`. You can use the platform helper or a terminal:

   ```text
   python shar.py
   ```

1. A lightweight GUI will ask for the original game installation. You will be
   able to browse, paste/type the path, or drag `Simpsons.exe` onto the window
   where the platform GUI supports file dropping. On Windows, if you do not know
   the location, the original shortcut's **Open file location** action is usually
   the easiest way
   to find it.

1. SHAR performs a fast minimum-source gate and then the deeper deterministic
   validation/reconstruction checks. Editing `game.jsonl` is not a way to make
   an invalid installation valid.

   The exact root requirements are:

   - `Simpsons.exe`: mandatory, minimum 1.
   - `Simpsons.ico`: mandatory, minimum 1.
   - `uninst.ico`: optional; relevant only to some from-scratch workflows.
   - English game data: mandatory canonical base language.
   - Other official original languages: preserved/exported as SHAR language
     mods, not compiled into the canonical base package.

   **TODO: Make English the only canonical base language.**

1. Select any targets you want:

   - Android ARM64 — APK
   - iOS ARM64 — IPA
   - Linux ARM64
   - Linux AMD64 / x86-64
   - macOS ARM64
   - Windows ARM64
   - Windows AMD64 / x86-64

   The exact host match is labelled `← this is your system`, but is not silently
   selected and no other target is hidden. `user.json` remembers the choices;
   target toggles are on by default until you disable them.

1. SHAR runs the reconstruction/build procedure and shows progress plus an ETA.
   The algorithm still requires lawful local game input. It is not a bundled
   copy of the original game. The initial supported flow does not promise saved
   partial progress/resume state; interrupted work is restarted unless a later
   TODO explicitly adds resumability.

1. Results appear under `dist/`.

## Platform outputs

### Windows AMD64 / ARM64

```text
dist/<windows-target>/The Simpsons Hit & Run/
```

Copy or rename `The Simpsons Hit & Run/` wherever you want. The packaged
Shipping executable and its Unreal runtime stay inside that portable directory.
An optional `direct-access-desktop.ps1` helper can be double-clicked or run to
create a desktop shortcut named `The Simpsons Hit & Run™`; you can also create a
shortcut manually.

### Linux AMD64 / ARM64

```text
dist/<linux-target>/The Simpsons Hit & Run/
```

The directory is portable like the Windows package and uses the native Linux
entrypoint. Linux users probably do not need a tutorial explaining how to run an
executable from a directory, so this section intentionally stays short.

### macOS ARM64

```text
dist/macos-arm64/The Simpsons Hit & Run.app
```

The final package uses the normal macOS application layout and includes only the
runtime data needed by the port. Signing/notarization constraints are documented
when they are actually required by the chosen distribution path.

### Android ARM64

```text
dist/android-arm64/the-simpsons-hit-and-run.apk
```

Install the APK using Android's normal local APK flow. Platform security settings
may require explicitly allowing installation from the app/file manager you use.

### iOS ARM64

```text
dist/ios-arm64/the-simpsons-hit-and-run.ipa
```

SHAR produces a local/sideload-oriented IPA. It does not provide App Store
submission. Installing an unsigned or locally signed IPA depends on Apple's
current signing/device requirements; SHAR will document its package output, not
pretend those external requirements do not exist.

## Reconstruction algorithms and integrity files

Each target release uses separate files instead of one huge algorithm document:

```text
algorithms/<arch>/<os>/build.txt
algorithms/<arch>/<os>/hash.txt
```

`build.txt` describes the public-safe reconstruction/build procedure.
`hash.txt` is only for exact integrity/modification comparison of expected
distributable artifacts and can be combined with separate security checks.
**The hash is never used to create game files.**

For coverage analysis, the maintainer's complete lawful installation is treated
as the private internal **100% reference**. The current planning estimate treats
a traditional minimum installation as roughly **60%** of that complete reference,
but that estimate must be measured across lawful installations before it becomes
a product rule. The current design target is to investigate a **45–55% minimum
similarity window** for accepting candidate source layouts.

Those numbers are deliberately not production policy yet: “similarity” must
first be defined and tested so the gate cannot substitute for required original
bytes, exact identities, deeper structure, hashes, or provenance validation.
The algorithm generator must not serialize or full-tree-diff the private 100%
reference into a recipe. The published algorithm must not contain a reversible
encoding of the complete reference installation.

**TODO: Define a public-safe reconstruction algorithm gate with bounded
similarity.**

## How to install mods

> **The base SHAR pipeline does not consume `game/mods/` anymore.** It builds the
> pure faithful game. Mods are a later runtime/user-tool concern and must never
> be silently injected into the base build. Jebano and Muckluck are converter
> test cases only.

**TODO: Keep the base pipeline pristine and remove `game/mods` from supported
input.**

On desktop, go to the exported game's installation directory and place a SHAR
mod **folder or `.zip`** inside:

```text
mods/
```

Android and iOS require platform-specific writable locations. Those exact paths
will be documented only after the final application identities and packaged-device
tests prove them; the README will not invent mobile paths in advance.

**TODO: Use one normalized portable mod import contract.**

Only install mods from authors/distributors you trust. You are responsible for
what you install and for having the rights required to redistribute any content
you package. SHAR mods are intended to stay inspectable: assets and source are
not intentionally locked or obfuscated. “Inspectable/open” describes the package
shape; it does **not** force GPL or any other specific license on a mod author.

## How to convert LMLM mods to SHAR

> **Never start a new mod in LMLM.** Its legacy limitations will constrain your
> creativity and make complex work harder. Start new mods directly for SHAR so
> you can use the broader modern toolset instead.

Legacy conversion is intentionally separate from the faithful base release.
Anyone who wants it downloads the standalone Python directory:

```text
tools/lmlm/
```

Run `python main.py inspect <mod.lmlm>` for read-only validation or
`python main.py convert <mod.lmlm> <output>` to create an inspectable open
conversion workspace. The current converter extracts recoverable content and
records SHA-256 provenance; final SHAR-package adaptation remains gated on the
mod-package schema instead of pretending every legacy behavior translates.

The default converter policy is:

```text
decompilable-mods-only=true
```

This is a local converter policy. If a user changes that policy, the resulting
conversion choice and its legal, licensing, and redistribution consequences are
the user's responsibility. The converter will initially be tested with the
Jebano and Muckluck mods; that does **not** mean all LMLM mods are supported or
that SHAR will maintain custom conversion logic for every legacy package.
Community conversion tools are welcome.

**TODO: Convert decompilable LMLM mods into open SHAR mod packages.**

## How to make mods — coding or no coding, it does not matter

> This authoring workflow is still WIP.

Download the SHAR repository ZIP and open it with your preferred coding agent.
Popular examples include Cursor, Antigravity, Codex, and Claude. `AGENTS.md` is
intended to contain two clearly separated modes: technical repository/build
engineering and a **default SHAR mod-authoring personality**.

Example prompts:

```text
I am working on a mod. I converted an LMLM package, but some things are missing
and some missions are broken.
```

A useful agent should explain that unsupported legacy behavior may not translate,
ask for lawful missing/new assets when necessary, ask how you want the mod to
behave, and help rebuild the missing behavior for SHAR rather than pretending
that the old package is authoritative.

Other examples:

```text
Yes, I want to make a boss battle after the last mission of level 2.
```

```text
Yes, I want to unify the map and translate the coordinates of the original
missions.
```

You iterate by testing the mod and telling the agent what works, what looks
wrong, and what you want changed. Repository skills provide the technical game
and Unreal knowledge so the agent does not have to rediscover every convention.

Native mod code is C++. Published source should pass the same strict
Clang-family validation expected by SHAR. If you want to write a mod by hand
**for some reason**, you can; source is still source. A package may physically
exist without a clean lint result, but the tooling must report that clearly and
agents should produce clean code by default.

**TODO: Make `AGENTS.md` default to SHAR mod authoring and validate C++ with
Clang.**

## How to build SHAR from scratch

This is the **developer/expert workflow**, not the normal player installation
path. It intentionally assumes you already know how to use a terminal, install
native compiler/SDK prerequisites, diagnose platform toolchains, and read build
logs. If you only want to play, use the lightweight release flow above.

1. Place a lawful source installation directly under `game/` with no extra
   directory layer.
1. Use the repository-pinned Python/toolchain versions and supported Unreal
   Engine version documented by the build tooling.
1. Run `python tools/build/dependencies.py`.
1. Run `python tools/build/check.py`.
1. Run `python tools/build/arch.py`.
1. Run `python tools/build/run.py`.
1. Validate the repository with Jig.

`Simpsons.ico` is preserved as canonical source identity. SHAR does not need an
artistic replacement icon system. If a target needs a different icon container,
that should be a deterministic build-only representation of the original icon,
not a replacement artwork pipeline.

**TODO: Preserve the canonical source icon without replacing or regenerating
it.**

## Release and changelog policy

The public ZIP is built from `src/user/` only. It must not contain repository
caches, original game assets, proprietary engine source, or developer-only state.
Player/modder changelogs are handwritten and intentionally nontechnical; build
manifests and validation evidence remain separate technical artifacts.

**TODO: Publish `src/user` as the versioned SHAR release ZIP.**

## Project documents

- [`TODO.md`](TODO.md) is the canonical unfinished-work index.
- [`ROADMAP.md`](ROADMAP.md) contains project phases and progress.
- [`AGENTS.md`](AGENTS.md) contains AI-agent guidance.
- [`docs/adr/index.md`](docs/adr/index.md) contains architecture decisions.
- [`docs/technical/index.md`](docs/technical/index.md) contains technical specs.
- [`docs/legal/index.md`](docs/legal/index.md) contains legal research/scope notes.
- [`skills/`](skills/) contains task and Unreal guidance.

## Legal and provenance

SHAR is an independent interoperability and reimplementation project. It is not
affiliated with or endorsed by the original publishers, developers, licensors,
platform holders, or Epic Games. You are responsible for obtaining lawful game
input and for the licenses/permissions applicable to mods or assets you install,
convert, modify, or redistribute.

Repository-owned material is available under the MIT License in
[`LICENSE-MIT`](LICENSE-MIT).

## README images

Screenshots will be added after the final player GUI and output layout stop
moving.

**TODO: Add final player-facing README screenshots.**
