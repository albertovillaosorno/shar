# Transactional phone-booth vehicle retrieval

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Owned-vehicle selection, repair, and world delivery

## Context

Players need a level-independent way to inspect and retrieve vehicles they own.
The service must preserve damage, respect mission restrictions, display locked
content without granting it, repair destroyed vehicles through the economy, and
spawn the selected vehicle without duplicating ownership or leaving a partial
world mutation.

Vehicle retrieval is not vehicle purchase, costume purchase, traffic hijacking,
or mission-forced vehicle ownership. Combining those concerns would make save,
economy, mission, and world state disagree.

## Decision

Phone booths are Smart Object interactions backed by one
`USharVehicleRetrievalSubsystem` game-instance service and one Common UI vehicle
browser.

The subsystem builds an immutable projection from canonical catalog definitions,
accepted ownership, campaign reach, completion overrides, vehicle health, active
mission policy, currency, and the selected booth placement.

The Common UI browser consumes that projection through C++ UMG viewmodels. It
loads thumbnail and three-dimensional preview bundles through the Asset Manager
and renders them in an isolated presentation scene with no gameplay, ownership,
repair, or world authority. Rapid selection changes cancel the prior preview
lease, and stale asset callbacks cannot replace the accepted selection.

Browser selection hands the exact vehicle, booth, projection, mission, currency,
and world revisions to the retrieval service. A visible or loaded preview never
grants ownership, repairs damage, debits currency, spawns a gameplay vehicle, or
claims that persistence completed.

Selecting an eligible vehicle runs one transaction: validate eligibility,
persist current owned-vehicle health, stage any required repair debit, load the
selected primary-asset bundles, reserve a safe delivery transform, spawn or
reuse
one canonical vehicle instance, apply the declared driver presentation, verify
world registration, and commit the active retrieval slot and economy revision.

A destroyed owned vehicle requires the declared repair charge. The base policy
is
10 coins. Insufficient currency rejects the repair and preserves vehicle,
currency, and world state.

The retrieval service never changes clothing. Costume ownership and selection
remain separate purchase and character-presentation contracts.

## Consequences

- Locked vehicles may appear in the browser but cannot be selected.
- Only accepted ownership or an explicit completion override grants retrieval.
- Traffic access, secret-world access, mission use, and forced use do not imply
  persistent ownership.
- Damage survives vehicle replacement, level travel, and save reload.
- Forced-vehicle missions may disable retrieval or reject replacement.
- Re-selecting the already active canonical vehicle reuses it when valid rather
  than creating a duplicate.
- Repair, spawn, active-slot replacement, and read-back commit atomically.
- Driver characters are presentation bindings and do not create vehicle
  identity.
- Booth placement controls delivery location but never vehicle ownership.
- Failed loading, blocked transforms, stale projections, or save failure restore
  the prior accepted state.

## Rejected alternatives

- Spawning an arbitrary vehicle from a filename or editor path.
- Treating every traffic or secret vehicle encountered as owned.
- Repairing a destroyed vehicle by mutating health without an economy
  transaction.
- Debiting currency below zero to preserve historical behavior.
- Despawning the current vehicle before the replacement is verified.
- Combining clothing selection with the vehicle retrieval transaction.
- Allowing a mission-forced vehicle to become permanent ownership.
