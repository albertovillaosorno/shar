# Faithful base and optional mod boundary

- Status: Accepted
- Decision date: 2026-08-12
- Scope: Canonical base reconstruction and optional creative changes

## Context

SHAR reconstructs a lawful local source into a faithful base game and also
supports user-authored mods. Those two goals need separate authorities. If an
optional redesign, replacement, or enhancement can enter the base merely because
it is technically possible, the base stops being reproducible from source
evidence and parity defects become difficult to distinguish from intentional
creative changes.

Unreal compatibility can require a different container, coordinate basis,
compression, package structure, runtime representation, or deterministic repair.
Those technical translations are not permission to redesign source content.

## Decision

The canonical base is source-backed. A base asset or gameplay change is admitted
only when it is one of the following:

1. deterministic conversion of validated lawful source evidence;
1. a representation change required for Unreal or a supported target platform;
1. an import/runtime correctness change required to reproduce source behavior;
1. a correction for a defect introduced by SHAR's own conversion or port.

Every admitted base change must retain enough evidence to distinguish the source
fact from the conversion rule and resulting artifact. When evidence is missing,
ambiguous, or contradictory, the base fails closed or leaves the work explicitly
pending. An operator, editor session, AI agent, screenshot, aesthetic
preference,
or convenience heuristic is not source authority.

Terrain, world layout, missions, mission ordering, models, textures, materials,
animations, audio, cinematics, UI, progression, tuning, and localization are not
manually redesigned for the canonical base. Equivalent technical
representations may differ internally, but they do not intentionally change the
source-authored presentation, topology, timing, ordering, placement, or gameplay
meaning except where a documented target constraint makes an exact
representation impossible.

Optional creative work belongs to a SHAR mod. This includes new or redesigned
missions, worlds, geometry, textures, audio, UI, balance, abilities,
presentation,
quality-of-life behavior, replacement media, and other enhancements that are not
required for faithful conversion. A mod may use registered extension and
replacement points, but it cannot become evidence for the canonical base.

Interior relocation is specifically outside the canonical base. Interior
packages retain their own source-space coordinates. Base conversion does not
apply precomputed placement matrices, connected-map coordinate transplants,
center shifts, or relocation offsets to them. Moving an interior to another
location or
arranging interiors into a redesigned world is optional mod behavior.

A bug fix discovered while authoring a mod may move into the base only when the
repository can independently prove that the defect was introduced by SHAR or
that the fix is required to reproduce lawful source behavior. The mod itself is
never that proof.

## Consequences

- Base outputs remain reviewable against source evidence and deterministic
  conversion rules.
- Missing parity cannot be hidden by a nicer replacement or manual editor fix.
- Target/platform compatibility work stays technical rather than artistic.
- Optional improvements can evolve rapidly without changing canonical base
  identity.
- Agents and operators can answer "base or mod?" from one durable policy instead
  of making per-feature aesthetic judgments.

## Rejected alternatives

- Improving or modernizing base assets because the new engine can do so.
- Accepting hand-edited terrain, mission, material, texture, or placement fixes
  without source-backed conversion evidence.
- Treating popular mods, screenshots, videos, or third-party replacements as
  canonical source authority.
- Hiding conversion defects behind replacement art or adjusted gameplay.
- Requiring optional creative work to preserve base parity instead of packaging
  it as a mod.
