#!/usr/bin/env python3
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

from unreleased_packages import unreleased_packages

UNRELEASED = re.compile(r"^## Unreleased$", re.MULTILINE)

def main() -> None:
    selected = sys.argv[1:]
    release_date = datetime.now(timezone.utc).strftime("%Y-%m-%d")

    for package in unreleased_packages(selected):
        changelog = Path(package["manifest_path"]).with_name("CHANGELOG.md")
        header = f"## {package['version']} - {release_date}"
        content = UNRELEASED.sub(
            lambda match: f"{match[0]}\n\n{header}", changelog.read_text(), count=1
        )
        changelog.write_text(content)

if __name__ == "__main__":
    main()
