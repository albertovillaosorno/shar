# LMLM compatibility tool

This directory is the complete LMLM compatibility boundary. It is intentionally
separate from SHAR's faithful base extraction/build pipeline and does not install
or inject mods into the game.

Do **not** start a new mod in LMLM. This tool exists only to recover supported
legacy packages into inspectable material that can be adapted with modern SHAR
tools.

## Requirements

- Rust **1.97.1**
- A SHAR repository checkout

The converter does not install Rust, download a toolchain, or provide a
bootstrapper. It deliberately reuses the repository's Rust crates instead of
copying their implementations: Pure3D inspection comes from `src/formats/p3d`,
filesystem/path handling from `src/foundation/filesystem`, command-line behavior
from `src/foundation/command-line`, and SHA-256 from `src/foundation/sha256`.
LSPA/LMLM parsing remains local because that legacy container is specific to this
tool.

## Inspect a legacy package

```text
cargo run --manifest-path tools/lmlm/Cargo.toml -- inspect MyLegacyMod.lmlm
```

Inspection is read-only. It validates the LSPA v5 container, prints deterministic
JSON evidence, hashes the source and entries, and asks SHAR's existing Pure3D
parser to inspect `.p3d` payloads without executing archive content.

## Create a conversion workspace

```text
cargo run --manifest-path tools/lmlm/Cargo.toml -- convert MyLegacyMod.lmlm converted-mod
```

The converter publishes an atomic `converted-mod/` workspace containing the
validated original payloads under `content/` plus `conversion-report.json`. It
refuses an existing destination. The current status remains
`extracted-needs-shar-package-adaptation` until SHAR's final portable mod package
schema is authoritative.

There is intentionally no ZIP helper, installer, automatic game import, or
one-command toolchain setup here. Those are not part of the compatibility
problem.

## Scope

The converter is conservative and decompilable-only. Jebano and Muckluck are the
initial compatibility fixtures, not a promise that arbitrary historical LMLM
packages or behaviors will translate automatically. Lua and other legacy source
files are treated as data and are never executed by this tool. Unsupported
behavior should remain visible so an author can recreate it intentionally.

Users are responsible for having the rights required to inspect, convert, modify,
or redistribute the content they provide to the tool.
