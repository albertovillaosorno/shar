# Icon Function

## Purpose

Reconstructs the local authored SVG icon masters from the canonical lawful icon
source and exports those SVGs into native Windows, macOS, Linux, Android, and
iOS icon formats.

## Ownership

`icon_algorithm.txt` is the tracked source-bound reconstruction record.
`assets/` is local recovered/authored evidence and `out/` is generated output;
both remain ignored by Git. Platform export code lives under `composition/`.

## Source identity

Icon reconstruction accepts `game/Simpsons.ico` only. `game/uninst.ico` is
installation material and must never become icon reconstruction evidence.
