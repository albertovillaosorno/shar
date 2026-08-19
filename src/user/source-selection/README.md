# Source Selection Function

## Purpose

Resolves one ordinary-player source selection to a lawful local game root before
validation or reconstruction begins.

## Ownership

Owns normalization of a selected installation folder, pasted or typed path, and
a selected or dropped `Simpsons.exe` path. It requires the canonical executable
directly beneath one resolved source root. The selected source path must not pass
through symbolic directory links or Windows junctions.

## Prohibitions

Never writes to the selected source, validates proprietary payload contents,
chooses reconstruction targets, or includes the selected private path in its
error text.

## Integration

The future lightweight user GUI can bind browse, text-entry, and drop events to
this adapter without changing source-root semantics. Fast manifest validation,
deep structural validation, similarity admission, and export remain separate
responsibilities.
