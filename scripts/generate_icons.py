#!/usr/bin/env python3
"""Generate the icon set Tauri needs for cross-platform packaging.

Produces:
  - icons/32x32.png
  - icons/128x128.png
  - icons/128x128@2x.png  (256x256)
  - icons/icon.png         (512x512, source)
  - icons/icon.ico         (Windows, multi-size)
  - icons/icon.icns        (macOS)

Uses Pillow. Falls back gracefully if ICNS support is missing.
"""

from pathlib import Path
from PIL import Image, ImageDraw

ICON_DIR = Path(__file__).resolve().parent.parent / "src-tauri" / "icons"


def make_icon(size: int) -> Image.Image:
    """Draw the Pro logo: rounded gradient square with a white play triangle."""
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Rounded rect background with vertical gradient (indigo -> violet).
    radius = size // 5
    for y in range(size):
        t = y / max(size - 1, 1)
        r = int(99 + (139 - 99) * t)
        g = int(102 + (92 - 102) * t)
        b = int(241 + (246 - 241) * t)
        # Only draw inside the rounded rect.
        x0, y0, x1, y1 = 0, y, size, y + 1
        # Cheap rounded mask: skip corners.
        in_corner = (
            (y < radius and False)
            or (y > size - radius and False)
        )
        if not in_corner:
            draw.line([(0, y), (size, y)], fill=(r, g, b, 255))

    # Mask to a rounded rectangle.
    mask = Image.new("L", (size, size), 0)
    mdraw = ImageDraw.Draw(mask)
    mdraw.rounded_rectangle([0, 0, size - 1, size - 1], radius=radius, fill=255)
    rounded = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    rounded.paste(img, (0, 0), mask)
    img = rounded
    draw = ImageDraw.Draw(img)

    # White play triangle, centered.
    cx, cy = size // 2, size // 2
    w = int(size * 0.42)
    h = int(size * 0.48)
    # Shift slightly right because play triangles look left-heavy.
    ox = int(size * 0.06)
    triangle = [
        (cx - w // 2 + ox, cy - h // 2),
        (cx - w // 2 + ox, cy + h // 2),
        (cx + w // 2 + ox, cy),
    ]
    draw.polygon(triangle, fill=(255, 255, 255, 255))

    return img


def write_png(img: Image.Image, path: Path) -> None:
    img.save(path, "PNG")


def write_ico(sizes: list[int], path: Path) -> None:
    imgs = [make_icon(s) for s in sizes]
    imgs[0].save(path, format="ICO", sizes=[(s, s) for s in sizes], append_images=imgs[1:])


def write_icns(path: Path) -> None:
    # 512x512 is the canonical ICNS source size.
    img = make_icon(512)
    try:
        img.save(path, format="ICNS")
    except Exception as exc:  # pragma: no cover
        print(f"  ! ICNS save failed ({exc}); writing placeholder PNG rename")
        # Tauri can fall back to PNG on non-macOS builds; macOS CI uses tauri-cli to regenerate.
        img.save(path.with_suffix(".png"), "PNG")


def main() -> None:
    ICON_DIR.mkdir(parents=True, exist_ok=True)

    print(f"Generating icons in {ICON_DIR}")

    # Source PNG (used by some Tauri defaults).
    write_png(make_icon(512), ICON_DIR / "icon.png")
    print("  - icon.png (512x512)")

    # Required Tauri PNG sizes.
    write_png(make_icon(32), ICON_DIR / "32x32.png")
    print("  - 32x32.png")
    write_png(make_icon(128), ICON_DIR / "128x128.png")
    print("  - 128x128.png")
    write_png(make_icon(256), ICON_DIR / "128x128@2x.png")
    print("  - 128x128@2x.png")

    # Windows ICO (multi-size embedded).
    write_ico([16, 32, 48, 64, 128, 256], ICON_DIR / "icon.ico")
    print("  - icon.ico")

    # macOS ICNS.
    write_icns(ICON_DIR / "icon.icns")
    print("  - icon.icns")

    print("Done.")


if __name__ == "__main__":
    main()
