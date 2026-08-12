#!/usr/bin/env python3
"""Generate the app icon PNG for the native Rust binary."""
from pathlib import Path
from PIL import Image, ImageDraw

OUT = Path(__file__).resolve().parent.parent / "assets" / "icon.png"

def make_icon(size: int = 512) -> Image.Image:
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    radius = size // 5
    # Gradient background
    for y in range(size):
        t = y / max(size - 1, 1)
        r = int(99 + (139 - 99) * t)
        g = int(102 + (92 - 102) * t)
        b = int(241 + (246 - 241) * t)
        draw.line([(0, y), (size, y)], fill=(r, g, b, 255))
    # Mask rounded
    mask = Image.new("L", (size, size), 0)
    ImageDraw.Draw(mask).rounded_rectangle([0, 0, size - 1, size - 1], radius=radius, fill=255)
    rounded = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    rounded.paste(img, (0, 0), mask)
    img = rounded
    draw = ImageDraw.Draw(img)
    # Play triangle
    cx, cy = size // 2, size // 2
    w = int(size * 0.42)
    h = int(size * 0.48)
    ox = int(size * 0.06)
    triangle = [
        (cx - w // 2 + ox, cy - h // 2),
        (cx - w // 2 + ox, cy + h // 2),
        (cx + w // 2 + ox, cy),
    ]
    draw.polygon(triangle, fill=(255, 255, 255, 255))
    return img

def main() -> None:
    OUT.parent.mkdir(parents=True, exist_ok=True)
    make_icon(512).save(OUT, "PNG")
    print(f"Wrote {OUT}")

if __name__ == "__main__":
    main()
