# Contextual interaction query and transaction boundary

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Unreal runtime interaction discovery, prompts, and execution

## Context

The game exposes many context-sensitive actions through one player-facing input:
mission conversations, interior entrances, vehicle entry, phone booths, animated
props, destructible props, vending machines, collectibles, repair pickups,
teleports, and purchases. These actions share discovery and prompt behavior, but
they do not share side effects, persistence, cancellation, or failure rules.

A class hierarchy that lets every world object poll input and mutate unrelated
systems would hide priority conflicts, duplicate eligibility logic, and make
partial execution difficult to recover. A single all-purpose interaction actor
would create the same coupling in a different form.

## Decision

SHAR uses a data-driven interaction pipeline with three separate authorities:

1. `USharInteractionDefinition` primary data assets define stable identity,
   gameplay tags, prompt policy, eligibility, execution kind, persistence, and
   verification requirements.
1. `USharInteractionSourceComponent` exposes world-local candidates. Actions
   that require a character position or exclusive use expose native Smart
   Object slots; automatic collision pickups do not allocate a Smart Object.
1. `USharInteractionSubsystem` owns deterministic candidate selection,
   reservation, execution, cancellation, and result publication.

The player's Enhanced Input `Interact` action is consumed only by the interactor
component. World actors never poll input. Interaction taxonomy and eligibility
use Gameplay Tags. Multi-step interactions execute through StateTree tasks;
atomic interactions use typed C++ executors behind the same transaction port.

Candidate ordering is fixed by descending authored priority, ascending squared
distance, and ascending canonical interaction identity. The selected candidate
is revalidated after reservation and immediately before the first side effect.

Every execution is a transaction with explicit phases: query, reserve,
revalidate, prepare presentation, commit domain effects, publish the result, and
release. A failed phase must either leave domain state unchanged or invoke the
interaction's typed compensation before releasing the reservation.

## Consequences

- Prompts and actions cannot disagree about eligibility because both consume the
  same query result.
- Mission, vehicle, progression, economy, and save systems remain behind ports;
  interaction code requests effects instead of modifying their storage.
- Smart Object reservations prevent two characters from occupying the same
  authored slot, while collision pickups remain lightweight.
- StateTree represents visible multi-step sequencing without becoming the owner
  of rewards, inventory, progression, or save data.
- Deterministic ordering makes overlapping interactables testable and prevents
  frame-order selection drift.
- New interaction kinds require a definition schema extension, a typed executor,
  and parity tests; adding another inheritance branch is not sufficient.

## Rejected alternatives

- Letting each interactable actor poll the input subsystem.
- Selecting the first overlap returned by physics or actor iteration order.
- Encoding prompts, eligibility, and effects only in Blueprint graphs.
- Using one monolithic handler hierarchy for every interaction family.
- Granting rewards or changing progression before reservation and final
  eligibility checks complete.
- Requiring Smart Objects for passive pickups that need no authored use slot.
