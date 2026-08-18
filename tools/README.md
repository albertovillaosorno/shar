# Tools

## Purpose

Reserves the canonical boundary for independently shipped repository tools.

## Ownership

Owns tool functions that are not product source functions.

## Prohibitions

Does not own source crates, generated artifacts, or local dependencies.

## Navigation

- [`lmlm/`](lmlm/) is the Rust-only LMLM compatibility boundary (Rust 1.97.1)
  for users who explicitly choose to inspect or convert supported legacy LMLM
  mods.
- [`source-similarity/`](source-similarity/) measures content-free structural
  calibration evidence without choosing an admission threshold.
- [`validation/python_dependencies.py`](validation/python_dependencies.py)
  materializes the exact repository-local pytest/Ruff environment used by Jig.
