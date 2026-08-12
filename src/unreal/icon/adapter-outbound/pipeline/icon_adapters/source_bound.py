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
#   - Source-bound transform adapters.
# - Must-Not:
#   - Cross declared architecture boundaries or persist undeclared dependencies.
# - Allows:
#   - Inputs: values admitted by this module interface.
#   - Outputs: deterministic values or effects declared by that interface.
#   - Side effects: only those explicitly owned by the implementation.
# - Split-When:
#   - Split when another responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Source-bound transform adapters.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Source-bound transform adapters."""

from __future__ import annotations

from contextlib import contextmanager
from pathlib import Path
import shutil
import tempfile
from typing import Iterator

from .generated_plan import load_generated_plan
from .vendor.source_bound_diff.admission import identity_tree
from .vendor.source_bound_diff.emit_rust import write_rust_transform
from .vendor.source_bound_diff.exact import build_exact_plan, snapshot_tree
from .vendor.source_bound_diff.fingerprints import AnchorPolicy
from .vendor.source_bound_diff.protected import (
    materialize_protected_exact_plan,
    protect_exact_plan,
)
from .vendor.source_bound_diff.source_binding import SourceBindingPolicy


@contextmanager
def _staged_source(source_files: tuple[Path, ...]) -> Iterator[Path]:
    """Expose only selected ``game/*.ico`` files as one deterministic tree."""
    if not source_files:
        raise RuntimeError("source-bound icon evidence cannot be empty")
    names = tuple(path.name for path in source_files)
    if len(set(names)) != len(names):
        raise RuntimeError(
            "source-bound icon evidence contains duplicate names"
        )
    if any(
        path.suffix.lower() != ".ico" or not path.is_file()
        for path in source_files
    ):
        raise RuntimeError(
            "source-bound icon evidence must contain regular .ico files"
        )

    with tempfile.TemporaryDirectory(prefix="shar-icon-source-") as temporary:
        root = Path(temporary)
        for source in source_files:
            shutil.copyfile(source, root / source.name)
        yield root


def _identity(source_root: Path, relative_paths: tuple[str, ...]):
    return identity_tree(
        {
            relative: (source_root / relative).read_bytes()
            for relative in relative_paths
        }
    )


class SourceBoundAuthorer:
    """Emit a Rust transform whose target bytes require original game ICOs."""

    profile = "shar-icon-v1-exact-baseline-v1"

    def author(
        self,
        source_files: tuple[Path, ...],
        oracle_root: Path,
        output_algorithm: Path,
    ) -> None:
        with _staged_source(source_files) as source_root:
            exact = build_exact_plan(source_root, oracle_root)
            source_paths = tuple(record.path for record in exact.source.files)
            identity = _identity(source_root, source_paths)
            policy = SourceBindingPolicy(
                threshold_fraction=0.66,
                maximum_anchors=127,
                minimum_anchor_files=1,
                anchor_policy=AnchorPolicy(
                    window_bytes=32,
                    selection_modulus=64,
                ),
            )
            protected = protect_exact_plan(
                exact,
                identity,
                binding_policy=policy,
                context=b"shar-icon-v1:exact-baseline-v1",
            )

            # Prove the protected plan exactly reproduces the ignored authored
            # SVG tree before publishing a new algorithm/main.rs.
            with tempfile.TemporaryDirectory(
                prefix="shar-icon-author-"
            ) as temporary:
                recovered = Path(temporary) / "recovered"
                materialize_protected_exact_plan(
                    source_root,
                    identity,
                    plan=protected,
                    output_root=recovered,
                )
                if snapshot_tree(recovered) != snapshot_tree(oracle_root):
                    raise RuntimeError(
                        "source-bound verification did not reproduce "
                        "authored SVGs"
                    )

            write_rust_transform(protected, self.profile, output_algorithm)


class SourceBoundReconstructor:
    """Recover ignored SVGs from ``main.rs`` plus matching ``game/*.ico``."""

    def reconstruct(
        self,
        source_files: tuple[Path, ...],
        algorithm: Path,
        output_root: Path,
    ) -> None:
        plan = load_generated_plan(algorithm)
        by_name = {path.name: path for path in source_files}
        expected_names = tuple(record.path for record in plan.source.files)
        try:
            admitted = tuple(by_name[name] for name in expected_names)
        except KeyError as error:
            raise RuntimeError(
                f"required original game icon is missing: {error.args[0]}"
            ) from error

        with _staged_source(admitted) as source_root:
            identity = _identity(source_root, expected_names)
            output_root.parent.mkdir(parents=True, exist_ok=True)
            materialize_protected_exact_plan(
                source_root,
                identity,
                plan=plan,
                output_root=output_root,
            )
