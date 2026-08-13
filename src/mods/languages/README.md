# Canonical official-language mods

SHAR's canonical base language is English. This source-owned generator emits
original non-English localization evidence into deterministic, inspectable
source bundles as canonical language-mod content for the final result.

```text
python src/mods/languages/export.py french  <game-root> <output>
python src/mods/languages/export.py german  <game-root> <output>
python src/mods/languages/export.py italian <game-root> <output>
python src/mods/languages/export.py spanish <game-root> <output>
```

Each bundle preserves the original `srr2.txt` bytes, the selected character-set
sidecar when present, the exact official dialogue RCF when present, and the
localized readme when known and present. `text.jsonl` also exposes the selected
translation column as UTF-8 records while retaining the English comparison
value. `manifest.json` records SHA-256/size evidence and unavailable optional
sources.

These are **source bundles**, not final runtime SHAR mod packages yet. The
bundle status deliberately remains `source-bundle-needs-final-mod-package-adaptation`
until the shared mod-package schema is authoritative. The exporter never
changes the source game directory and refuses output below it.
