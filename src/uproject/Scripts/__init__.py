# File:
#   - __init__.py
# Path:
#   - src/uproject/Scripts/__init__.py
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
#   - The import boundary for Unreal maintenance scripts.
# - Must-Not:
#   - Run project generation or alter files during import.
# - Allows:
#   - Tests and launchers to import deterministic helpers.
# - Split-When:
#   - A script family gains an independent lifecycle.
# - Merge-When:
#   - Another package owns this maintenance boundary.
# - Summary:
#   - Package marker for Unreal maintenance tooling.
# - Description:
#   - Exposes project-generation and validation helpers.
# - Usage:
#   - Imported by tests and project-generation entry points.
# - Defaults:
#   - Package import invokes no external process.
#
# ADRs:
# - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
#
# Large file:
#   - false
#

# ruff: noqa: N999 -- Unreal project paths are externally defined.

"""Maintenance scripts owned by the SHAR Unreal project."""
