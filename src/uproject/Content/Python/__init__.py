# File:
#   - __init__.py
# Path:
#   - src/uproject/Content/Python/__init__.py
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
#   - The package marker for Unreal Editor Python discovery.
# - Must-Not:
#   - Run editor actions or mutate project state during import.
# - Allows:
#   - Unreal and validation tools to discover authored modules.
# - Split-When:
#   - An editor extension gains an independent package boundary.
# - Merge-When:
#   - Another marker owns this same discovery boundary.
# - Summary:
#   - Package marker for Unreal Editor Python tooling.
# - Description:
#   - Keeps the editor-facing Python directory importable.
# - Usage:
#   - Imported by Unreal and repository validation discovery.
# - Defaults:
#   - Package import performs no work.
#
# ADRs:
# - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
#
# Large file:
#   - false
#

# ruff: noqa: N999 -- path name is externally defined and cannot be renamed safely.

"""Package marker for validation discovery."""
