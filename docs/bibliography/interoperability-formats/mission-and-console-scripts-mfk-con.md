# MFK And CON Command Scripts

This non-governing record documents one interoperability subject without
granting rights in proprietary code, tools, documentation, game data, assets,
names, marks, or user-supplied content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Command, argument, comment, ordering,
  scope, and non-execution behavior for the reviewed repository corpus is
  independently verified and cross-checked against current community command
  documentation; the formal grammar, historical command ownership, version
  history, title extensions, and authoritative first-party documentation remain
  unresolved.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-08-08.
- Distribution posture: Local interoperability research only.
- Subject class: Proprietary textual command and configuration script family.

## Covered Material

MFK mission and gameplay scripts and CON vehicle or gameplay configuration
files, including commands, arguments, comments, ordering, and route-specific
semantic summaries.

## Repository Use And Scope

The pipeline reads valid UTF-8 user-supplied files and emits structured JSON
summaries. It does not execute the scripts, reproduce a historical parser, or
publish original script bodies as repository content.

## Provenance And Version History

Repository evidence supports a related command-language family. Donut Team's DT
Docs command reference was reviewed as a secondary interoperability cross-check
for current command scope, syntax, and parameter descriptions. The reviewed
cross-checks include `AddObjective`, `AddNPC`, `AddObjectiveNPCWaypoint`,
`AddCollectible`, ambient dialogue animation commands, conversation-camera
commands, `SetDurationTime`, `AddStageVehicle`, `ActivateVehicle`,
`AddStageWaypoint`, `SetStageTime`, `AddStageTime`, countdown and completion
commands, stage AI/catch-up commands, `SetHUDIcon`, `SetMaxTraffic`,
`SetMusicState`, `SetFadeOut`, `SetIrisWipe`, `RESET_TO_HERE`,
`SetStageMessageIndex`, swap locators, mission camera and multi-controller
commands, mission reset locators, `SetInitialWalk`, `SetDynaLoadData`,
`StreetRacePropsLoad`, `StreetRacePropsUnload`, `UsePedGroup`, and mission HUD
controls. The Dyna Load Data reference was also cross-checked for its documented
postfix operations: region load/unload, interior load/unload, and World Sphere
enable/disable. Mod Launcher `AddStageDynaLoadData`, `SetStageDynaLoadData`, and
checkpoint extensions are not treated as evidence that those commands exist in
the original game corpus. `AddNPC`, `AddObjectiveNPCWaypoint`,
`AddStageVehicle`, and `ActivateVehicle` were also used as secondary
cross-checks that character,
vehicle, and locator arguments are source names rather than generated package
identifiers; optional or sentinel driver/locator forms remain explicit instead of
becoming fabricated catalog objects. Community mission tutorials and public
compatibility implementations were additionally reviewed for source shapes such
as `SetDestination`,
`SetDialogueInfo`, `SetCamBestSide`, `SetConversationCam`, AI tuples, and mission
FMV usage. These secondary descriptions are used only where repository evidence
independently closes the same command form; they are not used to invent a
precedence rule when the same decoded locator name is present in multiple active
packages. Undocumented `AddStage` numeric values, condition parameters, AI
units, and compatibility arguments remain opaque. DT Docs is not treated as an
original developer specification, and the formal grammar, version history,
historical command ownership, title-specific extensions, and authoritative
first-party documentation remain unresolved.

## Authorship, Ownership, And Attribution

Historical developers, contributors, publishers, licensors, and any successors
retain applicable rights in upstream code, documentation, tools, marks, and
protected expression. SHAR claims rights only in independently authored
repository material to the extent supported by authorship evidence and law.

## License Or Terms Basis

No standalone public specification license or redistribution grant for this
proprietary subject has been verified. The SHAR MIT License applies only to
material the repository owner has authority to license and does not absorb
upstream expression, assets, marks, patents, trade secrets, or contracts.

## Distribution, Modification, And Compatibility

Independently observed functional facts may support compatibility work, but
successful parsing does not authorize distribution of the input, extracted
content, historical tools, or copied documentation. Copyright, contract, anti-
circumvention, trademark, patent, trade-secret, and jurisdiction questions
require separate fact-specific analysis in docs/legal.

## Compliance Posture

- Use only user-supplied local input obtained on a documented lawful basis.
- Keep original and extracted proprietary payloads outside Git and distributed
  artifacts.
- Use synthetic or independently authored fixtures for tracked regression tests.
- Preserve private hashes, acquisition dates, and version evidence without
  publishing local routes.
- Do not infer ownership, authorization, or redistribution rights from
  successful decoding.
- Separate factual command names and argument shapes from copied expressive
  script bodies.
- Do not execute untrusted commands during classification or documentation
  generation.

## Source References

- [Radical Entertainment historical toolchain provenance
  record](radical-entertainment-toolchain-and-formats.md).
- Historical Radical source notices reviewed locally; source material not
  distributed.
- Donut Team (n.d.) *The Simpsons: Hit & Run — All Console Script Commands* and
  linked command pages in DT Docs. Available at:
  <https://docs.donutteam.com/docs/TheSimpsonsHitAndRun/Scripting/ConsoleCommands/AllCommands>
  (Accessed: 8 August 2026). Secondary community interoperability reference.
- SHAR repository (2026) pipeline straggler command decoder and MFK/CON tests.
