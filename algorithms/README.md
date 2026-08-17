# Algorithm workspace

This directory is the maintainer-facing workspace for authoring and replaying
source-bound reconstruction algorithms. Local source evidence, authored masters,
and generated output stay untracked. Only public README metadata and serialized
algorithm plans under `algorithm/*.txt` are publishable from this tree.

## Directory contract

Each algorithm family uses three semantic directories:

- `in/` contains the minimum lawful local evidence needed to author or replay
  that family. Pipeline code addresses these inputs by relative path and may
  search user-selected source locations for matching evidence.
- `master/` contains the local target tree used when authoring an algorithm. Its
  content is private and is needed only when creating or updating a plan.
- `algorithm/` contains the durable `.txt` plans produced by the generic
  algorithm engine.

Generated files do not live inside an algorithm family. There is one shared
`out/` tree:

- the base `game` export is generated directly below `out/`;
- language and Muckluck exports are mods and are generated below `out/mods/`.

`out/` is disposable generated state. It must never be used as reconstruction
input or as authority for `master/`.

## Families

`game/` owns the base game algorithm. Its local `in/` is deliberately small: it
contains only the evidence required to establish the minimum source set used by
this workspace. The current local reference includes executable variants, the
canonical icon, core RCF archives, and common cinematics. The final user-facing
minimum is governed by source-validation policy rather than by the mere presence
of a file in this maintainer workspace.

`lang/<locale>/` owns one language overlay per locale. Each locale README names
the exact local input currently expected by that authoring workspace. Language
families have no private `out/`; successful exports go to `out/mods/`.

`muckluck/` owns the SHAR Remastered compatibility mod workspace. It follows the
same mod layout as a language family and exports to `out/mods/`.

## Onboarding direction

The future lightweight Python onboarding flow consumes this contract rather
than copying this private workspace. It locates the user's lawful source by
relative path, determines which algorithm inputs and mod inputs are actually
available, offers only exports supported by that evidence, asks which available
languages to include, and optionally offers Muckluck when its local source is
present.

The existing target `.txt` files may be placeholders while an algorithm has not
yet been authored. A file's presence alone is not evidence that a target is
buildable; the serialized document must pass the generic algorithm validator.
