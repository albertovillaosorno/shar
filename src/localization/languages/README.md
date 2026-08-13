# Canonical official-language mods

English is SHAR's canonical base language. `shar_languages` is internal Rust
product code that composes original non-English localization into canonical
language-mod source bundles for the final result. It is not a user-facing tool,
and there is no repository mod drop directory here.

The language surface is broader than text. French, German, and Spanish bundles
require the matching TextBible data, dialogue RCF, localized loading/license UI
assets, and localized cinematic audio. The original PC/PAL runtime selects FMV
audio indexes 1/2/3 for French/German/Spanish; faithful movie extraction maps
those streams to `audio_track_02.wav`, `_03.wav`, and `_04.wav` because English
stream 0 becomes `_01.wav`.

Italian remains represented because the TextBible declares it, but the current
lawful source contains only `???` Italian text and no `dialogi.rcf`. Generation
therefore fails closed instead of inventing an Italian localization.

`export_language` receives the lawful game root plus the already-normalized
movie package root, writes atomically outside both inputs, preserves exact source
bytes with SHA-256 evidence, emits UTF-8 `text.jsonl`, and copies the selected
movie WAV tracks into the language bundle. Final package adaptation remains open
until the shared SHAR mod-package contract is authoritative.
