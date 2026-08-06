# Recoverable extraction publication

- Status: Active
- Last reviewed: 2026-08-05

## Purpose

This specification defines how complete extraction runs preserve the last
accepted generated tree while rebuilding and how the next run recovers after a
process interruption.

It applies to complete extraction commands:

```text
pipeline extract-game game extracted
pipeline extract-game-resume game extracted
```

The partial `export-movies` and `export-lmlm` commands share the same output
lease and startup recovery. They update their admitted subtrees in place rather
than publishing a complete candidate root.

Both complete commands execute the same ten ordered extraction stages from the
beginning.
`extract-game-resume` does not skip stages or trust partially generated stage
output. Its additional meaning is that package continuity from the accepted
output must remain valid before the rebuild proceeds.

## Publication boundary

Every complete extraction writes below an empty sibling candidate directory.
The accepted output root remains unchanged while source validation, decoding,
optional-package application, normalization, output verification, minor-unit
manifest generation, and the run report execute.

After every stage succeeds, publication uses sibling directory renames:

1. move the accepted root to a backup when one exists;
1. move the complete candidate to the accepted name;
1. remove the backup and transaction state.

Candidate, backup, and state artifacts share the accepted root's parent so the
rename boundary does not cross a filesystem or volume by construction.

## Transaction artifacts

For an accepted root named `<output>`, the reserved sibling identities are:

```text
.<output>.pipeline-staging
.<output>.pipeline-backup
.<output>.pipeline-transaction.json
.<output>.pipeline-lock
```

They are generated, ignored by Git, and never part of the accepted extraction
manifest or report. Candidate, backup, and state are transient. The empty lock
file is persistent and reusable so every process competes for the same file
identity without a creation race. The state document contains only the fixed
schema identity
`shar-schoenwald.extraction-transaction.v1`; it contains no local paths, package
tokens, or source metadata.

The persistent lock file carries an exclusive standard-library file lease. A
live process acquires it before inspecting recovery state and keeps it for the
entire command. Another process targeting the same accepted root is rejected
before it can inspect, remove, publish, or partially update the active output.
The operating system releases the lease when the owner exits, including
abnormal
process termination.

## Startup recovery

Recovery is the first operation after validating the source and output root
relationship. It runs before optional-package approval and before package-set
continuity checks.

The following states are accepted:

- No state, candidate, or backup: begin the requested command.
- State and candidate with the accepted root unchanged: remove the candidate.
- State, candidate, and backup with no accepted root: restore the backup, then
  remove the candidate.
- State, accepted root, and backup: keep the published accepted root, then
  remove the backup.

After recovery, the old state is removed. A complete extraction creates a new
empty candidate. A partial export retains the same exclusive lease and updates
only its admitted subtree.

## Fail-closed states

The command stops without choosing an output when:

- candidate or backup artifacts exist without the exact state document;
- the state document is malformed, has a different schema, or is a symlink;
- the persistent lock identity is symlinked, not a regular file, or actively
  leased by another process;
- candidate, backup, accepted output, or parent identities have an unexpected
  file type or are symlinked;
- another process still holds the output lease;
- backup restoration, candidate cleanup, rename, or state cleanup fails.

Unknown artifacts are never inferred to be safe merely from their names or
ages. No stale timeout can override a live lease.

## Stage and package ordering

A recovered accepted root is available before optional-package approval is
evaluated. A missing or stale approval token therefore cannot strand the prior
accepted tree under the backup identity.

For `extract-game-resume`, optional-package manifest continuity is checked
against the recovered accepted tree. A changed, removed, or legacy package set
still requires a clean `extract-game` invocation. The new candidate receives the
same approved package token, but no stage reads generated files from the old
accepted tree.

## Failure behavior

Failure before publication removes the candidate and state while leaving the
accepted root byte-for-byte unchanged. Failure after the accepted root was
moved but before candidate publication restores the backup before returning.

Once the complete candidate has received the accepted name, it is a complete
published tree rather than partial stage output. If later backup or state
cleanup fails, the exact state document remains recovery evidence for the next
run.

This contract protects against process interruption and ordinary filesystem
operation failures covered by the adapter. It does not claim protection from
physical storage loss, filesystem corruption, or a platform violating its
ordinary same-parent rename and file-lock semantics.

## Verification

Unit and integration tests prove:

- failed clean and resume runs preserve an accepted sentinel;
- abort removes a candidate without changing the accepted root;
- successful publication replaces the complete root;
- interruption before candidate publication restores the backup;
- interruption after candidate publication keeps the complete candidate;
- active lease contention preserves both accepted and candidate data;
- malformed state and unowned artifacts fail without mutation;
- recovery precedes both optional-package approval and continuity checks;
- partial exports respect an active full-extraction lease and recover an
  abandoned full publication before their own validation;
- canonical output identity cannot alias a location inside the source tree;
- missing path components followed by `..` normalize without creating the
  discarded directory;
- linked parent prefixes cannot redirect full or partial output creation; and
- relative output names derive portable sibling transaction identities.

A maximum local extraction was externally terminated on 2026-08-05 during the
normalized-output audit, before the candidate received the accepted name. The
next invocation used the current transaction implementation to remove the
abandoned 136,411-file candidate in 85 seconds before evaluating
optional-package approval. It then rejected the missing approval token with no
accepted output, no transient transaction artifacts, and one empty reusable
lock file.
