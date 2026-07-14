# File:
#   - __init__.py
# Path:
#   - src/mcp/tests/__init__.py
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
#   - The Python package marker for MCP translator tests.
# - Must-Not:
#   - Execute tests or mutate runtime state during import.
# - Allows:
#   - Stable absolute imports between test support modules.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Defines the MCP translator test package.
# - Description:
#   - Keeps black-box support imports explicit.
# - Usage:
#   - Imported only by the Python test runner.
# - Defaults:
#   - Importing this package has no side effects.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Regression tests for the Unreal MCP terminal translator."""
