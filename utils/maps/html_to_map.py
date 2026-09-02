import re
import sys
from pathlib import Path

from bs4 import BeautifulSoup

html = Path(sys.argv[1]).read_text()
soup = BeautifulSoup(html, "html.parser")

css = soup.find("style").get_text()

styles = {
    name: body.lower()
    for name, body in re.findall(
        r"\.(s\d+)\s*\{([^}]*)\}",
        css,
        re.DOTALL,
    )
}

rows = soup.select("tbody tr")

height = len(rows)
width = len(rows[0].find_all("td", recursive=False))

cover = []

for y, row in enumerate(rows):
    cells = row.find_all("td", recursive=False)

    for x, cell in enumerate(cells):
        style = " ".join(styles.get(cls, "") for cls in cell.get("class", []))

        for border, direction in [
            ("border-right", "East"),
            ("border-bottom", "South"),
        ]:
            match = re.search(
                rf"{border}\s*:[^;]*#([0-9a-f]{{6}})",
                style,
            )

            if not match:
                continue

            color = match.group(1)

            cover_type = "Half" if color == "ff0000" else "Full"

            cover.append((x, y, direction, cover_type))

print("(")
print(f"    width: {width},")
print(f"    height: {height},")
print("    blocked: [],")
print("    cover: [")

for x, y, direction, kind in cover:
    print(f"        ({x}, {y}, {direction}, {kind}),")

print("    ],")
print(")")
