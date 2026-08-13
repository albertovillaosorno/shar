# Source Audit Function

## Purpose

Performs a read-only structural audit of supported containers in a lawful local
source tree before migration work consumes them.

## Ownership

Owns aggregate deep validation for Pure3D packages, RCF archives, RSD audio, and
RMV/Bink movie inputs. It reuses the repository's format parsers and reports only
format-class diagnostics and aggregate counts.

## Prohibitions

Never extracts source containers, invokes media transcoders, writes into source
inputs, or includes the selected private source path in public diagnostics.
