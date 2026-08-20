# Icon Function

## Purpose

Reconstructs the local authored SVG icon masters from the canonical lawful icon
source and exports those SVGs into native Windows, macOS, Linux, Android, and
iOS icon formats.

## Ownership

`contract/icon_algorithm.txt` is the tracked source-bound reconstruction record.
`assets/` is local recovered/authored evidence and `out/` is generated output;
both remain ignored by Git. Platform export code lives under `composition/`.

## Source identity

Icon reconstruction accepts `game/Simpsons.ico` only. `game/uninst.ico` is
installation material and must never become icon reconstruction evidence.

## Local reconstruction and export

From the repository root, reconstruct only from the canonical game icon into the
ignored output tree:

```sh
cargo run -p shar_algorithm --bin algorithm -- replay \
  --source game/Simpsons.ico \
  --algorithm src/migration/icon/contract/icon_algorithm.txt \
  --output src/migration/icon/out/recovered
```

Then export the recovered SVGs to every supported platform icon layout:

```sh
python src/migration/icon/composition/export_cli.py \
  --assets src/migration/icon/out/recovered \
  --out src/migration/icon/out/platform
```

Do not substitute `game/uninst.ico`, wildcard `game/*.ico`, or the ignored local
`assets/` oracle as reconstruction evidence.
