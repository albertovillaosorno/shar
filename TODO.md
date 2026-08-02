# SHAR TODO

## Repository governance

### Adopt Jig as the canonical validator

- Status: Open
- Keep the local development installation source-linked from
  `.dependencies/jig/source` to the operator's Jig checkout.
- Do not vendor Jig, copy its source, or modify its repository from SHAR work.
- Do not add a GitHub Jig workflow until the repository-local policy is complete
  and the canonical source layout is stable.
- Author the tracked `.jig/` policy, taxonomy, language adapters, and validation
  projections only after the source-tree migration is complete.
- Replace direct diagnostic commands with one canonical Jig validation command
  only when that command can validate SHAR without weakening existing gates.

Acceptance requires a clean local exhaustive Jig validation, reproducible local
installation instructions, and an explicit later decision about hosted CI.
