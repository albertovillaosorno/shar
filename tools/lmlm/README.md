# LMLM compatibility tool

This is a small legacy-conversion base I made because I wanted to migrate the
Jebano and Muckluck mods. Those are the packages I test. I do **not** claim
broad
LMLM compatibility, and a complete converter for every historical mod is outside
my scope. If you need more, fork or extend this directory; you are welcome to
use
it as a starting point.

If you have never made an LMLM mod, start a new project directly for SHAR. That
avoids legacy format limitations and converter-specific design constraints.

This tool is intentionally separate from `src/` and from SHAR's reconstruction
pipeline. The product never invokes it. LMLM-specific parsing stays here, while
shared Rust behavior such as Pure3D decompilation, filesystem safety, CLI
handling, and SHA-256 is reused from the repository crates instead of copied.

## Requirements

Install Rust **1.97.1** yourself. There is no Rust installer, bootstrapper, or
automatic toolchain setup in this directory.

## Convert your imports

Put legacy `.lmlm` files directly in:

```text
tools/lmlm/import/
```

Then run from the repository root:

```text
cargo run -p shar_lmlm
```

With no arguments, `main` orchestrates the whole compatibility flow. Each import
is validated as an LSPA v5 archive, extracted into a persistent WIP workspace,
and valid `.p3d` payloads are decompiled through SHAR's existing Rust `p3d`
crate.

WIP state is deliberately retained at:

```text
.cache/lmlm/wip/
```

User-facing converted workspaces are published under:

```text
tools/lmlm/export/
```

Each converted export carries a `shar.mod-package.v1` `mod.json`. Its canonical
identity is derived from the exact source-package SHA-256, while its content
revision is derived from the current open workspace members. The package stays
content-only and requires `legacy.lmlm.review.v1`: conversion makes the legacy
payload inspectable, but does not claim unsupported legacy behavior is already
native SHAR runtime behavior.

Imports are read-only, and redirected or special import entries fail closed
instead of disappearing from the batch scan. Existing WIP state is verified
against the current source package but remains intentionally editable. When an
export is republished from
edited WIP, `mod.json` is regenerated from those current members instead of
copying stale package metadata. Existing exports are never silently
overwritten,
and a stale/tampered export package fails reuse. Lua and other legacy source
files are treated as data; archive content is never executed.

## Scope

The current converter is conservative and decompilable-only. A successful
extraction does not imply every legacy behavior already has a native SHAR
translation. Unsupported behavior remains visible so an author can rebuild it
intentionally.

Jebano and Muckluck are compatibility fixtures, not endorsements, dependencies,
or promises of future official SHAR versions. If either author later publishes a
native SHAR mod, prefer their version over a legacy conversion.

Manual commands remain available for debugging:

```text
cargo run -p shar_lmlm -- inspect MyLegacyMod.lmlm
cargo run -p shar_lmlm -- \
  convert MyLegacyMod.lmlm output
```

Users are responsible for having the rights required to inspect, convert,
modify, or redistribute content they provide to the tool.
