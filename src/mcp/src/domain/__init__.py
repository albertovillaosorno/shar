# File:
#   - __init__.py
# Path:
#   - src/mcp/src/domain/__init__.py
#
# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE
# Path-Rule:
#   - All paths in this header are repository-root relative.
#
# Boundary-Contract:
# - Owns:
#   - The public facade for this hexagonal package boundary.
# - Must-Not:
#   - Introduce outward dependencies into inner package layers.
# - Allows:
#   - Explicit imports that preserve the package dependency direction.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Defines one package facade for the terminal translator.
# - Description:
#   - Keeps package discovery explicit and side-effect free.
# - Usage:
#   - Imported through normal Python package resolution.
# - Defaults:
#   - Importing the package performs no network or file IO.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Pure translator domain values and invariants."""
