"""Generate compact SVG QR codes for the workshop site.

Usage: python scripts/gen_qr.py
Writes site/qr/<name>.svg. Requires the `qrcode` package.
"""

from pathlib import Path

import qrcode
from qrcode.constants import ERROR_CORRECT_M

BASE_URL = "https://tomoki977.github.io/stellarworkshop/"

DARK = "#111014"
LIGHT = "#FBFBF8"
BORDER = 2  # quiet zone, in modules

# Comparison slides get a QR that deep-links to their own section.
SLIDE_IDS = ["gentle-ai", "cli", "skills", "mcp", "demo"]

TARGETS = {
    "site": BASE_URL,
    "linktree": "https://linktr.ee/tomoki977",
    "repo": "https://github.com/TOMOKI977/stellarworkshop",
    **{slide_id: f"{BASE_URL}#{slide_id}" for slide_id in SLIDE_IDS},
}

OUT_DIR = Path(__file__).resolve().parent.parent / "site" / "qr"


def qr_matrix(data: str) -> list[list[bool]]:
    qr = qrcode.QRCode(error_correction=ERROR_CORRECT_M, border=BORDER)
    qr.add_data(data)
    qr.make(fit=True)
    return qr.get_matrix()  # includes the border


def to_svg(matrix: list[list[bool]]) -> str:
    size = len(matrix)
    runs = []
    for y, row in enumerate(matrix):
        x = 0
        while x < size:
            if row[x]:
                start = x
                while x < size and row[x]:
                    x += 1
                # One horizontal run of dark modules becomes a single rectangle.
                runs.append(f"M{start} {y}h{x - start}v1h{start - x}z")
            else:
                x += 1
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" '
        f'shape-rendering="crispEdges">'
        f'<rect width="{size}" height="{size}" fill="{LIGHT}"/>'
        f'<path fill="{DARK}" d="{"".join(runs)}"/>'
        f"</svg>\n"
    )


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    for name, url in TARGETS.items():
        path = OUT_DIR / f"{name}.svg"
        path.write_text(to_svg(qr_matrix(url)), encoding="utf-8", newline="\n")
        print(f"{path.relative_to(OUT_DIR.parent.parent).as_posix()} -> {url}")


if __name__ == "__main__":
    main()
