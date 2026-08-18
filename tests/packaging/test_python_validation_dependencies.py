# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Repository validation regression tests.
# - Must-Not:
#   - Publish private game inputs or mutate external repositories.
# - Allows:
#   - Repository-local policy and bootstrap inspection.
# - Split-When:
#   - One validation policy gains an independent lifecycle.
# - Merge-When:
#   - The guarded policies become one inseparable repository contract.
# - Summary:
#   - Guards repository validation policy.
# - Description:
#   - Exercises tracked configuration and repository-local validation behavior.
# - Usage:
#   - Run through the canonical Jig pytest gate or repository-local pytest.
# - Defaults:
#   - Reads the current repository and writes only test-managed temporary state.
#

"""Host-portability tests for SHAR's Python validation bootstrap."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import sys
import unittest

_ROOT = Path(__file__).resolve().parents[2]
_PATH = _ROOT / "tools" / "validation" / "python_dependencies.py"
_SPEC = importlib.util.spec_from_file_location(
    "shar_python_validation_dependencies_test",
    _PATH,
)
if _SPEC is None or _SPEC.loader is None:
    raise RuntimeError("cannot load Python validation bootstrap")
_MODULE = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_MODULE)


class PythonValidationBootstrapTests(unittest.TestCase):
    """Keep default interpreter selection independent from host path layout."""

    def test_default_source_is_current_interpreter_on_every_host(self) -> None:
        self.assertEqual(
            _MODULE._default_source_python(_ROOT),
            Path(sys.executable),
        )

    def test_venv_layout_uses_current_host_convention(self) -> None:
        environment, python, pytest = _MODULE._venv_paths(_ROOT)
        self.assertEqual(
            environment,
            _ROOT / ".dependencies/python" / _MODULE._PYTHON_VERSION,
        )
        if sys.platform == "win32":
            self.assertEqual(python.name, "python.exe")
            self.assertEqual(pytest.name, "pytest.exe")
            self.assertEqual(python.parent.name, "Scripts")
        else:
            self.assertEqual(python.name, "python")
            self.assertEqual(pytest.name, "pytest")
            self.assertEqual(python.parent.name, "bin")


if __name__ == "__main__":
    unittest.main()
