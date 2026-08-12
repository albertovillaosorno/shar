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
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Concrete platform packaging from reconstructed SVG masters."""

from __future__ import annotations

from pathlib import Path
import shutil
import struct

from icon_contract import (
    ANDROID_DENSITY_SCALE,
    ANDROID_LEGACY,
    LINUX_SIZES,
    MAC_ICONSET,
    WINDOWS_SIZES,
)
from icon_ports import SvgRenderer


class PlatformExporter:
    """Package the authored platform SVGs into target-system icon assets."""

    def __init__(
        self,
        renderer: SvgRenderer,
        icon_name: str = "simpsons-hit-run",
    ):
        self.renderer = renderer
        self.icon_name = icon_name

    def export_all(
        self,
        source_root: Path,
        output_root: Path,
    ) -> tuple[str, ...]:
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

        self._windows(windows_linux, output_root / "windows")
        self._macos(macos_linux, output_root / "macos")
        self._linux(
            windows_linux,
            macos_linux,
            output_root / "linux",
        )
        self._android(android, output_root / "android")

        notes = (
            "Windows: windows-linux.svg",
            "macOS: macos-linux.svg",
            "Linux primary: windows-linux.svg",
            "Linux alternate: macos-linux.svg",
            "Android: android.svg (full-bleed adaptive background)",
        )
        (output_root / "SOURCES.txt").write_text(
            "\n".join(notes) + "\n", encoding="utf-8"
        )
        return notes

    @staticmethod
    def _required(path: Path, label: str) -> Path:
        if not path.is_file():
            raise RuntimeError(f"required icon asset is missing: {label}")
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
                "<BBBBHHII",
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
            stream.write(struct.pack("<HHH", 0, 1, count))
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
        try:
            script.chmod(0o755)
        except OSError:
            pass

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

        # Legacy launchers get a normal raster of the complete Android artwork.
        for density, size in ANDROID_LEGACY.items():
            target = res / f"mipmap-{density}"
            target.mkdir(parents=True, exist_ok=True)
            (target / "ic_launcher.png").write_bytes(
                self.renderer.render_png(svg, size)
            )

        # android.svg is authored as a full-bleed square. Android treats it as
        # the adaptive background and applies the device launcher mask itself
        # (circle, squircle, rounded square, ...). Important content therefore
        # belongs near the center; decorative background may bleed to the edge.
        for density, scale in ANDROID_DENSITY_SCALE.items():
            size = round(108 * scale)
            target = res / f"drawable-{density}"
            target.mkdir(parents=True, exist_ok=True)
            (target / "ic_launcher_background.png").write_bytes(
                self.renderer.render_png(svg, size)
            )

        anydpi = res / "mipmap-anydpi-v26"
        anydpi.mkdir(parents=True, exist_ok=True)
        adaptive = """<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@drawable/ic_launcher_background" />
    <foreground android:drawable="@android:color/transparent" />
</adaptive-icon>
"""
        (anydpi / "ic_launcher.xml").write_text(adaptive, encoding="utf-8")
        (anydpi / "ic_launcher_round.xml").write_text(
            adaptive,
            encoding="utf-8",
        )

        (folder / "play_store_512.png").write_bytes(
            self.renderer.render_png(svg, 512)
        )
        (folder / "AndroidManifest-icon-snippet.txt").write_text(
            """android:icon=\"@mipmap/ic_launcher\"
android:roundIcon=\"@mipmap/ic_launcher_round\"
""",
            encoding="utf-8",
        )
