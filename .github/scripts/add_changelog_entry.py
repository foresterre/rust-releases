#!/usr/bin/env python3
import re
import sys
from pathlib import Path

UNRELEASED = re.compile(r"^## Unreleased$", re.MULTILINE)

def main() -> None:
    changelog = Path(sys.argv[1])
    section, entry = sys.argv[2], sys.argv[3]

    content = UNRELEASED.sub(
        lambda match: f"{match[0]}\n\n### {section}\n\n- {entry}",
        changelog.read_text(),
        count=1,
    )
    changelog.write_text(content)

if __name__ == "__main__":
    main()
