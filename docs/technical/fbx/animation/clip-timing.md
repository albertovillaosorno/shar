# Animation clip timing

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

- [Hexagonal scene export](../../../adr/pipeline/fbx/hexagonal-scene-export.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical source-document evidence classification and publication boundary](../../unreal/historical-source-document-evidence-classification-and-publication-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](../../unreal/native-import-material-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](../../unreal/playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Character animation clip catalog and vehicle-handoff choreography runtime](../../unreal/character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md)

## Purpose

This specification defines how repository-owned animation conversion preserves
authored frame rate, range, duration, key ordering, transition timing, event
timing, and root displacement from private digital-content-creation evidence to
normalized interchange and native Unreal Animation Sequence assets.

A source scene, editor playback preference, backup-directory label, or filename
is evidence only. The canonical clip definition and deterministic conversion
recipe own target timing.

## Timing identity

Every clip records:

- canonical character identity;
- canonical clip identity and revision;
- source evidence class;
- exact source time-rate numerator and denominator;
- authored start and end frame;
- endpoint inclusion policy;
- expected sample count;
- expected duration as an exact rational value;
- animated track set;
- root-motion policy;
- looping or finite policy;
- transition, contact, impact, hold, and event markers;
- conversion-recipe revision; and
- target Skeleton and native asset revisions.

A nominal label such as film, PAL, NTSC, game, or custom is converted to the
exact configured rational rate before duration is calculated.

## Repository model

Input adapters provide the declared source rate, logical frame identities, clip
bounds, key values, tangent data, curve types, and marker positions. The
canonical clip model keeps those values distinct.

The normalized writer converts source values into its serialization time unit
without changing logical playback speed. The native import adapter converts the
normalized payload into one Animation Sequence against the exact declared
Skeleton.

Neither adapter may infer a frame rate from the current workstation, Maya
playback preference, Unreal editor preference, render frame rate, or the first
previously imported asset.

## Source animation evidence

Historical Maya ASCII clips may contain animation curves and one clip node with
no embedded Skeleton hierarchy. Such evidence is animation-only input and must
bind to a separately verified canonical rig.

Review extracts only:

- source time unit;
- animated track and attribute names;
- key times and values;
- tangent and infinity policy when material;
- clip bounds;
- curve and clip metadata;
- root or motion-track evidence; and
- conversion warnings.

Source scene text, private paths, editor state, backup labels, unused nodes,
comments, and raw curve dumps do not enter public documentation or packaged
runtime.

## Clip range

The clip range is explicit. Import policy chooses one of:

- exact exported interval;
- exact animated interval after validated trimming; or
- an explicit set range.

The choice is stored in the conversion recipe. It cannot vary by operator,
platform, editor session, or importer default.

Keys outside the accepted range are rejected unless a declared trim operation
proves that they are irrelevant source residue. A trim operation records the
original range, resulting range, and reason.

## Sample and key preservation

Key ordering is monotonic within each channel. Conversion preserves:

- authored key times;
- authored value ordering;
- intended holds;
- contact and impact timing;
- fall, landing, flail, recovery, idle, locomotion, and dialogue boundaries;
- root displacement;
- loop closure when required;
- event and notify positions; and
- curve extrema within declared tolerances.

Resampling is allowed only through a versioned recipe with a target rate,
interpolation policy, error tolerance, and deterministic implementation.

## Animation Sequence import

The native importer creates or updates one canonical Animation Sequence and
validates:

- exact Skeleton binding;
- imported duration;
- sampled frame count;
- track count and normalized names;
- root-motion settings;
- curves and attributes;
- compression policy;
- notifies or event metadata;
- additive settings when applicable;
- looping policy; and
- retained dependencies.

Montages, sections, slots, transitions, and notifies are derived target assets.
They are created only when the public character or gameplay contract requires
them; their timing cannot be inferred from source filenames alone.

## Root motion

Root motion is a closed clip policy:

- none;
- presentation-only root movement;
- extracted movement owned by the movement component;
- montage-owned root motion; or
- explicitly rejected source displacement.

The recipe identifies the authoritative root or motion track and expected total
translation and rotation. Import cannot select a root by array position or first
track order.

## Transition clips

Fall, landing, impact, recovery, get-up, turn, and other transition clips
declare
entry and exit pose expectations. Validation may compare:

- root transform;
- major joint orientation;
- contact state;
- velocity intent;
- pose distance; and
- marker alignment.

A transition clip that cannot connect to its declared states fails validation or
requires a separately authored blend policy.

## Phased choreography timing

Vehicle entry, vehicle exit, and other phased interactions may be represented by
one composed clip or multiple phase clips. The canonical choreography records:

- ordered semantic phases;
- exact clip interval for every phase;
- section and marker positions;
- blend and overlap policy;
- root-motion continuity;
- door, hardpoint, threshold, seat, attachment, detachment, and control-transfer
  marker timing;
- interruption windows; and
- composed-versus-phased equivalence tolerances.

A source filename token such as all, driver, high, open, or close cannot define
phase timing. The normalized catalog and native read-back own those semantics.
Composed and phased variants must reach equivalent accepted gameplay state under
<!-- markdownlint-disable-next-line MD013 -->
[Character animation clip catalog and vehicle-handoff choreography runtime](../../unreal/character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md).

## Looping clips

A looping clip declares loop eligibility and closure tolerances. Validation
compares first and last accepted poses, root displacement, velocity, cyclic
curves, and event duplication.

A source clip is not looped merely because playback was repeated in the source
editor.

## Determinism

The same source evidence, canonical rig, recipe, toolchain, and target profile
must produce identical normalized timing metadata and equivalent native read-
back within declared compression tolerances.

Wall-clock time, thread scheduling, filesystem order, locale, editor selection,
and current playback range cannot affect output.

## Invariants

- Clip bounds are finite and ordered.
- The exact rational frame rate is known.
- Key times are monotonic within each channel.
- Representation-unit conversion preserves authored duration.
- No importer or review application supplies an implicit frame-rate default.
- Source editor playback settings are not clip authority.
- Animation-only evidence targets one exact Skeleton revision.
- Root-motion policy is explicit.
- Looping and finite clips are distinguished.
- Native duration, sample count, tracks, and curves are read back.
- Source scene text and private metadata never ship.

## Failure behavior

The clip fails closed when:

- rate is missing, zero, negative, ambiguous, or contradictory;
- bounds are non-finite, reversed, or inconsistent with keys;
- keys are outside the accepted range without a validated trim;
- timing is non-monotonic;
- a required track is missing;
- Skeleton binding is incompatible;
- root ownership is ambiguous;
- resampling exceeds tolerance;
- native duration or sample count differs from expectation;
- a required marker, curve, or transition boundary is lost;
- deterministic reimport differs unexpectedly; or
- a generated artifact contains private evidence or source text.

Failure publishes no native clip and leaves the previous accepted asset
unchanged.

## Verification

- Domain tests compare exact source and canonical duration.
- Writer tests read back serialized key times and clip bounds.
- Import tests verify native duration, sampled frame count, tracks, curves,
  Skeleton binding, and root policy.
- Regression fixtures cover fractional rates, multiple channels, holds, trims,
  transition clips, loops, and root motion.
- Determinism tests repeat conversion and compare normalized and native
  read-back.
- Privacy tests reject source paths, source scene text, and private metadata.
