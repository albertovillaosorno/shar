# File:
#   - __init__.py
# Path:
#   - src/uproject/tests/__init__.py
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
#   - The package marker for Unreal tooling regression tests.
# - Must-Not:
#   - Contain production behavior or shared mutable state.
# - Allows:
#   - Pytest and type checking to discover sliced tests.
# - Split-When:
#   - Another test package targets a separate product boundary.
# - Merge-When:
#   - Another marker owns the same test discovery surface.
# - Summary:
#   - Package marker for Unreal tooling regression tests.
# - Description:
#   - Keeps the feature-sliced tooling suite importable.
# - Usage:
#   - Discovered by the project-local pytest configuration.
# - Defaults:
#   - Test package import performs no setup.
#
# ADRs:
# - docs/adr/unreal/runtime/runtime-parity-test-boundary.md
#
# Large file:
#   - false
#


"""Regression tests for the SHAR Unreal project."""
