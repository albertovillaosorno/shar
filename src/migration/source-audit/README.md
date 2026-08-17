# Source Audit Function

## Purpose

Performs a read-only structural audit of supported containers in a lawful local
source tree before migration work consumes them.

## Ownership

Owns aggregate deep validation for Pure3D packages, RCF archives, RSD audio, and
RMV/Bink movie inputs. It reuses the repository's format parsers and reports
only format-class diagnostics and aggregate counts.

## Prohibitions

Never extracts source containers, invokes media transcoders, writes into source
inputs, or includes the selected private source path in public diagnostics.

## Build preflight integration

`tools/build/dependencies.py` publishes the `validate-source-deep` binary with
its binary SHA-256 and repository source-closure SHA-256.
`tools/build/check.py` first runs the exact minimum-manifest validator and only
then runs this deep audit. A deep-audit failure therefore cannot be bypassed by
editing the public minimum manifest.
