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
#   - Concrete platform packaging from reconstructed SVG masters.
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
#   - Concrete platform packaging from reconstructed SVG masters.
# - Description:
#   - Implements the declared responsibility for the migration icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Concrete platform packaging from reconstructed SVG masters."""

from __future__ import annotations

from collections.abc import Iterator
from contextlib import contextmanager
from contextlib import suppress
import json
from pathlib import Path
import re
import shutil
import struct
import tempfile

from profiles import ANDROID_DENSITY_SCALE
from profiles import ANDROID_LEGACY
from profiles import IOS_APPICON_SIZE
from profiles import LINUX_SIZES
from profiles import MAC_ICONSET
from profiles import WINDOWS_SIZES
from rendering import SvgRenderer

_ICO_DIRECTORY_ENTRY_FORMAT = "<" + ("B" * 4) + ("H" * 2) + ("I" * 2)
_ICO_HEADER_FORMAT = "<" + ("H" * 3)


class PlatformExporter:
    """Package the authored platform SVGs into target-system icon assets."""

    def __init__(
        self,
        renderer: SvgRenderer,
        icon_name: str = "simpsons-hit-run",
    ) -> None:
        """Initialize an exporter with its explicit rasterization dependency."""
        self.renderer = renderer
        self.icon_name = icon_name

    def export_all(
        self,
        source_root: Path,
        output_root: Path,
    ) -> tuple[str, ...]:
        """Export every supported platform layout from authored SVG masters.

        Returns:
            Stable source notes written beside the exported layouts.

        """
        if output_root.exists():
            shutil.rmtree(output_root)
        output_root.mkdir(parents=True, exist_ok=True)

        windows_linux = self._required(
            source_root / "windows-linux.svg", "windows-linux.svg"
        )
        macos_linux = self._required(
            source_root / "macos-linux.svg", "macos-linux.svg"
        )
        android = self._required(source_root / "android.svg", "android.svg")
        ios = self._required(source_root / "ios.svg", "ios.svg")

        self._windows(windows_linux, output_root / "windows")
        self._macos(macos_linux, output_root / "macos")
        self._linux(
            windows_linux,
            macos_linux,
            output_root / "linux",
        )
        self._android(android, output_root / "android")
        self._ios(ios, output_root / "ios")

        notes = (
            "Windows: windows-linux.svg",
            "macOS: macos-linux.svg",
            "Linux primary: windows-linux.svg",
            "Linux alternate: macos-linux.svg",
            "Android: android.svg (transparent foreground over black)",
            "iOS: ios.svg (single-size 1024 px AppIcon asset catalog)",
        )
        (output_root / "SOURCES.txt").write_text(
            "\n".join(notes) + "\n", encoding="utf-8"
        )
        return notes

    @staticmethod
    def _required(path: Path, label: str) -> Path:
        if not path.is_file():
            message = f"required icon asset is missing: {label}"
            raise RuntimeError(message)
        return path

    def _windows(self, svg: Path, folder: Path) -> None:
        folder.mkdir(parents=True, exist_ok=True)
        frames = [
            (size, self.renderer.render_png(svg, size))
            for size in WINDOWS_SIZES
        ]
        self._write_ico(frames, folder / "icon.ico")
        (folder / "icon_256.png").write_bytes(dict(frames)[256])

    @staticmethod
    def _write_ico(frames: list[tuple[int, bytes]], output: Path) -> None:
        """Write a PNG-backed multi-resolution Windows ICO without Pillow."""
        count = len(frames)
        directory = bytearray()
        payload = bytearray()
        offset = 6 + 16 * count

        for size, data in frames:
            width = 0 if size == 256 else size
            height = 0 if size == 256 else size
            directory += struct.pack(
                _ICO_DIRECTORY_ENTRY_FORMAT,
                width,
                height,
                0,
                0,
                1,
                32,
                len(data),
                offset,
            )
            payload += data
            offset += len(data)

        with output.open("wb") as stream:
            stream.write(struct.pack(_ICO_HEADER_FORMAT, 0, 1, count))
            stream.write(directory)
            stream.write(payload)

    def _macos(self, svg: Path, folder: Path) -> None:
        iconset = folder / "icon.iconset"
        iconset.mkdir(parents=True, exist_ok=True)
        cache = {
            size: self.renderer.render_png(svg, size)
            for size in sorted(set(MAC_ICONSET.values()))
        }
        for filename, size in MAC_ICONSET.items():
            (iconset / filename).write_bytes(cache[size])
        self._write_icns(cache, folder / "icon.icns")
        (folder / "icon_1024.png").write_bytes(cache[1024])

    @staticmethod
    def _write_icns(cache: dict[int, bytes], output: Path) -> None:
        """Write modern PNG-backed ICNS chunks without Xcode or Pillow."""
        chunks = (
            (b"icp4", 16),
            (b"ic11", 32),
            (b"icp5", 32),
            (b"ic12", 64),
            (b"icp6", 64),
            (b"ic07", 128),
            (b"ic13", 256),
            (b"ic08", 256),
            (b"ic14", 512),
            (b"ic09", 512),
            (b"ic10", 1024),
        )
        body = bytearray()
        for signature, size in chunks:
            data = cache[size]
            body += signature + struct.pack(">I", len(data) + 8) + data
        with output.open("wb") as stream:
            stream.write(b"icns")
            stream.write(struct.pack(">I", len(body) + 8))
            stream.write(body)

    def _linux(
        self,
        primary_svg: Path,
        alternate_svg: Path,
        folder: Path,
    ) -> None:
        self._linux_hicolor(primary_svg, folder / "hicolor")
        self._linux_hicolor(
            alternate_svg,
            folder / "alternate-macos-style" / "hicolor",
        )
        script = folder / "install-user.sh"
        script.write_text(
            """#!/usr/bin/env sh
set -eu
SRC="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)/hicolor"
TARGET="${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor"
mkdir -p "$TARGET"
cp -R "$SRC/"* "$TARGET/"
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "$TARGET" || true
fi
echo "Installed icon into $TARGET"
""",
            encoding="utf-8",
        )
        with suppress(OSError):
            script.chmod(0o755)

    def _linux_hicolor(self, svg: Path, root: Path) -> None:
        for size in LINUX_SIZES:
            target = root / f"{size}x{size}" / "apps"
            target.mkdir(parents=True, exist_ok=True)
            (target / f"{self.icon_name}.png").write_bytes(
                self.renderer.render_png(svg, size)
            )
        scalable = root / "scalable" / "apps"
        scalable.mkdir(parents=True, exist_ok=True)
        shutil.copy2(svg, scalable / f"{self.icon_name}.svg")

    def _android(self, svg: Path, folder: Path) -> None:
        res = folder / "app" / "src" / "main" / "res"

        # Keep the authored circular SVG transparent. Adaptive launchers use it
        # as foreground while black is a separate background layer.
        values = res / "values"
        values.mkdir(parents=True, exist_ok=True)
        (values / "colors.xml").write_text(
            """<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="ic_launcher_background">#000000</color>
</resources>
""",
            encoding="utf-8",
        )

        with self._svg_inside_safe_zone(svg, 66 / 108) as foreground_svg:
            for density, scale in ANDROID_DENSITY_SCALE.items():
                size = round(108 * scale)
                target = res / f"drawable-{density}"
                target.mkdir(parents=True, exist_ok=True)
                (target / "ic_launcher_foreground.png").write_bytes(
                    self.renderer.render_png(foreground_svg, size)
                )

        anydpi = res / "mipmap-anydpi-v26"
        anydpi.mkdir(parents=True, exist_ok=True)
        adaptive = """<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/ic_launcher_background" />
    <foreground android:drawable="@drawable/ic_launcher_foreground" />
</adaptive-icon>
"""
        (anydpi / "ic_launcher.xml").write_text(adaptive, encoding="utf-8")
        (anydpi / "ic_launcher_round.xml").write_text(
            adaptive,
            encoding="utf-8",
        )

        # Legacy launchers and store art cannot rely on the adaptive background,
        # so compose the same transparent source over black while rasterizing.
        with self._svg_over_background(svg, "#000000") as opaque_svg:
            for density, size in ANDROID_LEGACY.items():
                target = res / f"mipmap-{density}"
                target.mkdir(parents=True, exist_ok=True)
                (target / "ic_launcher.png").write_bytes(
                    self.renderer.render_png(opaque_svg, size)
                )
            (folder / "play_store_512.png").write_bytes(
                self.renderer.render_png(opaque_svg, 512)
            )

        (folder / "play_store_512_transparent_reference.png").write_bytes(
            self.renderer.render_png(svg, 512)
        )
        (folder / "AndroidManifest-icon-snippet.txt").write_text(
            """android:icon=\"@mipmap/ic_launcher\"
android:roundIcon=\"@mipmap/ic_launcher_round\"
""",
            encoding="utf-8",
        )

    def _ios(self, svg: Path, folder: Path) -> None:
        catalog = folder / "Assets.xcassets"
        appicon = catalog / "AppIcon.appiconset"
        appicon.mkdir(parents=True, exist_ok=True)

        filename = "AppIcon-1024.png"
        rendered = self.renderer.render_png(svg, IOS_APPICON_SIZE)
        (appicon / filename).write_bytes(rendered)
        (folder / "icon_1024.png").write_bytes(rendered)

        catalog_info = {
            "info": {"author": "xcode", "version": 1},
        }
        (catalog / "Contents.json").write_text(
            json.dumps(catalog_info, indent=2) + "\n",
            encoding="utf-8",
        )
        appicon_info = {
            "images": [
                {
                    "filename": filename,
                    "idiom": "universal",
                    "platform": "ios",
                    "size": "1024x1024",
                }
            ],
            "info": {"author": "xcode", "version": 1},
        }
        (appicon / "Contents.json").write_text(
            json.dumps(appicon_info, indent=2) + "\n",
            encoding="utf-8",
        )

    @staticmethod
    @contextmanager
    def _svg_inside_safe_zone(
        svg: Path,
        ratio: float,
    ) -> Iterator[Path]:
        """Zoom out an SVG page so authored art fits a centered safe zone.

        Yields:
            Path to a temporary SVG carrying the expanded view box.

        Raises:
            ValueError: If ``ratio`` is outside the open-closed unit interval.
            RuntimeError: If the source SVG has no parseable view box.

        """
        if not 0 < ratio <= 1:
            message = "safe-zone ratio must be in (0, 1]"
            raise ValueError(message)
        source = svg.read_text(encoding="utf-8")
        pattern = re.compile(
            r'viewBox=["\']\s*'
            r'(-?[0-9.]+)[ ,]+'
            r'(-?[0-9.]+)[ ,]+'
            r'([0-9.]+)[ ,]+'
            r'([0-9.]+)\s*["\']'
        )
        match = pattern.search(source)
        if match is None:
            message = f"SVG viewBox is missing: {svg}"
            raise RuntimeError(message)
        min_x, min_y, width, height = map(float, match.groups())
        expanded_width = width / ratio
        expanded_height = height / ratio
        expanded_x = min_x - (expanded_width - width) / 2
        expanded_y = min_y - (expanded_height - height) / 2
        replacement = (
            'viewBox="'
            f"{expanded_x:.9g} {expanded_y:.9g} "
            f'{expanded_width:.9g} {expanded_height:.9g}"'
        )
        composed = source[: match.start()] + replacement + source[match.end() :]
        with tempfile.NamedTemporaryFile(
            mode="w",
            suffix=".svg",
            encoding="utf-8",
            delete=False,
        ) as handle:
            handle.write(composed)
            temporary = Path(handle.name)
        try:
            yield temporary
        finally:
            temporary.unlink(missing_ok=True)

    @staticmethod
    @contextmanager
    def _svg_over_background(
        svg: Path,
        color: str,
    ) -> Iterator[Path]:
        """Temporarily place an opaque rectangle below an authored SVG.

        Yields:
            Path to a temporary SVG containing the background rectangle.

        Raises:
            RuntimeError: If the source SVG root element cannot be located.

        """
        source = svg.read_text(encoding="utf-8")
        start = source.find("<svg")
        end = source.find(">", start)
        if start < 0 or end < 0:
            message = f"SVG root element is missing: {svg}"
            raise RuntimeError(message)
        rectangle = (
            f'\n<rect x="0" y="0" width="100%" height="100%" '
            f'fill="{color}"/>\n'
        )
        composed = source[: end + 1] + rectangle + source[end + 1 :]
        with tempfile.NamedTemporaryFile(
            mode="w",
            suffix=".svg",
            encoding="utf-8",
            delete=False,
        ) as handle:
            handle.write(composed)
            temporary = Path(handle.name)
        try:
            yield temporary
        finally:
            temporary.unlink(missing_ok=True)
