# Runtime parity test boundary

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Native gameplay and platform verification

## Context

Implementation similarity cannot prove that a reimplementation behaves
faithfully. Parity evidence must cover observable contracts at unit, native
automation, integration, packaged, and full-playthrough boundaries.

The runtime supports multiple platform families, processor architectures,
graphics presets, and input adapters. A package passing on one desktop target or
one quality preset cannot prove another target.

## Decision

Parity is proved through deterministic unit, native automation, integration,
platform-package, graphics-preset, input-adapter, and full-playthrough evidence
mapped to observable contracts rather than source implementation similarity.

Every claimed platform and architecture requires its own native packaged
acceptance evidence. Every desktop graphics preset requires resolved-setting,
visual, performance, and deterministic-replay evidence. Android requires
separate evidence for forced Low quality, native touch completion, safe areas,
display cutouts, storage, lifecycle transitions, and the 144-frames-per-second
ceiling.

## Consequences

- Unit, native automation, integration, packaged smoke, and full-playthrough
  evidence map to explicit observable parity contracts.
- Windows, Linux, macOS, Android, x64, and ARM64 support claims remain unproved
  until the corresponding native package passes its required evidence layers.
- Low, Medium, High, Epic, and Ultra share deterministic simulation evidence but
  each desktop preset has independent visual and resolved-configuration proof.
- Android Low has independent touch, presentation, lifecycle, thermal-state,
  frame-cap, and complete-playthrough evidence.
- Keyboard and mouse, gamepad, and touch adapters are tested against the same
  semantic action contract.
- Internal source similarity is irrelevant when the required behavior is proved.
- A failed evidence layer identifies an unproved contract instead of being
  masked by success at another layer.

## Rejected alternatives

- Treating unit tests alone as complete runtime parity evidence.
- Inferring one operating system, architecture, preset, or input adapter from a
  passing result on another.
- Claiming platform availability from compilation, emulation, editor play, or
  screenshots without a native packaged run.
- Relying only on manual play or subjective visual review.
- Comparing implementation structure instead of observable behavior.
